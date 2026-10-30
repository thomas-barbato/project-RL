//! Shared bounded image preparation. Workers never access the GL context.
use macroquad::prelude::*;
use project_rl::asset_cache::ByteCache;
use std::{
    cell::RefCell,
    collections::BTreeSet,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};

pub struct PreparedImage {
    pub image: Image,
    pub cell: [u16; 2],
    pub anchor: [f32; 2],
}
pub struct PreparedTexture {
    pub texture: Texture2D,
    pub cell: [u16; 2],
    pub anchor: [f32; 2],
}
pub type Builder = Box<dyn FnOnce() -> Result<PreparedImage, String> + Send>;
pub type Request = (String, Builder);
type Completed = (String, Result<PreparedImage, String>);
static READS: AtomicU64 = AtomicU64::new(0);
static READ_BYTES: AtomicU64 = AtomicU64::new(0);
static READ_NS: AtomicU64 = AtomicU64::new(0);
static DECODE_NS: AtomicU64 = AtomicU64::new(0);
static BUILD_NS: AtomicU64 = AtomicU64::new(0);
static UPLOAD_NS: AtomicU64 = AtomicU64::new(0);
static SOURCE_PEAK: AtomicU64 = AtomicU64::new(0);
struct Queue {
    send: mpsc::SyncSender<Request>,
    receive: mpsc::Receiver<Completed>,
    pending: BTreeSet<String>,
    cache: ByteCache<String, Result<PreparedTexture, String>>,
    uploads: usize,
    jobs: usize,
}
impl Queue {
    fn new() -> Self {
        let (send, work) = mpsc::sync_channel::<Request>(16);
        let (done, receive) = mpsc::sync_channel(2);
        std::thread::Builder::new()
            .name("resource-preparation".into())
            .spawn(move || {
                while let Ok((key, build)) = work.recv() {
                    let start = Instant::now();
                    let result = build();
                    BUILD_NS.fetch_add(start.elapsed().as_nanos() as u64, Ordering::Relaxed);
                    if done.send((key, result)).is_err() {
                        break;
                    }
                }
            })
            .expect("resource preparation worker");
        Self {
            send,
            receive,
            pending: BTreeSet::new(),
            cache: ByteCache::new(128 * 1024 * 1024),
            uploads: 0,
            jobs: 0,
        }
    }
}
thread_local! {
    // Logic-only tests construct creation state without a Macroquad context.
    // Mark readiness only on the window's graphics thread, after its creation.
    static GRAPHICS_READY:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};
    static QUEUE:RefCell<Option<Queue>>=const{RefCell::new(None)};
    // This is used only on CPU preparation workers. Recent decoded sources are
    // reusable by any map, animation, interface or effect preparation job.
    static IMAGES:RefCell<ByteCache<String,Arc<Image>>>=const{RefCell::new(ByteCache::new(64*1024*1024))};
}
pub fn initialize_graphics_thread() {
    GRAPHICS_READY.set(true);
}
pub fn graphics_thread_ready() -> bool {
    GRAPHICS_READY.get()
}
fn queue<T>(f: impl FnOnce(&mut Queue) -> T) -> T {
    QUEUE.with_borrow_mut(|q| f(q.get_or_insert_with(Queue::new)))
}
pub fn image(path: &str) -> Result<Arc<Image>, String> {
    let key = path.to_owned();
    IMAGES.with_borrow_mut(|cache| {
        if let Some(value) = cache.get(&key) {
            return Ok(Arc::clone(value));
        }
        let start = Instant::now();
        let bytes = project_rl::data_paths::read(path)?;
        READS.fetch_add(1, Ordering::Relaxed);
        READ_BYTES.fetch_add(bytes.len() as u64, Ordering::Relaxed);
        READ_NS.fetch_add(start.elapsed().as_nanos() as u64, Ordering::Relaxed);
        let start = Instant::now();
        let image = Arc::new(
            super::asset_loading::image(&bytes, Some(ImageFormat::Png))
                .map_err(|e| e.to_string())?,
        );
        DECODE_NS.fetch_add(start.elapsed().as_nanos() as u64, Ordering::Relaxed);
        cache.insert(key, Arc::clone(&image), image.bytes.len());
        SOURCE_PEAK.fetch_max(cache.bytes_used() as u64, Ordering::Relaxed);
        Ok(image)
    })
}
pub fn request(key: String, build: Builder) -> bool {
    queue(|q| {
        if q.cache.get(&key).is_some() || q.pending.contains(&key) {
            return true;
        }
        match q.send.try_send((key.clone(), build)) {
            Ok(()) => {
                q.pending.insert(key);
                q.jobs += 1;
                true
            }
            Err(mpsc::TrySendError::Full(_)) => false,
            Err(mpsc::TrySendError::Disconnected(_)) => {
                q.cache
                    .insert(key, Err("Preparation worker stopped".into()), 0);
                true
            }
        }
    })
}
/// Upload only a small ready batch per preparation tick, on the context thread.
pub fn pump() {
    queue(|q| {
        for _ in 0..2 {
            let Ok((key, result)) = q.receive.try_recv() else {
                break;
            };
            q.pending.remove(&key);
            let bytes = result.as_ref().map_or(0, |p| p.image.bytes.len());
            let ready = result.map(|p| {
                let start = Instant::now();
                let texture = super::asset_loading::from_image(&p.image);
                UPLOAD_NS.fetch_add(start.elapsed().as_nanos() as u64, Ordering::Relaxed);
                texture.set_filter(FilterMode::Nearest);
                q.uploads += 1;
                PreparedTexture {
                    texture,
                    cell: p.cell,
                    anchor: p.anchor,
                }
            });
            q.cache.insert(key, ready, bytes);
        }
    });
}
pub fn with_texture<T>(key: &str, f: impl FnOnce(&PreparedTexture) -> T) -> Option<T> {
    queue(|q| {
        q.cache
            .get(&key.to_owned())
            .and_then(|r| r.as_ref().ok())
            .map(f)
    })
}
pub fn ready(key: &str) -> Result<bool, String> {
    queue(|q| match q.cache.get(&key.to_owned()) {
        Some(Ok(_)) => Ok(true),
        Some(Err(e)) => Err(e.clone()),
        None => Ok(false),
    })
}
/// This gate is called before first display, outside every drawing function.
/// Lookahead normally completes these jobs on earlier preparation ticks.
pub fn prepare(requests: impl Fn() -> Vec<Request>) -> Result<Duration, String> {
    let start = Instant::now();
    let mut waited = false;
    loop {
        let mut complete = true;
        for (key, build) in requests() {
            if !ready(&key)? {
                complete = false;
                request(key, build);
            }
        }
        pump();
        if complete {
            return Ok(if waited {
                start.elapsed()
            } else {
                Duration::ZERO
            });
        }
        waited = true;
        if start.elapsed() > Duration::from_secs(30) {
            return Err("Resource preparation did not converge within its bounded cache".into());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
#[cfg(debug_assertions)]
pub fn stats() -> serde_json::Value {
    queue(|q| {
        serde_json::json!({"jobs":q.jobs,"uploads":q.uploads,"pending":q.pending.len(),"texture_bytes":q.cache.bytes_used(),"texture_budget":128*1024*1024,"decoded_source_budget":64*1024*1024,
            "source_reads":READS.load(Ordering::Relaxed),"read_bytes":READ_BYTES.load(Ordering::Relaxed),"read_ms":READ_NS.load(Ordering::Relaxed) as f64/1e6,"decode_ms":DECODE_NS.load(Ordering::Relaxed) as f64/1e6,"worker_build_ms_including_sources":BUILD_NS.load(Ordering::Relaxed) as f64/1e6,"texture_create_ms":UPLOAD_NS.load(Ordering::Relaxed) as f64/1e6,"decoded_source_peak_bytes":SOURCE_PEAK.load(Ordering::Relaxed)})
    })
}

//! Opt-in loading measurements. These never enter world or save state.
#[cfg(debug_assertions)]
use std::{cell::RefCell, time::Instant};

#[cfg(debug_assertions)]
#[derive(Default)]
struct Profile {
    enabled: bool,
    phase: String,
    samples: Vec<Sample>,
}

#[cfg(debug_assertions)]
#[derive(serde::Serialize)]
pub struct Sample {
    pub phase: String,
    pub stage: &'static str,
    pub asset: String,
    pub bytes: usize,
    pub elapsed_ms: f64,
}

#[cfg(debug_assertions)]
thread_local! {
    static PROFILE: RefCell<Profile> = RefCell::new(Profile::default());
}

#[cfg(debug_assertions)]
pub fn start() {
    PROFILE.with_borrow_mut(|profile| {
        *profile = Profile {
            enabled: true,
            phase: "startup".into(),
            ..Default::default()
        };
    });
}

#[cfg(debug_assertions)]
pub fn phase(name: &str) {
    PROFILE.with_borrow_mut(|profile| profile.phase = name.to_owned());
}

#[cfg(debug_assertions)]
pub fn finish() -> Vec<Sample> {
    PROFILE.with_borrow_mut(|profile| {
        profile.enabled = false;
        std::mem::take(&mut profile.samples)
    })
}

pub struct Span {
    #[cfg(debug_assertions)]
    sample: Option<(Instant, Sample)>,
}

impl Span {
    #[inline]
    pub fn new(stage: &'static str, asset: &str, bytes: usize) -> Self {
        #[cfg(debug_assertions)]
        {
            let sample = PROFILE.with_borrow(|profile| {
                profile.enabled.then(|| {
                    (
                        Instant::now(),
                        Sample {
                            phase: profile.phase.clone(),
                            stage,
                            asset: asset.to_owned(),
                            bytes,
                            elapsed_ms: 0.,
                        },
                    )
                })
            });
            Self { sample }
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = (stage, asset, bytes);
            Self {}
        }
    }

    #[inline]
    #[track_caller]
    pub fn caller(stage: &'static str, bytes: usize) -> Self {
        #[cfg(debug_assertions)]
        {
            if PROFILE.with_borrow(|profile| profile.enabled) {
                let caller = std::panic::Location::caller();
                return Self::new(
                    stage,
                    &format!("{}:{}", caller.file(), caller.line()),
                    bytes,
                );
            }
        }
        let _ = (stage, bytes);
        Self {
            #[cfg(debug_assertions)]
            sample: None,
        }
    }
}

impl Drop for Span {
    #[inline]
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        if let Some((started, mut sample)) = self.sample.take() {
            sample.elapsed_ms = started.elapsed().as_secs_f64() * 1000.;
            PROFILE.with_borrow_mut(|profile| profile.samples.push(sample));
        }
    }
}

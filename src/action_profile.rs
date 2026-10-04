//! Opt-in, thread-local CPU measurements. Never part of engine or save state.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Default)]
struct Profile {
    enabled: bool,
    samples: BTreeMap<&'static str, Vec<f64>>,
    paths: Vec<PathSample>,
}

#[derive(serde::Serialize)]
pub struct PathSample {
    start: crate::world::GridPos,
    goal: crate::world::GridPos,
    visited: usize,
    length: Option<usize>,
    elapsed_ms: f64,
}

thread_local! {
    static PROFILE: RefCell<Profile> = RefCell::new(Profile::default());
}

pub fn start() {
    PROFILE.with(|profile| {
        *profile.borrow_mut() = Profile {
            enabled: true,
            ..Default::default()
        };
    });
}

pub fn finish() -> BTreeMap<&'static str, Vec<f64>> {
    PROFILE.with(|profile| {
        let mut profile = profile.borrow_mut();
        profile.enabled = false;
        std::mem::take(&mut profile.samples)
    })
}

pub fn take_paths() -> Vec<PathSample> {
    PROFILE.with(|profile| std::mem::take(&mut profile.borrow_mut().paths))
}

pub struct Scope {
    name: &'static str,
    start: Option<Instant>,
}

impl Scope {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            start: PROFILE.with(|profile| profile.borrow().enabled.then(Instant::now)),
        }
    }

    pub fn path_result(
        &self,
        start: crate::world::GridPos,
        goal: crate::world::GridPos,
        visited: usize,
        length: Option<usize>,
    ) {
        if let Some(time) = self.start {
            let elapsed_ms = time.elapsed().as_secs_f64() * 1000.;
            PROFILE.with(|profile| {
                profile.borrow_mut().paths.push(PathSample {
                    start,
                    goal,
                    visited,
                    length,
                    elapsed_ms,
                })
            });
        }
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        if let Some(start) = self.start {
            let elapsed = start.elapsed().as_secs_f64() * 1000.;
            PROFILE.with(|profile| {
                profile
                    .borrow_mut()
                    .samples
                    .entry(self.name)
                    .or_default()
                    .push(elapsed)
            });
        }
    }
}

use log::{debug, info, warn};
use std::time::Instant;

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn generate(&self) {
        debug!("beginning code gen.");
        let now = Instant::now();
        for i in 0..100000000 {}
        let elapsed = now.elapsed();
        debug!(
            "time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
    }
}

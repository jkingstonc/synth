use std::time::Instant;

use log::debug;

use crate::ir::IR;

pub struct IRParser {}

impl IRParser {
    pub fn parse(&mut self) -> Box<IR> {
        let now = Instant::now();
        let elapsed = now.elapsed();
        debug!(
            "ir parsing time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        return Box::new(IR::NONE);
    }
}

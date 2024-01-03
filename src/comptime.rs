use std::time::Instant;

use log::debug;

use crate::ir::Instruction;

// perform comptime analysis
pub struct ComptimeAnalyzer {
    pub ir: Box<Vec<Instruction>>,
}

impl ComptimeAnalyzer {
    pub fn analyze(&mut self) -> Box<Vec<Instruction>> {
        let now = Instant::now();
        let elapsed = now.elapsed();
        debug!(
            "comptime analysis time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        Box::new(vec![])
    }
}

use std::time::Instant;

use log::debug;

use crate::ir::Instruction;

pub trait IROptimizer {
    fn optimize(&mut self) -> Box<Vec<Instruction>>;
}

pub struct GeneralPassIROptimizer {
    pub ir: Box<Vec<Instruction>>,
}

impl IROptimizer for GeneralPassIROptimizer {
    fn optimize(&mut self) -> Box<Vec<Instruction>> {
        let now = Instant::now();
        let elapsed = now.elapsed();
        debug!(
            "GeneralPassIROptimizer time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        Box::new(vec![])
    }
}

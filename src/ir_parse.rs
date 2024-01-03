use std::time::Instant;

use log::debug;

use crate::ir::{Instruction, InstructionData, InstructionType};

pub struct IRParser {}

impl IRParser {
    pub fn parse(&mut self) -> Box<Vec<Instruction>> {
        let now = Instant::now();
        let elapsed = now.elapsed();
        debug!(
            "ir parsing time elapsed {:.2?}ms ({:.2?}s).",
            elapsed.as_millis(),
            elapsed.as_secs()
        );
        return Box::new(vec![Instruction {
            instruction_type: InstructionType::NONE,
            data: InstructionData {},
        }]);
    }
}

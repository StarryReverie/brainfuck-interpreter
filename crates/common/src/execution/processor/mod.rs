use snafu::prelude::*;

use crate::compiler::instruction::{Instruction, InstructionList};
use crate::compiler::parser::syntax::AddUntilZeroArg;
use crate::execution::context::Context;
use crate::execution::memory::{Memory, MemoryError};

pub type Result<T> = std::result::Result<T, ProcessorError>;

struct Counter {
    val: usize,
}

impl Counter {
    fn new() -> Self {
        Self { val: 0 }
    }

    fn tick(&mut self) {
        self.val += 1;
    }

    fn jump(&mut self, target: usize) {
        self.val = target
    }

    fn get(&self) -> usize {
        self.val
    }
}

pub struct Processor {
    counter: Counter,
    instructions: InstructionList,
}

impl Processor {
    pub fn new(instructions: InstructionList) -> Self {
        Self {
            counter: Counter::new(),
            instructions,
        }
    }

    fn tick(&mut self) -> bool {
        self.counter.tick();
        self.instructions.0[self.counter.get()] != Instruction::Halt
    }

    fn current(&self) -> &Instruction {
        &self.instructions.0[self.counter.get()]
    }

    fn step(&mut self, context: &mut Context) -> Result<bool> {
        let Context {
            memory,
            in_stream,
            out_stream,
        } = context;

        match self.current() {
            Instruction::Add { val } => {
                memory.add(*val).context(MemorySnafu)?;
                Ok(self.tick())
            }
            Instruction::Seek { offset } => {
                memory.seek(*offset).context(MemorySnafu)?;
                Ok(self.tick())
            }
            Instruction::Clear => {
                memory.set(0).unwrap();
                Ok(self.tick())
            }
            Instruction::AddUntilZero { target } => {
                Self::add_while_zero(target, memory).context(MemorySnafu)?;
                Ok(self.tick())
            }
            Instruction::Input => {
                memory.set(in_stream.read()).unwrap();
                Ok(self.tick())
            }
            Instruction::Output => {
                out_stream.write(memory.get());
                Ok(self.tick())
            }
            Instruction::Jump { target } => {
                self.counter.jump(*target);
                Ok(*self.current() != Instruction::Halt)
            }
            Instruction::JumpIfZero { target } => {
                if memory.get() == 0 {
                    self.counter.jump(*target);
                    Ok(*self.current() != Instruction::Halt)
                } else {
                    Ok(self.tick())
                }
            }
            Instruction::Halt => Ok(false),
        }
    }

    fn add_while_zero(
        target: &[AddUntilZeroArg],
        memory: &mut Memory,
    ) -> std::result::Result<(), MemoryError> {
        let val = memory.get();

        if val == 0 {
            return Ok(());
        }

        memory.set(0).unwrap();

        for AddUntilZeroArg { offset, times } in target {
            memory.seek(*offset)?;
            memory.add(val * *times)?;
            memory.seek(-*offset)?;
        }

        Ok(())
    }

    pub fn run(mut self, context: &mut Context) -> Result<()> {
        if self.instructions.0.len() == 1 {
            return Err(ProcessorError::Empty);
        }

        while self.step(context)? {}

        Ok(())
    }
}

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum ProcessorError {
    #[snafu(display("invalid memory operation occurred"))]
    Memory { source: MemoryError },
    #[snafu(display("empty program loaded"))]
    Empty,
}

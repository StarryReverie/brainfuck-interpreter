pub mod add;
pub mod get;
pub mod help;
pub mod position;
pub mod run;
pub mod set;
pub mod view;

use common::execution::memory::AddrRange;
use snafu::prelude::*;

use crate::interpreter::Interpreter;
use self::{add::AddError, get::GetError, run::RunError, set::SetError, view::ViewError};

pub type Result<T> = std::result::Result<T, CommandError>;

/// REPL commands that interact with the interpreter state.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    /// Read the value at the given address
    Get { addr: isize },
    /// Print the current data pointer position
    Position,
    /// Compile and run Brainfuck source code
    Run { code: String },
    /// Add `val` to the cell at `addr`
    Add { addr: isize, val: i32 },
    /// Set the cell at `addr` to `val`
    Set { addr: isize, val: i32 },
    /// Display a memory view for the given address range
    View { range: AddrRange },
    /// Print available commands
    Help,
    /// Quit the REPL
    Exit,
}

impl Command {
    pub fn execute(self, interpreter: &mut Interpreter) -> Result<()> {
        match self {
            Command::Get { addr } => println!("{}", get::execute(interpreter.memory(), addr)?),
            Command::Position => println!("{}", position::execute(interpreter.memory())),
            Command::Run { code } => {
                run::execute(interpreter, &code)?;
                println!();
            }
            Command::Add { addr, val } => add::execute(interpreter.memory_mut(), addr, val)?,
            Command::Set { addr, val } => set::execute(interpreter.memory_mut(), addr, val)?,
            Command::View { range } => println!("{}", view::execute(interpreter.memory(), range)?),
            Command::Help => help::execute(),
            _ => unreachable!(),
        }

        Ok(())
    }
}

/// Errors that can occur when executing a REPL command.
#[derive(Snafu, Debug)]
pub enum CommandError {
    #[snafu(display("an error occurred when executing command `get`"))]
    Get { source: GetError },
    #[snafu(display("an error occurred when executing command `run`"))]
    Run { source: RunError },
    #[snafu(display("an error occurred when executing command `add`"))]
    Add { source: AddError },
    #[snafu(display("an error occurred when executing command `set`"))]
    Set { source: SetError },
    #[snafu(display("an error occurred when executing command `view`"))]
    View { source: ViewError },
}

impl From<GetError> for CommandError {
    fn from(source: GetError) -> Self {
        Self::Get { source }
    }
}

impl From<RunError> for CommandError {
    fn from(source: RunError) -> Self {
        Self::Run { source }
    }
}

impl From<AddError> for CommandError {
    fn from(source: AddError) -> Self {
        Self::Add { source }
    }
}

impl From<SetError> for CommandError {
    fn from(source: SetError) -> Self {
        Self::Set { source }
    }
}

impl From<ViewError> for CommandError {
    fn from(source: ViewError) -> Self {
        Self::View { source }
    }
}

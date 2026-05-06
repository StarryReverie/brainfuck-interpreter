use crate::execution::stream::EOF;

/// Strategy for handling EOF values when writing to a memory cell.
pub trait EofStrategy {
    /// Returns `Some(value)` to write, or `None` to skip the write.
    fn check(&self, input: i32) -> Option<i32>;
}

/// EOF is converted to `0` before writing.
pub struct ZeroEofStrategy {}

impl EofStrategy for ZeroEofStrategy {
    fn check(&self, input: i32) -> Option<i32> {
        if input == EOF {
            Some(0)
        } else {
            Some(input)
        }
    }
}

/// EOF value (`-1`) is written to the cell as-is.
pub struct KeepEofStrategy {}

impl EofStrategy for KeepEofStrategy {
    fn check(&self, input: i32) -> Option<i32> {
        Some(input)
    }
}

/// EOF is ignored — the cell is left unchanged.
pub struct IgnoreEofStrategy {}

impl EofStrategy for IgnoreEofStrategy {
    fn check(&self, input: i32) -> Option<i32> {
        if input == EOF {
            None
        } else {
            Some(input)
        }
    }
}

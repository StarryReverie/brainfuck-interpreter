use super::cell::CellStrategy;
use super::{MemoryError, Result};

/// Strategy for handling arithmetic overflow on cell values.
pub trait OverflowStrategy {
    fn add(&self, cell_strategy: &dyn CellStrategy, before: i32, add: i32) -> Result<i32>;

    fn set(&self, cell_strategy: &dyn CellStrategy, val: i32) -> Result<i32>;
}

/// Overflow produces an error.
pub struct ErrorOverflowStrategy {}

impl OverflowStrategy for ErrorOverflowStrategy {
    fn add(&self, cell_strategy: &dyn CellStrategy, before: i32, add: i32) -> Result<i32> {
        let res = before as i64 + add as i64;

        if cell_strategy.is_overflowed(res) {
            Err(MemoryError::AddOverflow { before, add })
        } else {
            Ok(res as i32)
        }
    }

    fn set(&self, cell_strategy: &dyn CellStrategy, val: i32) -> Result<i32> {
        if cell_strategy.is_overflowed(val as i64) {
            Err(MemoryError::SetOverflow { val })
        } else {
            Ok(val)
        }
    }
}

/// Overflow wraps around the cell's representable range.
pub struct WrapOverflowStrategy {}

impl OverflowStrategy for WrapOverflowStrategy {
    fn add(&self, cell_strategy: &dyn CellStrategy, before: i32, add: i32) -> Result<i32> {
        let res = before as i64 + add as i64;

        if cell_strategy.is_overflowed(res) {
            Ok(cell_strategy.wrap(res))
        } else {
            Ok(res as i32)
        }
    }

    fn set(&self, cell_strategy: &dyn CellStrategy, val: i32) -> Result<i32> {
        if cell_strategy.is_overflowed(val as i64) {
            Ok(cell_strategy.wrap(val as i64))
        } else {
            Ok(val)
        }
    }
}

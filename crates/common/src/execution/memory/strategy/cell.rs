/// Strategy for cell value validation (overflow detection) and wrapping.
pub trait CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool;

    fn wrap(&self, num: i64) -> i32;
}

/// 8-bit signed cell strategy: values in `-128..=127`.
pub struct I8CellStrategy {}

impl CellStrategy for I8CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool {
        num < i8::MIN as i64 || num > i8::MAX as i64
    }

    fn wrap(&self, num: i64) -> i32 {
        num as i8 as i32
    }
}

/// 32-bit signed cell strategy: values in `i32::MIN..=i32::MAX`.
pub struct I32CellStrategy {}

impl CellStrategy for I32CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool {
        num < i32::MIN as i64 || num > i32::MAX as i64
    }

    fn wrap(&self, num: i64) -> i32 {
        num as i32
    }
}

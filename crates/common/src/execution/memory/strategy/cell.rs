pub trait CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool;

    fn wrap(&self, num: i64) -> i32;
}

pub struct I8CellStrategy {}

impl CellStrategy for I8CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool {
        num < i8::MIN as i64 || num > i8::MAX as i64
    }

    fn wrap(&self, num: i64) -> i32 {
        num as i8 as i32
    }
}

pub struct I32CellStrategy {}

impl CellStrategy for I32CellStrategy {
    fn is_overflowed(&self, num: i64) -> bool {
        num < i32::MIN as i64 || num > i32::MAX as i64
    }

    fn wrap(&self, num: i64) -> i32 {
        num as i32
    }
}

mod addr;
mod cell;
mod eof;
mod overflow;

use super::{MemoryError, Result};

pub use addr::{AddrStrategy, SignedAddrStrategy, UnsignedAddrStrategy};
pub use cell::{CellStrategy, I32CellStrategy, I8CellStrategy};
pub use eof::{EofStrategy, IgnoreEofStrategy, KeepEofStrategy, ZeroEofStrategy};
pub use overflow::{ErrorOverflowStrategy, OverflowStrategy, WrapOverflowStrategy};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AddrRange {
    pub left: isize,
    pub right: isize,
}

impl AddrRange {
    pub fn len(&self) -> usize {
        (self.right - self.left + 1) as usize
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn contains(&self, addr: isize) -> bool {
        self.left <= addr && addr <= self.right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_addr_strategy() {
        let r = UnsignedAddrStrategy::new(5);
        assert_eq!(r.seek(0, 2), Ok(2));
        assert_eq!(
            r.seek(0, 5),
            Err(MemoryError::SeekOutOfBounds {
                now_position: 0,
                offset: 5,
                range: AddrRange { left: 0, right: 4 }
            })
        );
        assert_eq!(r.calc(4), 4);
    }

    #[test]
    fn signed_address_strategy() {
        let r = SignedAddrStrategy::new(5);
        assert_eq!(r.seek(0, -5), Ok(-5));
        assert_eq!(
            r.seek(0, -6),
            Err(MemoryError::SeekOutOfBounds {
                now_position: 0,
                offset: -6,
                range: AddrRange { left: -5, right: 4 }
            })
        );
        assert_eq!(r.calc(4), 9);
    }

    #[test]
    fn i8_cell_strategy() {
        let c = I8CellStrategy {};
        assert!(!c.is_overflowed(127));
        assert!(c.is_overflowed(128));
        assert!(!c.is_overflowed(-128));
        assert!(c.is_overflowed(-129));

        assert_eq!(c.wrap(127), 127);
        assert_eq!(c.wrap(128), -128);
        assert_eq!(c.wrap(-129), 127);
        assert_eq!(c.wrap(1121), 97);
        assert_eq!(c.wrap(-1211), 69);
        assert_eq!(c.wrap(-1111), -87);
    }

    #[test]
    fn i32_cell_strategy() {
        let c = I32CellStrategy {};
        assert!(c.is_overflowed(2147483648i64));
        assert!(!c.is_overflowed(-2147483648i64));
        assert!(c.is_overflowed(-2147483649i64));

        assert_eq!(c.wrap(-2147483649i64), 2147483647);
        assert_eq!(c.wrap(-2147483648i64 - 2147483647i64 - 1i64), 0);
    }

    #[test]
    fn error_overflow_strategy() {
        let o = ErrorOverflowStrategy {};
        let c = I8CellStrategy {};
        assert_eq!(o.add(&c, 0, 1), Ok(1));
        assert_eq!(
            o.add(&c, 127, 1),
            Err(MemoryError::AddOverflow {
                before: 127,
                add: 1
            })
        );
    }

    #[test]
    fn wrap_overflow_strategy() {
        let o = WrapOverflowStrategy {};
        let c = I8CellStrategy {};
        assert_eq!(o.add(&c, 0, 1), Ok(1));
        assert_eq!(o.add(&c, 127, 1), Ok(-128));
    }
}

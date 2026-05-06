use super::{AddrRange, MemoryError, Result};

/// Strategy for validating and translating logical memory addresses
/// into physical `Vec` indices.
pub trait AddrStrategy {
    fn initial(&self) -> isize {
        0
    }

    fn seek(&self, addr: isize, offset: isize) -> Result<isize>;

    fn calc(&self, addr: isize) -> usize;

    fn range(&self) -> AddrRange;
}

/// Addresses are unsigned: valid range is `[0, len - 1]`.
pub struct UnsignedAddrStrategy {
    len: usize,
}

impl UnsignedAddrStrategy {
    pub fn new(len: usize) -> Self {
        Self { len }
    }
}

impl AddrStrategy for UnsignedAddrStrategy {
    fn seek(&self, addr: isize, offset: isize) -> Result<isize> {
        let target = addr + offset;

        if 0 <= target && target < self.len as isize {
            Ok(target)
        } else {
            Err(MemoryError::SeekOutOfBounds {
                now_position: addr,
                offset,
                range: self.range(),
            })
        }
    }

    fn calc(&self, addr: isize) -> usize {
        addr as usize
    }

    fn range(&self) -> AddrRange {
        AddrRange {
            left: 0,
            right: self.len as isize - 1,
        }
    }
}

/// Addresses are signed: valid range is `[-half_len, half_len - 1]`.
pub struct SignedAddrStrategy {
    half_len: usize,
}

impl SignedAddrStrategy {
    pub fn new(half_len: usize) -> Self {
        Self { half_len }
    }
}

impl AddrStrategy for SignedAddrStrategy {
    fn seek(&self, addr: isize, offset: isize) -> Result<isize> {
        let target = addr + offset;

        if -(self.half_len as isize) <= target && target < self.half_len as isize {
            Ok(target)
        } else {
            Err(MemoryError::SeekOutOfBounds {
                now_position: addr,
                offset,
                range: self.range(),
            })
        }
    }

    fn calc(&self, addr: isize) -> usize {
        addr as usize + self.half_len
    }

    fn range(&self) -> AddrRange {
        AddrRange {
            left: -(self.half_len as isize),
            right: self.half_len as isize - 1,
        }
    }
}

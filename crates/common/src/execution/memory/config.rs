use super::DEFAULT_LEN;

/// Configuration for the memory tape.
#[derive(Clone)]
pub struct Config {
    pub len: usize,
    pub addr: Addr,
    pub cell: Cell,
    pub overflow: Overflow,
    pub eof: Eof,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            len: DEFAULT_LEN,
            addr: Addr::Unsigned,
            cell: Cell::I8,
            overflow: Overflow::Error,
            eof: Eof::Ignore,
        }
    }
}

/// Addressing mode for the memory tape.
#[derive(Clone)]
pub enum Addr {
    /// Addresses are in `[0, len - 1]`
    Unsigned,
    /// Addresses are in `[-ceil(len/2), ceil(len/2) - 1]`
    Signed,
}

/// Cell data type for the memory tape.
#[derive(Clone)]
pub enum Cell {
    /// 8-bit signed integer (`-128..=127`)
    I8,
    /// 32-bit signed integer
    I32,
}

/// Behavior when a cell value exceeds its representable range.
#[derive(Clone)]
pub enum Overflow {
    /// Return an error and abort
    Error,
    /// Wrap around (e.g. `127 + 1 → -128` for i8)
    Wrap,
}

/// Behavior when EOF is encountered during input.
#[derive(Clone)]
pub enum Eof {
    /// Write `0` to the cell
    Zero,
    /// Write `-1` to the cell
    Keep,
    /// Leave the cell unchanged
    Ignore,
}

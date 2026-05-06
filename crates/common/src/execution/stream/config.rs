use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Configuration for the input and output streams.
#[derive(Clone)]
pub struct Config {
    pub input: Input,
    pub output: Output,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input: Input::Standard,
            output: Output::CharStandard,
        }
    }
}

/// Input stream variant selection.
#[derive(Clone)]
pub enum Input {
    /// Always returns EOF
    Null,
    /// Reads from standard input
    Standard,
    /// Reads from an in-memory queue (for testing)
    Vec(Rc<RefCell<VecDeque<i32>>>),
}

/// Output stream variant selection.
#[derive(Clone)]
pub enum Output {
    /// Discards all output
    Null,
    /// Prints characters to standard output
    CharStandard,
    /// Prints integers to standard output
    IntStandard,
    /// Appends to an in-memory queue (for testing)
    Vec(Rc<RefCell<VecDeque<i32>>>),
}

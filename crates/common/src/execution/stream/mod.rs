pub mod config;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{stdin, BufReader, Read, Stdin};
use std::rc::Rc;

use config::{Config, Input, Output};

/// EOF sentinel value returned by input streams when no more data is available.
pub const EOF: i32 = -1;

/// Trait for input streams. Returns an ASCII byte (0–255) or [`EOF`].
pub trait InStream {
    fn read(&mut self) -> i32;
}

/// An input stream that always returns [`EOF`]. Useful when no input is expected.
pub struct NullInStream;

impl InStream for NullInStream {
    fn read(&mut self) -> i32 {
        EOF
    }
}

/// An input stream that reads from standard input.
pub struct StandardInStream {
    reader: BufReader<Stdin>,
}

impl StandardInStream {
    pub fn new() -> Self {
        Self {
            reader: BufReader::new(stdin()),
        }
    }
}

impl InStream for StandardInStream {
    fn read(&mut self) -> i32 {
        let mut buf = [0u8; 1];
        let res = self.reader.read(&mut buf);

        match res {
            Ok(0) | Err(_) => EOF,
            _ => buf[0] as i32,
        }
    }
}

/// An input stream backed by an in-memory `VecDeque`, useful for testing.
pub struct VecInStream {
    input: Rc<RefCell<VecDeque<i32>>>,
}

impl VecInStream {
    pub fn new(input: Rc<RefCell<VecDeque<i32>>>) -> Self {
        Self { input }
    }
}

impl InStream for VecInStream {
    fn read(&mut self) -> i32 {
        self.input.borrow_mut().pop_front().unwrap_or(EOF)
    }
}

/// Trait for output streams. Receives an ASCII byte (0–255).
pub trait OutStream {
    fn write(&mut self, content: i32);
}

/// An output stream that discards all output.
pub struct NullOutStream;

impl OutStream for NullOutStream {
    fn write(&mut self, _content: i32) {}
}

/// An output stream that prints characters to standard output.
pub struct CharStandardOutStream;

impl OutStream for CharStandardOutStream {
    fn write(&mut self, content: i32) {
        print!("{}", char::from_u32(content as u32).unwrap_or('\u{FFFD}'));
    }
}

/// An output stream that prints integer values to standard output.
pub struct IntStandardOutStream;

impl OutStream for IntStandardOutStream {
    fn write(&mut self, content: i32) {
        print!("{content} ");
    }
}

/// An output stream backed by an in-memory `VecDeque`, useful for testing.
pub struct VecOutStream {
    output: Rc<RefCell<VecDeque<i32>>>,
}

impl VecOutStream {
    pub fn new(output: Rc<RefCell<VecDeque<i32>>>) -> Self {
        Self { output }
    }
}

impl OutStream for VecOutStream {
    fn write(&mut self, content: i32) {
        self.output.borrow_mut().push_back(content);
    }
}

/// Builder for constructing an input/output stream pair from configuration.
pub struct Builder {
    input: Input,
    output: Output,
}

#[allow(dead_code)]
impl Builder {
    pub fn new() -> Self {
        Self {
            input: Input::Standard,
            output: Output::CharStandard,
        }
    }

    pub fn with_config(config: Config) -> Self {
        let Config { input, output } = config;
        Self { input, output }
    }

    pub fn input(mut self, input: Input) -> Self {
        self.input = input;
        self
    }

    pub fn output(mut self, output: Output) -> Self {
        self.output = output;
        self
    }

    pub fn build(self) -> (Box<dyn InStream>, Box<dyn OutStream>) {
        let input: Box<dyn InStream> = match self.input {
            Input::Null => Box::new(NullInStream),
            Input::Standard => Box::new(StandardInStream::new()),
            Input::Vec(v) => Box::new(VecInStream::new(v)),
        };

        let output: Box<dyn OutStream> = match self.output {
            Output::Null => Box::new(NullOutStream),
            Output::CharStandard => Box::new(CharStandardOutStream),
            Output::IntStandard => Box::new(IntStandardOutStream),
            Output::Vec(v) => Box::new(VecOutStream::new(v)),
        };

        (input, output)
    }
}

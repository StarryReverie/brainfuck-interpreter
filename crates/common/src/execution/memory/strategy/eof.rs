use crate::execution::stream::EOF;

pub trait EofStrategy {
    fn check(&self, input: i32) -> Option<i32>;
}

#[derive(Debug)]
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

pub struct KeepEofStrategy {}

impl EofStrategy for KeepEofStrategy {
    fn check(&self, input: i32) -> Option<i32> {
        Some(input)
    }
}

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

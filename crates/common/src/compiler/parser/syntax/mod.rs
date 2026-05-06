use snafu::prelude::*;

use crate::compiler::lexer::{SingleToken, Token, TokenList};

pub type Result<T> = std::result::Result<T, SyntaxError>;

/// A single target in an `AddUntilZero` operation: add `times` to the cell at
/// `offset` from the current pointer each iteration until the current cell is zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddUntilZeroArg {
    pub offset: isize,
    pub times: i32,
}

impl AddUntilZeroArg {
    pub fn new(offset: isize, times: i32) -> Self {
        Self { offset, times }
    }
}

/// Abstract syntax tree for a Brainfuck program. Each variant represents
/// a statement or compound block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxTree {
    /// Add `val` to the current cell
    Add { val: i32 },
    /// Move the data pointer by `offset`
    Seek { offset: i32 },
    /// Set the current cell to `val`
    Set { val: i32 },
    /// Set the current cell to zero
    Clear,
    /// Seek until a nonzero cell is found
    Scan { offset: i32 },
    /// Repeatedly add to other cells until the current cell is zero
    AddUntilZero { target: Vec<AddUntilZeroArg> },
    /// Read input into the current cell
    Input,
    /// Output the current cell
    Output,
    /// Top-level container for a sequence of statements
    Root { block: Vec<SyntaxTree> },
    /// A loop (`[...]`) that executes its body while the current cell is nonzero
    Loop { block: Vec<SyntaxTree> },
}

impl SyntaxTree {
    pub fn build(token_list: TokenList) -> Result<SyntaxTree> {
        let mut current = token_list.0.into_iter();
        let mut left_bracket_count = 0;
        let block = SyntaxTree::build_impl(&mut current, &mut left_bracket_count)?;
        Ok(SyntaxTree::Root { block })
    }

    fn build_impl<I>(current: &mut I, left_bracket_count: &mut i32) -> Result<Vec<SyntaxTree>>
    where
        I: Iterator<Item = Token>,
    {
        let mut res: Vec<SyntaxTree> = vec![];

        loop {
            if let Some(Token { token, count }) = current.next() {
                match token {
                    SingleToken::Add => res.push(SyntaxTree::Add { val: count }),
                    SingleToken::GreaterThan => res.push(SyntaxTree::Seek { offset: count }),
                    SingleToken::Comma => {
                        for _ in 0..count {
                            res.push(SyntaxTree::Input)
                        }
                    }
                    SingleToken::Dot => {
                        for _ in 0..count {
                            res.push(SyntaxTree::Output)
                        }
                    }
                    SingleToken::LeftBracket => {
                        *left_bracket_count += 1;
                        let block = SyntaxTree::build_impl(current, left_bracket_count)?;
                        res.push(SyntaxTree::Loop { block })
                    }
                    SingleToken::RightBracket => {
                        *left_bracket_count -= 1;
                        ensure!(*left_bracket_count >= 0, UnpairedRightBracketSnafu);
                        break;
                    }
                    // Both `SingleToken::Sub` and `SingleToken::LessThan` have been
                    // converted to `SingleToken::Add` and `SingleToken::GreaterThan`.
                    SingleToken::Sub | SingleToken::LessThan => {}
                }
            } else {
                if *left_bracket_count == 0 {
                    break;
                } else if *left_bracket_count > 0 {
                    return Err(SyntaxError::UnpairedLeftBracket);
                }
                // It's impossible to reach where `left_bracket_count < 0`, for it has
                // been already checked above.
            }
        }

        Ok(res)
    }
}

/// Errors that can occur when building a syntax tree from tokens.
#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum SyntaxError {
    #[snafu(display("found an unpaired `[`, expected another `]`"))]
    UnpairedLeftBracket,
    #[snafu(display("found an unpaired `]`, expected another `[`"))]
    UnpairedRightBracket,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_syntax_tree() {
        let tokens = TokenList(vec![
            Token::new(SingleToken::Add, 1),
            Token::new(SingleToken::Dot, 1),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::GreaterThan, -2),
            Token::new(SingleToken::Comma, 1),
            Token::new(SingleToken::GreaterThan, 1),
            Token::new(SingleToken::RightBracket, 1),
        ]);

        let expected = Ok(SyntaxTree::Root {
            block: vec![
                SyntaxTree::Add { val: 1 },
                SyntaxTree::Output,
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Seek { offset: -2 },
                        SyntaxTree::Input,
                        SyntaxTree::Seek { offset: 1 },
                    ],
                },
            ],
        });

        assert_eq!(SyntaxTree::build(tokens), expected);
    }

    #[test]
    fn unpaired_left_bracket() {
        let tokens = TokenList(vec![
            Token::new(SingleToken::Add, 1),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::LessThan, 2),
        ]);

        let expected = Err(SyntaxError::UnpairedLeftBracket);
        assert_eq!(SyntaxTree::build(tokens), expected);
    }

    #[test]
    fn unpaired_right_bracket() {
        let tokens = TokenList(vec![
            Token::new(SingleToken::Add, 1),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::RightBracket, 1),
            Token::new(SingleToken::RightBracket, 1),
            Token::new(SingleToken::LessThan, 2),
        ]);

        let expected = Err(SyntaxError::UnpairedRightBracket);
        assert_eq!(SyntaxTree::build(tokens), expected);
    }
}

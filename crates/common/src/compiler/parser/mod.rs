pub mod optimizer;
pub mod syntax;

use snafu::prelude::*;

use crate::compiler::lexer::TokenList;

use optimizer::Optimizer;
use syntax::{SyntaxError, SyntaxTree};

type Result<T> = std::result::Result<T, ParseError>;

pub struct Parser {
    optimizer: Optimizer,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            optimizer: Optimizer::with_default_rules(),
        }
    }

    pub fn with_optimizer(optimizer: Optimizer) -> Self {
        Self { optimizer }
    }

    pub fn parse(&self, token_list: TokenList) -> Result<SyntaxTree> {
        let tree = SyntaxTree::build(token_list).context(SyntaxSnafu)?;
        let tree = self.optimizer.optimize(tree);
        Ok(tree)
    }
}

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum ParseError {
    #[snafu(display("error occurred when parsing code"))]
    Syntax { source: SyntaxError },
}

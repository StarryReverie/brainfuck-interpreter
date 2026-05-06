use crate::compiler::parser::optimizer::BlockRule;
use crate::compiler::parser::syntax::SyntaxTree;

pub struct DeadClearRule;

impl DeadClearRule {
    pub fn new() -> Self {
        Self
    }
}

impl BlockRule for DeadClearRule {
    fn apply(&self, block: &mut Vec<SyntaxTree>) {
        block.dedup_by(|a, b| matches!((a, b), (SyntaxTree::Clear, SyntaxTree::Clear)));
    }
}
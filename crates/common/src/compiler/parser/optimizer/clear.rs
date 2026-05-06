use crate::compiler::parser::syntax::SyntaxTree;

pub struct ClearRule;

impl ClearRule {
    pub fn new() -> Self {
        Self
    }
}

impl super::Rule for ClearRule {
    fn apply(&self, block: SyntaxTree) -> SyntaxTree {
        match block {
            SyntaxTree::Loop { block } => {
                if block.len() == 1 && block[0] == (SyntaxTree::Add { val: -1 }) {
                    SyntaxTree::Clear
                } else {
                    SyntaxTree::Loop { block }
                }
            }
            otherwise => otherwise,
        }
    }
}

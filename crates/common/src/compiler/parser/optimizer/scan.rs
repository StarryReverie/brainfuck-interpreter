use crate::compiler::parser::optimizer::NodeRule;
use crate::compiler::parser::syntax::SyntaxTree;

pub struct ScanRule;

impl ScanRule {
    pub fn new() -> Self {
        Self
    }
}

impl NodeRule for ScanRule {
    fn apply(&self, node: SyntaxTree) -> SyntaxTree {
        let block = match node {
            SyntaxTree::Loop { block } => block,
            otherwise => return otherwise,
        };

        if block.is_empty() {
            return SyntaxTree::Loop { block };
        }

        let mut current_offset = 0;
        for stmt in &block {
            match stmt {
                SyntaxTree::Seek { offset } => current_offset += *offset as isize,
                _ => return SyntaxTree::Loop { block },
            }
        }

        if current_offset == 0 {
            return SyntaxTree::Loop { block };
        }

        SyntaxTree::Scan { offset: current_offset as i32 }
    }
}
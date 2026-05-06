use crate::compiler::parser::optimizer::BlockRule;
use crate::compiler::parser::syntax::SyntaxTree;

pub struct SetRule;

impl SetRule {
    pub fn new() -> Self {
        Self
    }
}

impl BlockRule for SetRule {
    fn apply(&self, block: &mut Vec<SyntaxTree>) {
        let mut i = 0;
        while i + 1 < block.len() {
            let is_clear_add = matches!(
                (&block[i], &block[i + 1]),
                (SyntaxTree::Clear, SyntaxTree::Add { .. })
            );
            if is_clear_add {
                let val = match &block[i + 1] {
                    SyntaxTree::Add { val } => *val,
                    _ => unreachable!(),
                };
                block.remove(i);
                block.remove(i);
                block.insert(i, SyntaxTree::Set { val });
            } else {
                i += 1;
            }
        }
    }
}
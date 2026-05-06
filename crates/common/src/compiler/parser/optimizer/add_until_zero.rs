use crate::compiler::parser::syntax::{AddUntilZeroArg, SyntaxTree};

pub struct AddUntilZeroRule;

impl AddUntilZeroRule {
    pub fn new() -> Self {
        Self
    }
}

impl super::Rule for AddUntilZeroRule {
    fn apply(&self, block: SyntaxTree) -> SyntaxTree {
        let block = match block {
            SyntaxTree::Loop { block } => block,
            otherwise => return otherwise,
        };

        match block.first() {
            Some(SyntaxTree::Add { val: -1 }) => (),
            _ => return SyntaxTree::Loop { block },
        }

        let mut current_offset = 0;
        let mut target = Vec::with_capacity(block.len() / 2);

        for statement in block.iter().skip(1) {
            match statement {
                SyntaxTree::Add { val } => {
                    if current_offset == 0 {
                        return SyntaxTree::Loop { block };
                    }

                    target.push(AddUntilZeroArg::new(current_offset, *val))
                }
                SyntaxTree::Seek { offset } => current_offset += *offset as isize,
                _ => return SyntaxTree::Loop { block },
            }
        }

        if current_offset != 0 {
            SyntaxTree::Loop { block }
        } else {
            SyntaxTree::AddUntilZero { target }
        }
    }
}

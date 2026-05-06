use crate::compiler::parser::optimizer::NodeRule;
use crate::compiler::parser::syntax::{AddUntilZeroArg, SyntaxTree};

/// Node rule that detects the "add-until-zero" idiom:
/// `[-<offset>A...<offset>B...]` where the loop decrements the current cell
/// by -1 and then seeks to other cells adding values. This is replaced by a
/// single `AddUntilZero` instruction.
pub struct AddUntilZeroRule;

impl AddUntilZeroRule {
    pub fn new() -> Self {
        Self
    }
}

impl NodeRule for AddUntilZeroRule {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_standard_pattern() {
        let rule = AddUntilZeroRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![
                SyntaxTree::Add { val: -1 },
                SyntaxTree::Seek { offset: 2 },
                SyntaxTree::Add { val: -2 },
                SyntaxTree::Seek { offset: -3 },
                SyntaxTree::Add { val: 1 },
                SyntaxTree::Seek { offset: 1 },
            ],
        };
        let expected = SyntaxTree::AddUntilZero {
            target: vec![AddUntilZeroArg::new(2, -2), AddUntilZeroArg::new(-1, 1)],
        };
        assert_eq!(rule.apply(tree), expected);
    }

    #[test]
    fn does_not_match_if_counter_not_decremented() {
        let rule = AddUntilZeroRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![
                SyntaxTree::Add { val: -1 },
                SyntaxTree::Seek { offset: 1 },
                SyntaxTree::Add { val: 1 },
                SyntaxTree::Seek { offset: -1 },
                SyntaxTree::Add { val: -1 },
            ],
        };
        assert_eq!(rule.apply(tree.clone()), tree);
    }

    #[test]
    fn does_not_match_non_loop() {
        let rule = AddUntilZeroRule::new();
        let tree = SyntaxTree::Add { val: -1 };
        assert_eq!(rule.apply(tree.clone()), tree);
    }

    #[test]
    fn does_not_match_if_first_not_neg_one() {
        let rule = AddUntilZeroRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![SyntaxTree::Add { val: -2 }],
        };
        assert_eq!(rule.apply(tree.clone()), tree);
    }
}

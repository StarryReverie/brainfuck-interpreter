use crate::compiler::parser::optimizer::NodeRule;
use crate::compiler::parser::syntax::SyntaxTree;

/// Node rule that detects a "clear" pattern: a loop containing exactly one
/// `Add { val: -1 }`. This is equivalent to setting the current cell to zero
/// and is replaced by `Clear`.
pub struct ClearRule;

impl ClearRule {
    pub fn new() -> Self {
        Self
    }
}

impl NodeRule for ClearRule {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_simple_decrement_loop() {
        let rule = ClearRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![SyntaxTree::Add { val: -1 }],
        };
        assert_eq!(rule.apply(tree), SyntaxTree::Clear);
    }

    #[test]
    fn does_not_match_non_decrement() {
        let rule = ClearRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![SyntaxTree::Add { val: -2 }],
        };
        assert_eq!(rule.apply(tree.clone()), tree);
    }

    #[test]
    fn does_not_match_multiple_instructions() {
        let rule = ClearRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![SyntaxTree::Add { val: -1 }, SyntaxTree::Add { val: 1 }],
        };
        assert_eq!(rule.apply(tree.clone()), tree);
    }
}

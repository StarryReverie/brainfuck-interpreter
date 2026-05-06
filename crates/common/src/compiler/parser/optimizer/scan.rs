use crate::compiler::parser::optimizer::NodeRule;
use crate::compiler::parser::syntax::SyntaxTree;

/// Node rule that detects a "scan" pattern: a loop containing only `Seek`
/// instructions with a nonzero net displacement. This is replaced by a single
/// `Scan` instruction that seeks until a nonzero cell is found.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_scan_right() {
        let rule = ScanRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![SyntaxTree::Seek { offset: 1 }],
        };
        assert_eq!(rule.apply(tree), SyntaxTree::Scan { offset: 1 });
    }

    #[test]
    fn matches_scan_left() {
        let rule = ScanRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![SyntaxTree::Seek { offset: -1 }],
        };
        assert_eq!(rule.apply(tree), SyntaxTree::Scan { offset: -1 });
    }

    #[test]
    fn matches_multiple_seeks() {
        let rule = ScanRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![
                SyntaxTree::Seek { offset: 1 },
                SyntaxTree::Seek { offset: 1 },
            ],
        };
        assert_eq!(rule.apply(tree), SyntaxTree::Scan { offset: 2 });
    }

    #[test]
    fn does_not_match_zero_offset() {
        let rule = ScanRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![
                SyntaxTree::Seek { offset: 1 },
                SyntaxTree::Seek { offset: -1 },
            ],
        };
        assert_eq!(rule.apply(tree.clone()), tree);
    }

    #[test]
    fn does_not_match_with_add() {
        let rule = ScanRule::new();
        let tree = SyntaxTree::Loop {
            block: vec![
                SyntaxTree::Seek { offset: 1 },
                SyntaxTree::Add { val: -1 },
            ],
        };
        assert_eq!(rule.apply(tree.clone()), tree);
    }

    #[test]
    fn does_not_match_empty_loop() {
        let rule = ScanRule::new();
        let tree = SyntaxTree::Loop { block: vec![] };
        assert_eq!(rule.apply(tree.clone()), tree);
    }
}

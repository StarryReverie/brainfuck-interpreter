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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_consecutive_clears() {
        let rule = DeadClearRule::new();
        let mut block = vec![SyntaxTree::Clear, SyntaxTree::Clear];
        rule.apply(&mut block);
        assert_eq!(block, vec![SyntaxTree::Clear]);
    }

    #[test]
    fn removes_triple_clears() {
        let rule = DeadClearRule::new();
        let mut block = vec![
            SyntaxTree::Clear,
            SyntaxTree::Clear,
            SyntaxTree::Clear,
        ];
        rule.apply(&mut block);
        assert_eq!(block, vec![SyntaxTree::Clear]);
    }

    #[test]
    fn does_not_remove_separated_clears() {
        let rule = DeadClearRule::new();
        let mut block = vec![
            SyntaxTree::Clear,
            SyntaxTree::Add { val: 1 },
            SyntaxTree::Clear,
        ];
        let before = block.clone();
        rule.apply(&mut block);
        assert_eq!(block, before);
    }

    #[test]
    fn does_not_touch_single_clear() {
        let rule = DeadClearRule::new();
        let mut block = vec![SyntaxTree::Clear];
        let before = block.clone();
        rule.apply(&mut block);
        assert_eq!(block, before);
    }

    #[test]
    fn does_not_touch_empty_block() {
        let rule = DeadClearRule::new();
        let mut block: Vec<SyntaxTree> = vec![];
        rule.apply(&mut block);
        assert!(block.is_empty());
    }
}

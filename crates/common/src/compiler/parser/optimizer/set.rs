use crate::compiler::parser::optimizer::BlockRule;
use crate::compiler::parser::syntax::SyntaxTree;

/// Block rule that detects adjacent `Clear` followed by `Add { val }` and
/// replaces them with `Set { val }`, since clearing then adding is equivalent
/// to setting directly.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_clear_add_to_set() {
        let rule = SetRule::new();
        let mut block = vec![SyntaxTree::Clear, SyntaxTree::Add { val: 3 }];
        rule.apply(&mut block);
        assert_eq!(block, vec![SyntaxTree::Set { val: 3 }]);
    }

    #[test]
    fn converts_multiple_clear_add_pairs() {
        let rule = SetRule::new();
        let mut block = vec![
            SyntaxTree::Clear,
            SyntaxTree::Add { val: 5 },
            SyntaxTree::Clear,
            SyntaxTree::Add { val: 10 },
        ];
        rule.apply(&mut block);
        assert_eq!(
            block,
            vec![SyntaxTree::Set { val: 5 }, SyntaxTree::Set { val: 10 }]
        );
    }

    #[test]
    fn does_not_touch_unrelated_nodes() {
        let rule = SetRule::new();
        let mut block = vec![
            SyntaxTree::Input,
            SyntaxTree::Clear,
            SyntaxTree::Add { val: 7 },
            SyntaxTree::Output,
        ];
        rule.apply(&mut block);
        assert_eq!(
            block,
            vec![
                SyntaxTree::Input,
                SyntaxTree::Set { val: 7 },
                SyntaxTree::Output
            ]
        );
    }

    #[test]
    fn does_not_match_clear_then_seek() {
        let rule = SetRule::new();
        let mut block = vec![
            SyntaxTree::Clear,
            SyntaxTree::Seek { offset: 1 },
        ];
        let before = block.clone();
        rule.apply(&mut block);
        assert_eq!(block, before);
    }
}

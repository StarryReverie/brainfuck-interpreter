pub mod add_until_zero;
pub mod clear;

use crate::compiler::parser::syntax::SyntaxTree;

use add_until_zero::AddUntilZeroRule;
use clear::ClearRule;

pub trait Rule {
    fn apply(&self, block: SyntaxTree) -> SyntaxTree;
}

pub struct Optimizer {
    rules: Vec<Box<dyn Rule>>,
}

impl Optimizer {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn with_default_rules() -> Self {
        let mut optimizer = Self::new();
        optimizer.add_rule(Box::new(ClearRule::new()));
        optimizer.add_rule(Box::new(AddUntilZeroRule::new()));
        optimizer
    }

    pub fn optimize(&self, mut tree: SyntaxTree) -> SyntaxTree {
        for rule in &self.rules {
            tree = rule.apply(tree);
        }

        match tree {
            SyntaxTree::Root { block } => SyntaxTree::Root {
                block: block.into_iter().map(|tree| self.optimize(tree)).collect(),
            },
            SyntaxTree::Loop { block } => SyntaxTree::Loop {
                block: block.into_iter().map(|tree| self.optimize(tree)).collect(),
            },
            otherwise => otherwise,
        }
    }

    pub fn add_rule(&mut self, rule: Box<dyn Rule>) {
        self.rules.push(rule);
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::parser::syntax::AddUntilZeroArg;

    use super::*;

    #[test]
    fn clear_rule() {
        let mut optimizer = Optimizer::new();
        optimizer.add_rule(Box::new(ClearRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![
                SyntaxTree::Input,
                SyntaxTree::Loop {
                    block: vec![SyntaxTree::Add { val: -1 }],
                },
            ],
        };

        let tree = optimizer.optimize(tree);

        let expected = SyntaxTree::Root {
            block: vec![SyntaxTree::Input, SyntaxTree::Clear],
        };

        assert_eq!(tree, expected);
    }

    #[test]
    fn add_until_zero_rule() {
        let mut optimizer = Optimizer::new();
        optimizer.add_rule(Box::new(AddUntilZeroRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Add { val: -1 },
                        SyntaxTree::Seek { offset: 2 },
                        SyntaxTree::Add { val: -2 },
                        SyntaxTree::Seek { offset: -3 },
                        SyntaxTree::Add { val: 1 },
                        SyntaxTree::Seek { offset: 1 },
                    ],
                },
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Add { val: -1 },
                        SyntaxTree::Seek { offset: 1 },
                        SyntaxTree::Output,
                        SyntaxTree::Add { val: 1 },
                        SyntaxTree::Seek { offset: -1 },
                    ],
                },
            ],
        };

        let tree = optimizer.optimize(tree);

        let expected = SyntaxTree::Root {
            block: vec![
                SyntaxTree::AddUntilZero {
                    target: vec![AddUntilZeroArg::new(2, -2), AddUntilZeroArg::new(-1, 1)],
                },
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Add { val: -1 },
                        SyntaxTree::Seek { offset: 1 },
                        SyntaxTree::Output,
                        SyntaxTree::Add { val: 1 },
                        SyntaxTree::Seek { offset: -1 },
                    ],
                },
            ],
        };

        assert_eq!(tree, expected);
    }

    #[test]
    fn add_while_zero_rule_with_changing_the_counter_incorrectly() {
        let mut optimizer = Optimizer::new();
        optimizer.add_rule(Box::new(AddUntilZeroRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![SyntaxTree::Loop {
                block: vec![
                    SyntaxTree::Add { val: -1 },
                    SyntaxTree::Seek { offset: 1 },
                    SyntaxTree::Add { val: 1 },
                    SyntaxTree::Seek { offset: -1 },
                    SyntaxTree::Add { val: -1 },
                ],
            }],
        };

        let tree = optimizer.optimize(tree);

        let expected = SyntaxTree::Root {
            block: vec![SyntaxTree::Loop {
                block: vec![
                    SyntaxTree::Add { val: -1 },
                    SyntaxTree::Seek { offset: 1 },
                    SyntaxTree::Add { val: 1 },
                    SyntaxTree::Seek { offset: -1 },
                    SyntaxTree::Add { val: -1 },
                ],
            }],
        };

        assert_eq!(tree, expected);
    }
}

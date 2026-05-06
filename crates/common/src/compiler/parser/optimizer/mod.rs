pub mod add_until_zero;
pub mod clear;
pub mod dead_clear;
pub mod scan;
pub mod set;

use std::mem;

use crate::compiler::parser::syntax::SyntaxTree;

use add_until_zero::AddUntilZeroRule;
use clear::ClearRule;
use dead_clear::DeadClearRule;
use scan::ScanRule;
use set::SetRule;

pub trait NodeRule {
    fn apply(&self, node: SyntaxTree) -> SyntaxTree;
}

pub trait BlockRule {
    fn apply(&self, block: &mut Vec<SyntaxTree>);
}

pub struct Optimizer {
    node_rules: Vec<Box<dyn NodeRule>>,
    block_rules: Vec<Box<dyn BlockRule>>,
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            node_rules: vec![],
            block_rules: vec![],
        }
    }

    pub fn with_default_rules() -> Self {
        let mut optimizer = Self::new();
        optimizer.add_node_rule(Box::new(ClearRule::new()));
        optimizer.add_node_rule(Box::new(AddUntilZeroRule::new()));
        optimizer.add_node_rule(Box::new(ScanRule::new()));
        optimizer.add_block_rule(Box::new(DeadClearRule::new()));
        optimizer.add_block_rule(Box::new(SetRule::new()));
        optimizer
    }

    pub fn optimize(&self, tree: SyntaxTree) -> SyntaxTree {
        let mut tree = self.pass(tree);
        loop {
            let new_tree = self.pass(tree.clone());
            if new_tree == tree {
                break;
            }
            tree = new_tree;
        }
        tree
    }

    fn pass(&self, tree: SyntaxTree) -> SyntaxTree {
        match tree {
            SyntaxTree::Root { block } => {
                let block = self.optimize_block(block);
                SyntaxTree::Root { block }
            }
            SyntaxTree::Loop { block } => {
                let block = self.optimize_block(block);
                SyntaxTree::Loop { block }
            }
            otherwise => otherwise,
        }
    }

    fn optimize_block(&self, block: Vec<SyntaxTree>) -> Vec<SyntaxTree> {
        let mut block: Vec<SyntaxTree> = block
            .into_iter()
            .map(|t| self.pass(t))
            .collect();

        for i in 0..block.len() {
            for rule in &self.node_rules {
                let node = mem::replace(&mut block[i], SyntaxTree::Clear);
                block[i] = rule.apply(node);
            }
        }

        for rule in &self.block_rules {
            rule.apply(&mut block);
        }

        block
    }

    pub fn add_node_rule(&mut self, rule: Box<dyn NodeRule>) {
        self.node_rules.push(rule);
    }

    pub fn add_block_rule(&mut self, rule: Box<dyn BlockRule>) {
        self.block_rules.push(rule);
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::parser::syntax::AddUntilZeroArg;

    use super::*;

    #[test]
    fn clear_rule() {
        let mut optimizer = Optimizer::new();
        optimizer.add_node_rule(Box::new(ClearRule::new()));

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
        optimizer.add_node_rule(Box::new(AddUntilZeroRule::new()));

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
        optimizer.add_node_rule(Box::new(AddUntilZeroRule::new()));

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

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

/// A rule that transforms a single [`SyntaxTree`] node. Implementations
/// may return the original node unchanged if no transformation applies.
pub trait NodeRule {
    fn apply(&self, node: SyntaxTree) -> SyntaxTree;
}

/// A rule that operates on an entire block (sequence) of [`SyntaxTree`]
/// nodes. This is useful for patterns that span multiple adjacent statements,
/// such as `Clear` followed by `Add`.
pub trait BlockRule {
    fn apply(&self, block: &mut Vec<SyntaxTree>);
}

/// The syntax-tree optimizer. It applies [`NodeRule`]s and [`BlockRule`]s
/// in a bottom-up, multi-pass fashion until the tree stops changing.
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
    fn multi_pass_converges() {
        let mut optimizer = Optimizer::new();
        optimizer.add_block_rule(Box::new(DeadClearRule::new()));
        optimizer.add_block_rule(Box::new(SetRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![
                SyntaxTree::Clear,
                SyntaxTree::Clear,
                SyntaxTree::Add { val: 5 },
            ],
        };

        let tree = optimizer.optimize(tree);

        assert_eq!(tree, SyntaxTree::Root {
            block: vec![SyntaxTree::Set { val: 5 }],
        });
    }

    #[test]
    fn bottom_up_clears_inner_loop_first() {
        let mut optimizer = Optimizer::new();
        optimizer.add_node_rule(Box::new(ClearRule::new()));
        optimizer.add_block_rule(Box::new(SetRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![SyntaxTree::Loop {
                block: vec![
                    SyntaxTree::Loop {
                        block: vec![SyntaxTree::Add { val: -1 }],
                    },
                    SyntaxTree::Add { val: 3 },
                ],
            }],
        };

        let tree = optimizer.optimize(tree);

        assert_eq!(tree, SyntaxTree::Root {
            block: vec![SyntaxTree::Loop {
                block: vec![SyntaxTree::Set { val: 3 }],
            }],
        });
    }

    #[test]
    fn node_rules_then_block_rules() {
        let mut optimizer = Optimizer::new();
        optimizer.add_node_rule(Box::new(ClearRule::new()));
        optimizer.add_block_rule(Box::new(SetRule::new()));

        let tree = SyntaxTree::Loop {
            block: vec![
                SyntaxTree::Loop {
                    block: vec![SyntaxTree::Add { val: -1 }],
                },
                SyntaxTree::Add { val: 7 },
            ],
        };

        let tree = optimizer.optimize(tree);

        assert_eq!(tree, SyntaxTree::Loop {
            block: vec![SyntaxTree::Set { val: 7 }],
        });
    }

    #[test]
    fn add_until_zero_in_context() {
        let mut optimizer = Optimizer::new();
        optimizer.add_node_rule(Box::new(AddUntilZeroRule::new()));
        optimizer.add_node_rule(Box::new(ScanRule::new()));
        optimizer.add_block_rule(Box::new(DeadClearRule::new()));
        optimizer.add_block_rule(Box::new(SetRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![SyntaxTree::Loop {
                block: vec![
                    SyntaxTree::Add { val: -1 },
                    SyntaxTree::Seek { offset: 2 },
                    SyntaxTree::Add { val: -2 },
                    SyntaxTree::Seek { offset: -3 },
                    SyntaxTree::Add { val: 1 },
                    SyntaxTree::Seek { offset: 1 },
                ],
            }],
        };

        let tree = optimizer.optimize(tree);

        assert_eq!(tree, SyntaxTree::Root {
            block: vec![SyntaxTree::AddUntilZero {
                target: vec![AddUntilZeroArg::new(2, -2), AddUntilZeroArg::new(-1, 1)],
            }],
        });
    }

    #[test]
    fn scan_rule_in_context() {
        let mut optimizer = Optimizer::new();
        optimizer.add_node_rule(Box::new(ScanRule::new()));

        let tree = SyntaxTree::Root {
            block: vec![SyntaxTree::Loop {
                block: vec![
                    SyntaxTree::Seek { offset: 1 },
                    SyntaxTree::Seek { offset: 1 },
                ],
            }],
        };

        let tree = optimizer.optimize(tree);

        assert_eq!(tree, SyntaxTree::Root {
            block: vec![SyntaxTree::Scan { offset: 2 }],
        });
    }
}

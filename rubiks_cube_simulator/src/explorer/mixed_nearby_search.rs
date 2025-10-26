use super::edge_modifier::{EdgeModifier, EdgeSwapModifier};
use super::modifier::{CornerModifier, SwapModifier};
use crate::cube::State;
use crate::inspection::{CornerOperation, EdgeOperation};

/// 混合操作（Corner + Edge）の列挙型
#[derive(Debug, Clone, PartialEq)]
pub enum MixedOperation {
    Corner(CornerOperation),
    Edge(EdgeOperation),
}

impl std::fmt::Display for MixedOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixedOperation::Corner(op) => write!(f, "{}", op),
            MixedOperation::Edge(op) => write!(f, "{}", op),
        }
    }
}

/// 混合操作の修正情報
#[derive(Debug, Clone)]
pub enum MixedModifier {
    Corner {
        step: usize,
        modifier: CornerModifier,
    },
    Edge {
        step: usize,
        modifier: EdgeModifier,
    },
}

/// 混合操作列の修正後シーケンス
#[derive(Debug, Clone)]
pub struct ModifiedMixedSequence {
    base_operations: Vec<MixedOperation>,
    modifiers: Vec<MixedModifier>,
}

impl ModifiedMixedSequence {
    /// 新しい ModifiedMixedSequence を作成
    pub fn new(base_operations: Vec<MixedOperation>) -> Self {
        Self {
            base_operations,
            modifiers: Vec::new(),
        }
    }

    /// 修正を追加
    pub fn add_modifier(&mut self, modifier: MixedModifier) {
        self.modifiers.push(modifier);
    }

    /// 修正後の操作列を取得
    pub fn get_sequence(&self) -> Vec<MixedOperation> {
        let mut operations = self.base_operations.clone();

        for modifier in &self.modifiers {
            match modifier {
                MixedModifier::Corner {
                    step,
                    modifier: corner_mod,
                } => {
                    if let Some(MixedOperation::Corner(ref mut op)) = operations.get_mut(*step) {
                        // Corner操作の修正を適用
                        match corner_mod {
                            CornerModifier::Swap(swap_mod) => {
                                *op = CornerOperation::Swap(swap_mod.modifier.clone());
                            }
                            CornerModifier::Twist(twist_mod) => {
                                *op = CornerOperation::Twist(twist_mod.modifier.clone());
                            }
                        }
                    }
                }
                MixedModifier::Edge {
                    step,
                    modifier: edge_mod,
                } => {
                    if let Some(MixedOperation::Edge(ref mut op)) = operations.get_mut(*step) {
                        // Edge操作の修正を適用
                        match edge_mod {
                            EdgeModifier::Swap(swap_mod) => {
                                *op = EdgeOperation::Swap(swap_mod.modifier.clone());
                            }
                            EdgeModifier::Flip(flip_mod) => {
                                *op = EdgeOperation::Flip(flip_mod.modifier.clone());
                            }
                        }
                    }
                }
            }
        }

        operations
    }

    /// 修正の説明を取得
    pub fn get_description(&self) -> String {
        let mut descriptions = Vec::new();

        for modifier in &self.modifiers {
            match modifier {
                MixedModifier::Corner {
                    step,
                    modifier: corner_mod,
                } => {
                    descriptions.push(format!("Step {}: Corner {:?}", step + 1, corner_mod));
                }
                MixedModifier::Edge {
                    step,
                    modifier: edge_mod,
                } => {
                    descriptions.push(format!("Step {}: Edge {:?}", step + 1, edge_mod));
                }
            }
        }

        descriptions.join(", ")
    }
}

impl std::fmt::Display for ModifiedMixedSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operations = self.get_sequence();
        for (i, op) in operations.iter().enumerate() {
            let marker = if self.modifiers.iter().any(|m| match m {
                MixedModifier::Corner { step, .. } => *step == i,
                MixedModifier::Edge { step, .. } => *step == i,
            }) {
                "**"
            } else {
                ""
            };
            writeln!(f, "Step {}: {}{}{}", i + 1, marker, op, marker)?;
        }
        Ok(())
    }
}

/// 混合操作列の近傍を探索する構造体
pub struct NearbyMixedOperationSearch {
    base_operations: Vec<MixedOperation>,
}

impl NearbyMixedOperationSearch {
    /// 新しい NearbyMixedOperationSearch を作成
    pub fn new(base_operations: Vec<MixedOperation>) -> Self {
        Self { base_operations }
    }

    /// 元の操作列から最大1つの操作を変更したバリエーションを生成
    pub fn explore_variants_one_change(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedMixedSequence, State)> {
        let mut variants = Vec::new();

        for step_index in 0..self.base_operations.len() {
            match &self.base_operations[step_index] {
                MixedOperation::Corner(corner_op) => {
                    // Corner操作の代替案を生成
                    if let CornerOperation::Swap(_) = corner_op {
                        let alternatives = self.generate_corner_swap_alternatives();
                        for alt_swap in alternatives {
                            let mut modified =
                                ModifiedMixedSequence::new(self.base_operations.clone());
                            modified.add_modifier(MixedModifier::Corner {
                                step: step_index,
                                modifier: CornerModifier::Swap(SwapModifier {
                                    step: step_index,
                                    modifier: alt_swap,
                                }),
                            });

                            let final_state = self
                                .apply_mixed_operations(initial_state, &modified.get_sequence());
                            variants.push((modified, final_state));
                        }
                    }
                }
                MixedOperation::Edge(edge_op) => {
                    // Edge操作の代替案を生成
                    if let EdgeOperation::Swap(_) = edge_op {
                        let alternatives = self.generate_edge_swap_alternatives();
                        for alt_swap in alternatives {
                            let mut modified =
                                ModifiedMixedSequence::new(self.base_operations.clone());
                            modified.add_modifier(MixedModifier::Edge {
                                step: step_index,
                                modifier: EdgeModifier::Swap(EdgeSwapModifier {
                                    step: step_index,
                                    modifier: alt_swap,
                                }),
                            });

                            let final_state = self
                                .apply_mixed_operations(initial_state, &modified.get_sequence());
                            variants.push((modified, final_state));
                        }
                    }
                }
            }
        }

        variants
    }

    /// 元の操作列から最大2つの操作を変更したバリエーションを生成
    pub fn explore_variants_two_changes(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedMixedSequence, State)> {
        let mut variants = Vec::new();

        // 1つの変更のバリエーションも含める
        variants.extend(self.explore_variants_one_change(initial_state));

        // 2つの変更のバリエーション
        for step1 in 0..self.base_operations.len() {
            for step2 in (step1 + 1)..self.base_operations.len() {
                // 両方の操作について代替案を生成
                let alternatives1 = self.get_alternatives_for_step(step1);
                let alternatives2 = self.get_alternatives_for_step(step2);

                for alt1 in &alternatives1 {
                    for alt2 in &alternatives2 {
                        let mut modified = ModifiedMixedSequence::new(self.base_operations.clone());
                        modified.add_modifier(alt1.clone());
                        modified.add_modifier(alt2.clone());

                        let final_state =
                            self.apply_mixed_operations(initial_state, &modified.get_sequence());
                        variants.push((modified, final_state));
                    }
                }
            }
        }

        variants
    }

    /// 指定したステップの操作に対する代替案を生成
    fn get_alternatives_for_step(&self, step: usize) -> Vec<MixedModifier> {
        let mut alternatives = Vec::new();

        match &self.base_operations[step] {
            MixedOperation::Corner(corner_op) => {
                if let CornerOperation::Swap(_) = corner_op {
                    let corner_alternatives = self.generate_corner_swap_alternatives();
                    for alt in corner_alternatives {
                        alternatives.push(MixedModifier::Corner {
                            step,
                            modifier: CornerModifier::Swap(SwapModifier {
                                step,
                                modifier: alt,
                            }),
                        });
                    }
                }
            }
            MixedOperation::Edge(edge_op) => {
                if let EdgeOperation::Swap(_) = edge_op {
                    let edge_alternatives = self.generate_edge_swap_alternatives();
                    for alt in edge_alternatives {
                        alternatives.push(MixedModifier::Edge {
                            step,
                            modifier: EdgeModifier::Swap(EdgeSwapModifier {
                                step,
                                modifier: alt,
                            }),
                        });
                    }
                }
            }
        }

        alternatives
    }

    /// Corner Swap操作の代替案を生成
    fn generate_corner_swap_alternatives(&self) -> Vec<crate::inspection::CornerSwapOperation> {
        use crate::inspection::CornerSwapOperation;
        let mut alternatives = Vec::new();

        const CORNER_BUFFER: usize = 2; // UFR

        // 各コーナー位置について
        for target in 0..8 {
            if target == CORNER_BUFFER {
                continue;
            }

            // 各方向について
            for orientation in 0..3 {
                alternatives.push(CornerSwapOperation::new(CORNER_BUFFER, target, orientation));
            }
        }

        alternatives
    }

    /// Edge Swap操作の代替案を生成
    fn generate_edge_swap_alternatives(&self) -> Vec<crate::inspection::EdgeSwapOperation> {
        use crate::inspection::EdgeSwapOperation;
        let mut alternatives = Vec::new();

        const EDGE_BUFFER: usize = 6; // UF

        // 各エッジ位置について
        for target in 0..12 {
            if target == EDGE_BUFFER {
                continue;
            }

            // 各方向について
            for orientation in 0..2 {
                alternatives.push(EdgeSwapOperation::new(EDGE_BUFFER, target, orientation));
            }
        }

        alternatives
    }

    /// 混合操作列を状態に適用
    fn apply_mixed_operations(
        &self,
        initial_state: &State,
        operations: &[MixedOperation],
    ) -> State {
        let mut state = initial_state.clone();

        for operation in operations {
            match operation {
                MixedOperation::Corner(corner_op) => {
                    state = corner_op.apply(&mut state);
                }
                MixedOperation::Edge(edge_op) => {
                    state = edge_op.apply(&mut state);
                }
            }
        }

        state
    }
}

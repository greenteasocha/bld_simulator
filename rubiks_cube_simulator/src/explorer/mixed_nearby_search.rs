use super::mixed_modifier::{
    MixedCornerSwapModifier, MixedCornerTwistModifier, MixedEdgeFlipModifier,
    MixedEdgeSwapModifier, MixedModifier, ModifiedMixedSequence,
};
use crate::cube::State;

/// 混合操作（Corner + Edge）の列挙型
/// 各バリアントは CubeOperation を実装する型のみ
#[derive(Debug, Clone, PartialEq)]
pub enum MixedOperation {
    CornerSwap(crate::inspection::CornerSwapOperation),
    CornerTwist(crate::inspection::CornerTwistOperation),
    EdgeSwap(crate::inspection::EdgeSwapOperation),
    EdgeFlip(crate::inspection::EdgeFlipOperation),
}

// 将来的には、より汎用的な形として以下のような定義も可能:
// pub enum MixedOperation<CS, CT, ES, EF>
// where
//     CS: CubeOperation,
//     CT: CubeOperation,
//     ES: CubeOperation,
//     EF: CubeOperation,
// {
//     CornerSwap(CS),
//     CornerTwist(CT),
//     EdgeSwap(ES),
//     EdgeFlip(EF),
// }

impl std::fmt::Display for MixedOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixedOperation::CornerSwap(op) => write!(f, "{}", op),
            MixedOperation::CornerTwist(op) => write!(f, "{}", op),
            MixedOperation::EdgeSwap(op) => write!(f, "{}", op),
            MixedOperation::EdgeFlip(op) => write!(f, "{}", op),
        }
    }
}

impl ApplyableToState for MixedOperation {
    fn apply_to_state(&self, state: &State) -> State {
        match self {
            MixedOperation::CornerSwap(op) => op.apply(state),
            MixedOperation::CornerTwist(op) => op.apply(state),
            MixedOperation::EdgeSwap(op) => op.apply(state),
            MixedOperation::EdgeFlip(op) => op.apply(state),
        }
    }
}

/// Alternative generator trait for generating alternative operations
pub trait AlternativeGenerator<T> {
    fn generate_alternatives(&self, operation: &T) -> Vec<T>;
}

/// Trait for operations that can be applied to a State
pub trait ApplyableToState {
    fn apply_to_state(&self, state: &State) -> State;
}

/// 混合操作列の近傍を探索する構造体
pub struct NearbyMixedOperationSearch {
    base_operations: Vec<MixedOperation>,
    alternative_generators: Vec<Box<dyn AlternativeGenerator<MixedOperation>>>,
}

impl NearbyMixedOperationSearch {
    /// 新しい NearbyMixedOperationSearch を作成
    pub fn new(base_operations: Vec<MixedOperation>) -> Self {
        Self {
            base_operations,
            alternative_generators: Vec::new(),
        }
    }

    /// Alternative generator を追加
    pub fn add_alternative_generator(
        &mut self,
        generator: Box<dyn AlternativeGenerator<MixedOperation>>,
    ) {
        self.alternative_generators.push(generator);
    }

    /// Alternative generators 付きで作成
    pub fn with_alternative_generators(
        base_operations: Vec<MixedOperation>,
        generators: Vec<Box<dyn AlternativeGenerator<MixedOperation>>>,
    ) -> Self {
        Self {
            base_operations,
            alternative_generators: generators,
        }
    }

    /// 指定した操作に対する代替案を生成
    fn get_alternative_operations(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        let mut alternatives = Vec::new();

        for generator in &self.alternative_generators {
            alternatives.extend(generator.generate_alternatives(operation));
        }

        alternatives
    }

    /// 元の操作列から最大1つの操作を変更したバリエーションを生成
    pub fn explore_variants_one_change(
        &self,
        initial_state: &State,
    ) -> Vec<(ModifiedMixedSequence, State)> {
        let mut variants = Vec::new();

        for step_index in 0..self.base_operations.len() {
            let alternatives = self.get_alternative_operations(&self.base_operations[step_index]);

            for alternative in alternatives {
                let mut modified = ModifiedMixedSequence::new(self.base_operations.clone());
                let modifier = self.create_modifier(step_index, &alternative);
                modified.add_modifier(modifier);

                let final_state = modified.apply_to_state(initial_state);
                variants.push((modified, final_state));
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

        // 各ステップの代替案を事前に生成
        let variants_for_each_step: Vec<Vec<MixedOperation>> = self
            .base_operations
            .iter()
            .map(|op| self.get_alternative_operations(op))
            .collect();

        // 2つの変更のバリエーション
        for step1 in 0..self.base_operations.len() {
            for step2 in (step1 + 1)..self.base_operations.len() {
                for alt_op1 in &variants_for_each_step[step1] {
                    for alt_op2 in &variants_for_each_step[step2] {
                        let mut modified = ModifiedMixedSequence::new(self.base_operations.clone());
                        let modifier1 = self.create_modifier(step1, alt_op1);
                        let modifier2 = self.create_modifier(step2, alt_op2);
                        modified.add_modifier(modifier1);
                        modified.add_modifier(modifier2);

                        let final_state = modified.apply_to_state(initial_state);
                        variants.push((modified, final_state));
                    }
                }
            }
        }

        variants
    }

    /// MixedOperationからMixedModifierを作成
    fn create_modifier(&self, step: usize, operation: &MixedOperation) -> MixedModifier {
        match operation {
            MixedOperation::CornerSwap(op) => MixedModifier::CornerSwap(MixedCornerSwapModifier {
                step,
                modifier: op.clone(),
            }),
            MixedOperation::CornerTwist(op) => {
                MixedModifier::CornerTwist(MixedCornerTwistModifier {
                    step,
                    modifier: op.clone(),
                })
            }
            MixedOperation::EdgeSwap(op) => MixedModifier::EdgeSwap(MixedEdgeSwapModifier {
                step,
                modifier: op.clone(),
            }),
            MixedOperation::EdgeFlip(op) => MixedModifier::EdgeFlip(MixedEdgeFlipModifier {
                step,
                modifier: op.clone(),
            }),
        }
    }
}

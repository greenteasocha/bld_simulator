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


/// 混合操作列の修正後シーケンス
#[derive(Debug, Clone)]
pub struct ModifiedMixedSequence<T> {
    base_operations: Vec<T>,
    modified_operations: Vec<(usize, T)>, // (step_index, replacement_operation)
}

impl<T: Clone> ModifiedMixedSequence<T> {
    /// 新しい ModifiedMixedSequence を作成
    pub fn new(base_operations: Vec<T>) -> Self {
        Self {
            base_operations,
            modified_operations: Vec::new(),
        }
    }

    /// 修正を追加
    pub fn add_modification(&mut self, step_index: usize, replacement: T) {
        self.modified_operations.push((step_index, replacement));
    }

    /// 修正後の操作列を取得
    pub fn get_sequence(&self) -> Vec<T> {
        let mut operations = self.base_operations.clone();

        // 修正を適用
        for (step_index, replacement) in &self.modified_operations {
            if let Some(op) = operations.get_mut(*step_index) {
                *op = replacement.clone();
            }
        }

        operations
    }

    /// 修正の説明を取得
    pub fn get_description(&self) -> String 
    where 
        T: std::fmt::Debug,
    {
        let descriptions: Vec<String> = self.modified_operations
            .iter()
            .map(|(step, replacement)| format!("Step {}: {:?}", step + 1, replacement))
            .collect();
        
        descriptions.join(", ")
    }
}

impl<T> std::fmt::Display for ModifiedMixedSequence<T> 
where 
    T: Clone + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operations = self.get_sequence();
        for (i, op) in operations.iter().enumerate() {
            let marker = if self.modified_operations.iter().any(|(step, _)| *step == i) {
                "**"
            } else {
                ""
            };
            writeln!(f, "Step {}: {}{}{}", i + 1, marker, op, marker)?;
        }
        Ok(())
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
pub struct NearbyMixedOperationSearch<T> {
    base_operations: Vec<T>,
    alternative_generators: Vec<Box<dyn AlternativeGenerator<T>>>,
}

impl<T> NearbyMixedOperationSearch<T> 
where 
    T: Clone + std::fmt::Debug,
{
    /// 新しい NearbyMixedOperationSearch を作成
    pub fn new(base_operations: Vec<T>) -> Self {
        Self { 
            base_operations,
            alternative_generators: Vec::new(),
        }
    }

    /// Alternative generator を追加
    pub fn add_alternative_generator(&mut self, generator: Box<dyn AlternativeGenerator<T>>) {
        self.alternative_generators.push(generator);
    }

    /// Alternative generators 付きで作成
    pub fn with_alternative_generators(
        base_operations: Vec<T>,
        generators: Vec<Box<dyn AlternativeGenerator<T>>>,
    ) -> Self {
        Self {
            base_operations,
            alternative_generators: generators,
        }
    }

    /// 指定した操作に対する代替案を生成
    fn get_alternative_operations(&self, operation: &T) -> Vec<T> {
        let mut alternatives = Vec::new();
        
        for generator in &self.alternative_generators {
            alternatives.extend(generator.generate_alternatives(operation));
        }
        
        alternatives
    }

    /// 元の操作列から最大1つの操作を変更したバリエーションを生成
    pub fn explore_variants_one_change(&self, initial_state: &State) -> Vec<(ModifiedMixedSequence<T>, State)> 
    where 
        T: ApplyableToState,
    {
        let mut variants = Vec::new();

        for step_index in 0..self.base_operations.len() {
            let alternatives = self.get_alternative_operations(&self.base_operations[step_index]);
            
            for alternative in alternatives {
                let mut modified = ModifiedMixedSequence::new(self.base_operations.clone());
                modified.add_modification(step_index, alternative);
                
                let final_state = self.apply_operations_to_state(initial_state, &modified.get_sequence());
                variants.push((modified, final_state));
            }
        }

        variants
    }

    /// 元の操作列から最大2つの操作を変更したバリエーションを生成
    pub fn explore_variants_two_changes(&self, initial_state: &State) -> Vec<(ModifiedMixedSequence<T>, State)> 
    where 
        T: ApplyableToState,
    {
        let mut variants = Vec::new();

        // 1つの変更のバリエーションも含める
        variants.extend(self.explore_variants_one_change(initial_state));

        // 各ステップの代替案を事前に生成
        let variants_for_each_step: Vec<Vec<T>> = self.base_operations.iter()
            .map(|op| self.get_alternative_operations(op))
            .collect();

        // 2つの変更のバリエーション
        for step1 in 0..self.base_operations.len() {
            for step2 in (step1 + 1)..self.base_operations.len() {
                for alt_op1 in &variants_for_each_step[step1] {
                    for alt_op2 in &variants_for_each_step[step2] {
                        let mut modified = ModifiedMixedSequence::new(self.base_operations.clone());
                        modified.add_modification(step1, alt_op1.clone());
                        modified.add_modification(step2, alt_op2.clone());

                        let final_state = self.apply_operations_to_state(initial_state, &modified.get_sequence());
                        variants.push((modified, final_state));
                    }
                }
            }
        }

        variants
    }

    /// 操作列を状態に適用
    fn apply_operations_to_state(&self, initial_state: &State, operations: &[T]) -> State 
    where 
        T: ApplyableToState,
    {
        let mut state = initial_state.clone();
        
        for operation in operations {
            state = operation.apply_to_state(&state);
        }
        
        state
    }

}

/// Corner Swap用のAlternative Generator
pub struct CornerSwapAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for CornerSwapAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::CornerSwap(_) => {
                self.generate_corner_swap_alternatives()
                    .into_iter()
                    .map(|swap_op| MixedOperation::CornerSwap(swap_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl CornerSwapAlternativeGenerator {
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
}

/// Edge Swap用のAlternative Generator
pub struct EdgeSwapAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for EdgeSwapAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::EdgeSwap(_) => {
                self.generate_edge_swap_alternatives()
                    .into_iter()
                    .map(|swap_op| MixedOperation::EdgeSwap(swap_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl EdgeSwapAlternativeGenerator {
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
}

/// Corner Twist用のAlternative Generator
pub struct CornerTwistAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for CornerTwistAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::CornerTwist(_) => {
                self.generate_corner_twist_alternatives()
                    .into_iter()
                    .map(|twist_op| MixedOperation::CornerTwist(twist_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl CornerTwistAlternativeGenerator {
    fn generate_corner_twist_alternatives(&self) -> Vec<crate::inspection::CornerTwistOperation> {
        use crate::inspection::CornerTwistOperation;
        let mut alternatives = Vec::new();

        // 各コーナー位置について
        for target in 0..8 {
            // 各回転方向について (1=CW, 2=CCW)
            for twist in 1..=2 {
                alternatives.push(CornerTwistOperation::new(target, twist));
            }
        }

        alternatives
    }
}

/// Edge Flip用のAlternative Generator
pub struct EdgeFlipAlternativeGenerator;

impl AlternativeGenerator<MixedOperation> for EdgeFlipAlternativeGenerator {
    fn generate_alternatives(&self, operation: &MixedOperation) -> Vec<MixedOperation> {
        match operation {
            MixedOperation::EdgeFlip(_) => {
                self.generate_edge_flip_alternatives()
                    .into_iter()
                    .map(|flip_op| MixedOperation::EdgeFlip(flip_op))
                    .collect()
            },
            _ => vec![],
        }
    }
}

impl EdgeFlipAlternativeGenerator {
    fn generate_edge_flip_alternatives(&self) -> Vec<crate::inspection::EdgeFlipOperation> {
        use crate::inspection::EdgeFlipOperation;
        let mut alternatives = Vec::new();

        // 各エッジ位置について
        for target in 0..12 {
            alternatives.push(EdgeFlipOperation::new(target));
        }

        alternatives
    }
}

use crate::cube::State;
use crate::inspection::{CornerInspection, CornerOperation, EdgeInspection, EdgeOperation};
use crate::inspection::{MoveSequence, OperationsToTurns};

/// BLD (Blindfolded) solving workflow
///
/// Corner と Edge の解析を統合し、ルービックキューブを解くための操作列を生成します。
pub struct BldWorkflow {
    operations_converter: OperationsToTurns,
}

/// Corner と Edge の操作を含む完全な解法
#[derive(Debug, Clone)]
pub struct BldSolution {
    /// Corner 操作列
    pub corner_operations: Vec<CornerOperation>,
    /// Edge 操作列
    pub edge_operations: Vec<EdgeOperation>,
    /// 結合された全操作列 (Edge → Corner の順)
    pub all_operations: AllOperations,
    /// Move Sequence (Edge → Corner の順)
    pub move_sequence: MoveSequence,
}

/// Corner と Edge の操作を統合した列挙型
#[derive(Debug, Clone)]
pub enum Operation {
    Corner(CornerOperation),
    Edge(EdgeOperation),
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Corner(op) => write!(f, "{}", op),
            Operation::Edge(op) => write!(f, "{}", op),
        }
    }
}

/// 全操作列（Edge と Corner の両方）
#[derive(Debug, Clone)]
pub struct AllOperations {
    operations: Vec<Operation>,
}

impl AllOperations {
    /// 新しい AllOperations を作成
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Edge 操作を追加
    pub fn push_edge(&mut self, op: EdgeOperation) {
        self.operations.push(Operation::Edge(op));
    }

    /// Corner 操作を追加
    pub fn push_corner(&mut self, op: CornerOperation) {
        self.operations.push(Operation::Corner(op));
    }

    /// 操作列への参照を取得
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    /// 操作数を取得
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// 空かどうか
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

impl std::fmt::Display for AllOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, op) in self.operations.iter().enumerate() {
            writeln!(f, "Step {}: {}", i + 1, op)?;
        }
        Ok(())
    }
}

impl Default for AllOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl BldWorkflow {
    /// BldWorkflow を初期化
    ///
    /// # Arguments
    /// * `ufr_expanded_json` - Corner 2-swap 用の JSON データ
    /// * `ufr_parity_json` - Corner parity 用の JSON データ
    /// * `ufr_twist_json` - Corner twist 用の JSON データ
    /// * `uf_expanded_json` - Edge 2-swap 用の JSON データ
    /// * `uf_flip_json` - Edge flip 用の JSON データ
    ///
    /// # Returns
    /// 初期化された BldWorkflow インスタンス
    pub fn new(
        ufr_expanded_json: &str,
        ufr_parity_json: &str,
        ufr_twist_json: &str,
        uf_expanded_json: &str,
        uf_flip_json: &str,
    ) -> Result<Self, String> {
        let operations_converter = OperationsToTurns::new(
            ufr_expanded_json,
            ufr_parity_json,
            ufr_twist_json,
            uf_expanded_json,
            uf_flip_json,
        )?;

        Ok(Self {
            operations_converter,
        })
    }

    /// 完全な BLD 解法を生成
    ///
    /// # Arguments
    /// * `state` - 現在のキューブ状態
    ///
    /// # Returns
    /// Corner と Edge の操作列、および Move Sequence を含む BldSolution
    ///
    /// # ワークフロー
    /// 1. Corner workflow を実行し、Corner 用の操作列を取得
    /// 2. Corner の Swap 操作数が奇数の場合、Edge workflow で交換分析モードを使用
    /// 3. Edge workflow を実行し、Edge 用の操作列を取得
    /// 4. Edge → Corner の順で操作列を結合
    /// 5. 操作列から Move Sequence を生成
    pub fn solve(&self, state: &State) -> Result<BldSolution, String> {
        // 1. Corner workflow を実行
        let corner_operations =
            CornerInspection::solve_corner_permutation_with_orientation(state);

        // 2. 交換分析の有無を判断
        let corner_swap_count = corner_operations
            .iter()
            .filter(|op| matches!(op, CornerOperation::Swap(_)))
            .count();
        let use_swap_inspection = corner_swap_count % 2 == 1;

        // 3. Edge workflow を実行
        let edge_operations =
            EdgeInspection::solve_edge_permutation_with_orientation(state, use_swap_inspection);

        // 4. 操作列を結合 (Edge → Corner)
        let mut all_operations = AllOperations::new();
        
        // Edge の操作を全て追加
        for op in &edge_operations {
            all_operations.push_edge(op.clone());
        }
        
        // Corner の操作を全て追加
        for op in &corner_operations {
            all_operations.push_corner(op.clone());
        }

        // 5. Move Sequence を生成 (Edge → Corner)
        let mut move_sequence = MoveSequence::empty();
        
        // Edge operations を move sequences に変換
        let edge_sequences = self.operations_converter.convert_edge_operations(&edge_operations)?;
        for seq in edge_sequences {
            move_sequence.extend(seq);
        }
        
        // Corner operations を move sequences に変換
        let corner_sequences = self.operations_converter.convert(&corner_operations)?;
        for seq in corner_sequences {
            move_sequence.extend(seq);
        }

        Ok(BldSolution {
            corner_operations,
            edge_operations,
            all_operations,
            move_sequence,
        })
    }

    /// 解法を人間が読みやすい形式でフォーマット
    pub fn format_solution(solution: &BldSolution) -> String {
        let mut result = String::new();

        result.push_str("=== BLD Solution ===\n\n");

        // Corner 操作
        result.push_str("Corner Operations:\n");
        if solution.corner_operations.is_empty() {
            result.push_str("  (none)\n");
        } else {
            for (i, op) in solution.corner_operations.iter().enumerate() {
                result.push_str(&format!("  Step {}: {}\n", i + 1, op));
            }
        }
        result.push('\n');

        // Edge 操作
        result.push_str("Edge Operations:\n");
        if solution.edge_operations.is_empty() {
            result.push_str("  (none)\n");
        } else {
            for (i, op) in solution.edge_operations.iter().enumerate() {
                result.push_str(&format!("  Step {}: {}\n", i + 1, op));
            }
        }
        result.push('\n');

        // 全操作列 (Edge → Corner)
        result.push_str("All Operations (Edge → Corner):\n");
        if solution.all_operations.is_empty() {
            result.push_str("  (none)\n");
        } else {
            result.push_str(&format!("{}", solution.all_operations));
        }
        result.push('\n');

        // Move Sequence
        result.push_str("Move Sequence:\n");
        result.push_str(&format!("  {}\n", solution.move_sequence));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // テスト用のBldWorkflowを作成するヘルパー関数
    fn create_test_workflow() -> BldWorkflow {
        let ufr_expanded = r#"{
            "RDB": { "RDF": "R U R' U R U2 R'" },
            "RDF": { "UFL": "U R U' R' U R U2 R'" },
            "UBR": { "UBL": "L' U' L U' L' U2 L" },
            "BUR": { "BUL": "U L' U L U' L' U2 L" }
        }"#;
        
        let ufr_parity = r#"{
            "RDB": "R U R' U R U2 R'",
            "UBR": "R U R' U R U2 R'",
            "RUB": "U R U' R' U R U2 R'",
            "BUR": "R U2 R' U' R U' R'",
            "UBL": "L' U' L U' L' U2 L",
            "BUL": "U' L' U L U' L' U2 L",
            "LUB": "L' U2 L U L' U L"
        }"#;
        
        let ufr_twist = r#"{
            "FUL": "R' D R D' R' D R U' R' D' R D R' D' R U"
        }"#;
        
        let uf_expanded = r#"{
            "FR": { "DL": "R U R' F R F'" },
            "RU": { "BL": "U R U' R' U R U R'" }
        }"#;
        
        let uf_flip = r#"{
            "UB": "R U R' U R U2 R'",
            "UR": "R U R' U R U2 R' U"
        }"#;
        
        BldWorkflow::new(&ufr_expanded, &ufr_parity, &ufr_twist, &uf_expanded, &uf_flip)
            .expect("Failed to create test workflow")
    }

    #[test]
    fn test_bld_workflow_simple() {
        // 簡単なテストケース
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== BLD Workflow Simple Test ===");
        
        let workflow = create_test_workflow();
        let solution = workflow.solve(&state).expect("Failed to solve");
        
        println!("{}", BldWorkflow::format_solution(&solution));
        
        // Corner は 1 Swap 操作 → 奇数なので Edge は交換分析モード
        assert_eq!(solution.corner_operations.len(), 1);
        
        // Edge も 1 Swap 操作
        assert_eq!(solution.edge_operations.len(), 1);
        
        // 全操作列は Edge + Corner = 2
        assert_eq!(solution.all_operations.len(), 2);
    }

    #[test]
    fn test_bld_workflow_already_solved() {
        let state = State::solved();
        
        println!("\n=== BLD Workflow Already Solved Test ===");
        
        let workflow = create_test_workflow();
        let solution = workflow.solve(&state).expect("Failed to solve");
        
        println!("{}", BldWorkflow::format_solution(&solution));
        
        assert!(solution.corner_operations.is_empty());
        assert!(solution.edge_operations.is_empty());
        assert!(solution.all_operations.is_empty());
        assert!(solution.move_sequence.is_empty());
    }

    #[test]
    fn test_bld_workflow_corner_only() {
        // Corner のみが崩れている状態
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== BLD Workflow Corner Only Test ===");
        
        let workflow = create_test_workflow();
        let solution = workflow.solve(&state).expect("Failed to solve");
        
        println!("{}", BldWorkflow::format_solution(&solution));
        
        // Corner は 1 Swap 操作
        assert_eq!(solution.corner_operations.len(), 1);
        
        // Edge は交換分析モード（奇数）で、ep は全て正しい位置
        // しかし交換分析モードでは ep[5]=5, ep[6]=6 ではなく ep[5]=6, ep[6]=5 が終了状態
        // 初期状態では ep[5]=5, ep[6]=6 なので、調整が必要
        assert!(!solution.edge_operations.is_empty());
    }

    #[test]
    fn test_bld_workflow_edge_only() {
        // Edge のみが崩れている状態
        let state = State::new(
            [0, 1, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== BLD Workflow Edge Only Test ===");
        
        let workflow = create_test_workflow();
        let solution = workflow.solve(&state).expect("Failed to solve");
        
        println!("{}", BldWorkflow::format_solution(&solution));
        
        // Corner は操作なし
        assert!(solution.corner_operations.is_empty());
        
        // Edge は 1 Swap 操作（通常モード、偶数 = 0）
        assert_eq!(solution.edge_operations.len(), 1);
    }

    #[test]
    fn test_bld_workflow_complex() {
        // 複雑なケース
        let state = State::new(
            [1, 0, 6, 2, 5, 4, 3, 7],
            [2, 1, 0, 2, 2, 2, 2, 1],
            [4, 1, 10, 8, 5, 0, 9, 7, 6, 2, 11, 3],
            [0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1],
        );

        println!("\n=== BLD Workflow Complex Test ===");
        
        let workflow = create_test_workflow();
        let solution = workflow.solve(&state).expect("Failed to solve");
        
        println!("{}", BldWorkflow::format_solution(&solution));
        
        // Corner Swap 数を確認
        let corner_swap_count = solution
            .corner_operations
            .iter()
            .filter(|op| matches!(op, CornerOperation::Swap(_)))
            .count();
        
        println!("Corner Swap count: {}", corner_swap_count);
        println!("Use swap inspection: {}", corner_swap_count % 2 == 1);
        
        // 操作が生成されていることを確認
        assert!(!solution.corner_operations.is_empty());
        assert!(!solution.edge_operations.is_empty());
        
        // 全操作列が Edge + Corner の合計と一致
        assert_eq!(
            solution.all_operations.len(),
            solution.edge_operations.len() + solution.corner_operations.len()
        );
    }

    #[test]
    fn test_bld_workflow_operation_order() {
        // 操作の順序を確認
        let state = State::new(
            [1, 0, 2, 3, 4, 5, 6, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        let workflow = create_test_workflow();
        let solution = workflow.solve(&state).expect("Failed to solve");

        println!("\n=== BLD Workflow Operation Order Test ===");
        println!("All operations:");
        for (i, op) in solution.all_operations.operations().iter().enumerate() {
            println!("  Step {}: {:?}", i + 1, op);
        }

        // 最初の操作は Edge
        assert!(matches!(
            solution.all_operations.operations()[0],
            Operation::Edge(_)
        ));

        // 最後の操作は Corner
        let last_idx = solution.all_operations.len() - 1;
        assert!(matches!(
            solution.all_operations.operations()[last_idx],
            Operation::Corner(_)
        ));
    }

    #[test]
    fn test_bld_workflow_parity() {
        // パリティのテスト（Corner Swap が奇数）
        let state = State::new(
            [1, 2, 0, 3, 4, 5, 6, 7], // 3つの Corner Swap が必要（奇数でない場合は調整）
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );

        println!("\n=== BLD Workflow Parity Test ===");
        
        let workflow = create_test_workflow();
        let solution = workflow.solve(&state).expect("Failed to solve");
        
        let corner_swap_count = solution
            .corner_operations
            .iter()
            .filter(|op| matches!(op, CornerOperation::Swap(_)))
            .count();
        
        println!("Corner Swap count: {}", corner_swap_count);
        println!("Parity (odd swaps): {}", corner_swap_count % 2 == 1);
        
        println!("{}", BldWorkflow::format_solution(&solution));
    }
}

# Operations to Turns の実装完了

## 概要
`Vec<CornerOperation>` から `Vec<MoveSequence>` への変換機能を実装しました。

## 実装内容

### ファイル構成
- `src/inspection/operations_to_turns.rs`: 新規作成
- `src/inspection/mod.rs`: モジュール公開を更新
- `src/lib.rs`: 型のエクスポートを更新
- `examples/operations_to_turns.rs`: 動作確認用のexample
- `tests/operations_to_turns_test.rs`: 統合テスト

### 主要な型

#### `MoveSequence`
```rust
pub struct MoveSequence {
    pub moves: Sequence,
    pub description: String,
}
```
- `moves`: 実際の手順（NotationMoveの列）
- `description`: 手順の説明（例: "BDR → RDF", "Parity: BDR", "Twist: FUL"）

#### `OperationsToTurns`
```rust
pub struct OperationsToTurns {
    ufr_expanded: HashMap<String, HashMap<String, String>>,
    ufr_parity: HashMap<String, String>,
    ufr_twist: HashMap<String, String>,
}
```

### 変換ロジック

#### 優先順位
1. **連続する2つのSwap** → `ufr_expanded.json` から取得
2. **1つのSwap** → `ufr_parity.json` から取得
3. **1つのTwist** → `ufr_twist.json` から取得

#### TARGET_STICKERの定義

**Swap用**:
```rust
const TARGET_STICKERS: [[&str; 3]; 8] = [
    ["UBL", "BUL", "LUB"], // 0
    ["UBR", "RUB", "BUR"], // 1
    ["UFR", "FUR", "RUF"], // 2
    ["UFL", "LUF", "FUL"], // 3
    ["DBL", "LDB", "BDL"], // 4
    ["DBR", "BDR", "RDB"], // 5
    ["DFR", "RDF", "FDR"], // 6
    ["DFL", "FDL", "LDF"], // 7
];
```

**Twist用**:
```rust
const TWIST_TARGET_STICKERS: [[&str; 3]; 8] = [
    ["UBL", "LUB", "BUL"], // 0
    ["UBR", "BUR", "RUB"], // 1
    ["UFR", "RUF", "FUR"], // 2
    ["UFL", "FUL", "LUF"], // 3
    ["DBL", "BDL", "LDB"], // 4
    ["DBR", "RDB", "BDR"], // 5
    ["DFR", "FDR", "RDF"], // 6
    ["DFL", "LDF", "FDL"], // 7
];
```

### 使用例

```rust
// JSONファイルを読み込む
let ufr_expanded = fs::read_to_string("resources/ufr_expanded.json")?;
let ufr_parity = fs::read_to_string("resources/ufr_parity.json")?;
let ufr_twist = fs::read_to_string("resources/ufr_twist.json")?;

// OperationsToTurns を初期化
let converter = OperationsToTurns::new(&ufr_expanded, &ufr_parity, &ufr_twist)?;

// 操作列を作成
let operations = vec![
    CornerOperation::Swap(CornerSwapOperation::new(2, 5, 2)), // BDR
    CornerOperation::Swap(CornerSwapOperation::new(2, 6, 1)), // RDF
];

// 手順列に変換
let sequences = converter.convert(&operations)?;

// 結果を表示
for seq in &sequences {
    println!("{}: {}", seq.description, sequence_to_string(&seq.moves));
}
```

### テスト

#### 単体テスト
`src/inspection/operations_to_turns.rs` 内に以下のテストを含む:
- `test_convert_two_swaps`: 2つのSwapの変換
- `test_convert_single_swap`: 1つのSwapの変換（Parity）
- `test_convert_twist`: 1つのTwistの変換
- `test_mixed_operations`: 混合操作の変換

#### 統合テスト
`tests/operations_to_turns_test.rs` に実データを使用したテストを含む:
- 実際のJSONファイルを使用した変換テスト
- 複数の操作パターンの検証

### 実行方法

```bash
# 単体テストを実行
cargo test --lib operations_to_turns

# 統合テストを実行
cargo test --test operations_to_turns_test

# 例を実行
cargo run --example operations_to_turns
```

## 備考

### 前提条件
- `CornerInspection::solve_corner_permutation_with_orientation` が生成する操作列では、
  全てのSwap操作の `target1` が `BUFFER_PIECE` (2 = UFR) であることが保証される
- この前提により、`target2` と `orientation` のみを使用してTARGET_STICKERを決定できる

### エラーハンドリング
- JSONファイルのパースエラー
- 必要な手順がJSONに存在しない場合のエラー
- Move文字列のパースエラー

すべて `Result<T, String>` で適切にエラーメッセージを返す。

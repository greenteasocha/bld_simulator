ある操作列の一部分を変更する、という操作を再利用しやすいように、nearby_search から独立した構造体として定義したい。

SwapModifier{
    step: int,
    modifier: CornerSwapOperation,
}

TwistModifier{
    step: int,
    modifier: CornerTwistOperation,
}

CornerModifier = SwapModifier | TwistModifier

ModifiedSequence{
    original_sequence: Sequence,
    modifiers: Vec<CornerModifier>,
}

NearbyOperationSearch.explore_variants は ModifiedSequence を返却するようにする。



次に、impl std::fmt::Display for ModifiedSequence を実装する。
基本的には original の各要素について CornerOperation の Display を呼び出すが、modifier が存在する step については、CornerOperation をラップして独自の Display を実装する。

```
use std::fmt;

struct ModifiedCornerOperation {
    Operation: ModifiedCornerOperation,
}

impl std::fmt::Display for ModifiedCornerOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CornerOperation::Swap(op) => write!(f, "**{}**", op),
            CornerOperation::Twist(op) => write!(f, "**{}**", op),
        }
    }
}
```








# Modifier実装完了

## 実装内容

ドキュメント013に基づき、操作列の一部を変更する因子を構造体として定義しました。

### 新しい型

#### 1. **SwapModifier**
```rust
pub struct SwapModifier {
    pub step: usize,
    pub modifier: CornerSwapOperation,
}
```
Swap操作の変更を表す。

#### 2. **TwistModifier**
```rust
pub struct TwistModifier {
    pub step: usize,
    pub modifier: CornerTwistOperation,
}
```
Twist操作の変更を表す。

#### 3. **CornerModifier**
```rust
pub enum CornerModifier {
    Swap(SwapModifier),
    Twist(TwistModifier),
}
```
Corner操作の変更を表す列挙型。

#### 4. **ModifiedSequence**
```rust
pub struct ModifiedSequence {
    pub original_sequence: Vec<CornerOperation>,
    pub modifiers: Vec<CornerModifier>,
}
```
変更された操作列を表す。元の操作列と変更のリストを保持。

### 主な機能

#### ModifiedSequence
- `new()` - 新しいModifiedSequenceを作成
- `add_modifier()` - 変更を追加
- `get_sequence()` - 実際の操作列を取得（変更を適用済み）
- `is_modified()` - 指定したステップが変更されているか確認
- `Display` trait実装 - **で変更箇所を強調表示

#### CornerModifier
- `step()` - 変更対象のステップ番号を取得
- `operation()` - 変更後のCornerOperationを取得

### NearbyOperationSearchの変更

#### 戻り値の変更
```rust
// 修正前
pub fn explore_variants(&self, initial_state: &State) -> Vec<(Vec<CornerOperation>, State)>

// 修正後
pub fn explore_variants(&self, initial_state: &State) -> Vec<(ModifiedSequence, State)>
```

#### format_variantの変更
```rust
// 修正前
pub fn format_variant(operations: &[CornerOperation]) -> String

// 修正後
pub fn format_variant(modified: &ModifiedSequence) -> String
```

### Display実装の特徴

ModifiedSequenceのDisplayでは、変更されたステップを`**`で囲んで強調表示：

```
Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: **Swap: 0 ↔ 5 (ori: 2)**  ← 変更された
Step 3: Twist: corner[0] (ori: 1)
```

## 使用例

```rust
use rubiks_cube_simulator::{
    State, NearbyOperationSearch, CornerInspection,
    ModifiedSequence, SwapModifier, CornerModifier,
    CornerSwapOperation,
};

// 初期状態
let state = State::new(...);

// 元の解法を取得
let solution = CornerInspection::solve_corner_permutation_with_orientation(&state);

// NearbyOperationSearchで近傍を探索
let searcher = NearbyOperationSearch::new(solution);
let variants = searcher.explore_variants(&state);

// 各バリエーションを表示
for (modified, final_state) in variants {
    println!("{}", modified);  // 変更箇所が**で強調される
    println!("Solved: {}", final_state.is_solved());
}

// 手動でModifiedSequenceを作成
let mut modified = ModifiedSequence::new(solution);
modified.add_modifier(CornerModifier::Swap(SwapModifier {
    step: 2,
    modifier: CornerSwapOperation::new(0, 5, 1),
}));
println!("{}", modified);
```

## テスト

以下のテストを実装：
- `test_modified_sequence_basic` - 基本的な変更機能
- `test_modified_sequence_display` - Display実装の確認
- `test_corner_modifier_step` - ステップ番号取得
- `test_multiple_modifiers` - 複数の変更

既存のNearbyOperationSearchのテストもすべて更新済み。

## ファイル構成

```
src/explorer/
├── mod.rs                         # モジュール定義（modifier追加）
├── modifier.rs                    # 新規作成
├── nearby_search.rs               # ModifiedSequence対応に更新
└── wrong_operation_detector.rs    # 変更なし
```

## ビルド・テスト

```bash
# ビルド
cargo build

# テスト実行
cargo test explorer::modifier
cargo test explorer::nearby_search

# 全テスト
cargo test
```

## 既存コードへの影響

- `NearbyOperationSearch::explore_variants`の戻り値が変更されているため、この関数を使用している箇所は修正が必要
- Display出力が変更されているため、出力形式に依存するコードがあれば確認が必要

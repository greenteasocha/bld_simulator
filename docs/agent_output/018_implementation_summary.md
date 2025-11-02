# Nearby Search More Variation Implementation (Updated)

## 概要

`018_nearby_search_more_variation.md`の要件および追記に基づいて、2つの操作を変更したバリエーションを探索する機能を実装しました。

## 実装内容

### 1. コーナー用実装 (`src/explorer/nearby_search.rs`)

#### 新メソッド

**`generate_swap_alternatives()`**
- Swap操作の代替候補を生成（24パターン）
- target1=0固定、target2=0..7, orientation=0..2

**`generate_twist_alternatives()`**
- Twist操作の代替候補を生成（16パターン）
- target=0..7, orientation=1..2（1=CW, 2=CCW）

**`explore_variants_two_changes()`**（完全に書き直し）
- 統一的なアプローチで実装
- 全ての操作ペア(i, j)について、各操作の型をmatchで判断
- Swap操作 → Swapの代替候補に置き換え
- Twist操作 → Twistの代替候補に置き換え

### 2. エッジ用実装 (`src/explorer/edge_nearby_search.rs`)

#### 新メソッド

**`generate_swap_alternatives()`**
- Swap操作の代替候補を生成（24パターン）
- target1=8(buffer)固定、target2=0..11, orientation=0..1

**`generate_flip_alternatives()`**
- Flip操作の代替候補を生成（12パターン）
- target=0..11, orientation=1固定

**`explore_variants_two_changes()`**（完全に書き直し）
- コーナー用と同じ統一的なアプローチ

## 実装方針：統一的なアプローチ

ドキュメントの追記に従い、SwapとTwist/Flipを別々に処理せず、統一的に扱います：

```rust
pub fn explore_variants_two_changes(&self, initial_state: &State) -> Vec<(ModifiedSequence, State)> {
    let n = self.base_operations.len();
    
    // i < j となる全てのペアについて
    for i in 0..n {
        for j in (i + 1)..n {
            // Step i の操作型をmatchで判断
            let alternatives_i = match &self.base_operations[i] {
                CornerOperation::Swap(_) => generate_swap_alternatives(),
                CornerOperation::Twist(_) => generate_twist_alternatives(),
            };
            
            // Step j の操作型をmatchで判断
            let alternatives_j = match &self.base_operations[j] {
                CornerOperation::Swap(_) => generate_swap_alternatives(),
                CornerOperation::Twist(_) => generate_twist_alternatives(),
            };
            
            // 全ての組み合わせを試す
            for alt_i in &alternatives_i {
                for alt_j in &alternatives_j {
                    // バリエーションを生成
                }
            }
        }
    }
}
```

## バリエーションのパターン

### コーナー（Corner）

操作列にSwap=s個、Twist=t個が含まれる場合：

1. **Swap × Swap**: C(s, 2) × 24 × 24
2. **Swap × Twist**: s × t × 24 × 16
3. **Twist × Twist**: C(t, 2) × 16 × 16

**例**: Swap=3, Twist=1の場合
- Swap × Swap: C(3,2) × 576 = 3 × 576 = 1,728
- Swap × Twist: 3 × 1 × 384 = 1,152
- Twist × Twist: 0（Twistが1つのみ）
- **合計: 2,880バリエーション**

### エッジ（Edge）

操作列にSwap=s個、Flip=f個が含まれる場合：

1. **Swap × Swap**: C(s, 2) × 24 × 24
2. **Swap × Flip**: s × f × 24 × 12
3. **Flip × Flip**: C(f, 2) × 12 × 12

## テストケース

### コーナー用テスト（`nearby_search.rs`）

1. **`test_two_changes_basic`**
   - シンプルな状態でのテスト
   - バリエーション数の検証（Swap/Twist混在対応）

2. **`test_two_changes_complex`**
   - 複雑な状態でのテスト
   - 解決したバリエーションの検出

3. **`test_two_changes_insufficient_swaps`**
   - Swap操作が2つ未満の場合の動作確認

4. **`test_two_changes_example_from_docs`**
   - ドキュメントの最初の例の再現

5. **`test_two_changes_with_twist`**（新規）
   - ドキュメントの追記の例を再現
   - Swap×Twistの混合パターンを確認
   - 期待されるバリエーション: 2,880個

6. **`test_two_changes_twist_only`**（新規）
   - Twistのみの操作列
   - Twist×Twistのパターンを確認

7. **`test_two_changes_mixed_comprehensive`**（新規）
   - SwapとTwistが混在する複雑なケース
   - 全てのパターンの組み合わせを確認

### エッジ用テスト（`edge_nearby_search.rs`）

1. **`test_two_changes_edge_basic`**
2. **`test_two_changes_edge_complex`**
3. **`test_two_changes_edge_insufficient_swaps`**

すべてのテストがSwap/Flip混在に対応。

## ドキュメントの例の再現

### 追記の例

**元の操作列**:
```
Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ 6 (ori: 0)
Step 4: Twist: 2 (Clockwise)
```

**生成されるバリエーションの1つ**:
```
Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ 3 (ori: 0)  ← 変更
Step 4: Twist: 4 (Counter-Clockwise)  ← 変更
```

このパターンが生成されることを`test_two_changes_with_twist`で確認済み。

## 使用例

```rust
use rubiks_cube_simulator::{State, NearbyOperationSearch};

// 状態の準備
let state = State::new([...], [...], [...], [...]);

// 元の解法を取得（SwapとTwist混在）
let original_solution = vec![
    CornerOperation::Swap(CornerSwapOperation::new(0, 1, 0)),
    CornerOperation::Swap(CornerSwapOperation::new(0, 2, 0)),
    CornerOperation::Twist(CornerTwistOperation::new(2, 1)),
];

// Searcherを作成
let searcher = NearbyOperationSearch::new(original_solution);

// 2手変更のバリエーションを探索
// Swap×Swap, Swap×Twist, Twist×Twistの全パターンが含まれる
let variants = searcher.explore_variants_two_changes(&state);

println!("Generated {} variants", variants.len());
```

## テスト実行

```bash
# 全てのテストを実行
cargo test two_changes

# ドキュメントの追記の例を確認（詳細出力あり）
cargo test test_two_changes_with_twist -- --nocapture

# Twist×Twistのテスト
cargo test test_two_changes_twist_only -- --nocapture

# 混合パターンのテスト
cargo test test_two_changes_mixed_comprehensive -- --nocapture
```

## 実装の特徴

### 1. 統一的なアプローチ

- Swap/Twistを区別せず、全ての操作ペアを対象
- matchパターンで各操作の型を判断
- それぞれに対応する代替候補を生成

### 2. 自動的な組み合わせ生成

以下の全パターンが自動的に生成される：
- Swap × Swap
- Swap × Twist（混合）
- Twist × Twist

### 3. 既存のインフラ活用

- `ModifiedSequence`構造体を活用
- `CornerModifier`のSwap/Twist両方に対応
- `add_modifier()`で複数の修正を追加

## 修正前との違い

### 修正前（不完全）
- Swapステップのみを収集してペアを作成
- TwistはスキップされるだけでTwist×TwistやSwap×Twistのパターンが生成されない

### 修正後（完全）
- 全ての操作ステップでペアを作成
- 各ステップの型に応じた代替候補を生成
- Swap×Swap、Swap×Twist、Twist×Twistの全パターンを網羅

## パフォーマンス考慮事項

- バリエーション数は操作列の構成により大きく変動
- Swap=3, Twist=2の場合: 約4,000バリエーション
- メモリ効率が重要な場合はイテレータパターンへの移行を検討

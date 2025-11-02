# Edge BLD Solver Implementation

エッジピースのBLD（Blindfolded）ソルバーを実装しました。コーナーソルバーと同様の機能をエッジピースに対して提供します。

## 実装内容

### 1. 基本機能 (inspection/edge_solver.rs)

- **EdgeSwapOperation**: エッジの2点交換（向き考慮）
- **EdgeFlipOperation**: エッジの向き変更
- **EdgeInspection**: 完成状態までの操作列を自動計算

```rust
use rubiks_cube_simulator::cube::State;
use rubiks_cube_simulator::inspection::EdgeInspection;

let state = State::new(/* ... */);
let operations = EdgeInspection::solve_edge_permutation_with_orientation(&state);
```

### 2. エクスプローラー機能

#### 近傍探索 (explorer/edge_nearby_search.rs)
操作列の1ステップを変更した全バリエーションを生成

```rust
use rubiks_cube_simulator::explorer::NearbyEdgeOperationSearch;

let searcher = NearbyEdgeOperationSearch::new(operations);
let variants = searcher.explore_variants(&state);
```

#### 誤操作検出 (explorer/edge_wrong_operation_detector.rs)
誤った最終状態から適用した操作列を推測

```rust
use rubiks_cube_simulator::explorer::WrongEdgeOperationDetector;

let detector = WrongEdgeOperationDetector::new(initial_state);
let detected = detector.detect_wrong_operation(&wrong_state);
```

### 3. サンプルプログラム

```bash
# 基本ワークフロー
cargo run --example edge_bld_workflow

# 詳細ワークフロー
cargo run --example edge_bld_workflow_detailed
```

## エッジの表記

### ステッカー表記 (TARGET_STICKERS)
```
[["BL", "LB"], ["BR", "RB"], ["FR", "RF"], ["FL", "LF"],
 ["UB", "BU"], ["UR", "RU"], ["UF", "FU"], ["UL", "LU"],
 ["DB", "BD"], ["DR", "RD"], ["DF", "FD"], ["DL", "LD"]]
```

### 向きの表現
- `0`: "not flipped" (フリップなし)
- `1`: "flipped" (フリップ)

## テスト

```bash
# ユニットテスト
cargo test edge_solver

# 統合テスト
cargo test edge_solver_integration
```

## コーナーとの対応

| 項目 | コーナー | エッジ |
|------|---------|--------|
| バッファー | 0 (UBL) | 8 (DB) |
| 向きの値域 | {0,1,2} | {0,1} |
| ピース数 | 8個 | 12個 |
| 代替操作数 | 24通り | 24通り |

詳細は `docs/017_edge_implementation_summary.md` を参照してください。

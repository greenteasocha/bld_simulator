# テスト実行ガイド

## エッジソルバーの実装テスト

### 1. ユニットテスト（各モジュール単位）

#### エッジソルバーのコアロジック
```bash
# edge_solver.rsのすべてのテスト
cargo test --lib edge_solver

# 特定のテストのみ
cargo test --lib test_ep_only_example
cargo test --lib test_with_orientation
cargo test --lib test_flip_only
cargo test --lib test_complex_case
cargo test --lib test_debug_step_by_step
```

#### エッジモディファイア
```bash
# edge_modifier.rsのテスト
cargo test --lib edge_modifier
```

#### エッジ近傍探索
```bash
# edge_nearby_search.rsのテスト
cargo test --lib edge_nearby_search
```

#### エッジ誤操作検出
```bash
# edge_wrong_operation_detector.rsのテスト
cargo test --lib edge_wrong_operation_detector
```

### 2. 統合テスト

#### エッジソルバー統合テスト
```bash
# tests/edge_solver_integration.rsのすべてのテスト
cargo test --test edge_solver_integration

# 個別のテスト
cargo test --test edge_solver_integration test_edge_solver_integration
cargo test --test edge_solver_integration test_edge_nearby_search_integration
cargo test --test edge_solver_integration test_edge_wrong_operation_detector_integration
cargo test --test edge_solver_integration test_edge_swap_orientation_logic
cargo test --test edge_solver_integration test_edge_swap_orientation_edge_cases
cargo test --test edge_solver_integration test_edge_flip_operation
```

#### コーナー＆エッジ統合テスト
```bash
# tests/corner_edge_integration.rsのすべてのテスト
cargo test --test corner_edge_integration

# 個別のテスト
cargo test --test corner_edge_integration test_corner_and_edge_solver_together
cargo test --test corner_edge_integration test_complex_scramble
cargo test --test corner_edge_integration test_corner_only_scramble
cargo test --test corner_edge_integration test_edge_only_scramble
```

### 3. すべてのテストを一括実行

```bash
# プロジェクト全体のテスト（コーナー + エッジ）
cargo test

# 詳細な出力付き（printlnの内容も表示）
cargo test -- --nocapture

# 特定のキーワードを含むテストのみ
cargo test edge
cargo test corner
```

### 4. サンプルプログラムの実行

#### エッジBLDワークフロー
```bash
# 基本ワークフロー
cargo run --example edge_bld_workflow

# 詳細ワークフロー（ステップバイステップ）
cargo run --example edge_bld_workflow_detailed
```

#### コーナーBLDワークフロー（既存）
```bash
cargo run --example corner_bld_workflow
cargo run --example corner_bld_workflow_detailed
```

### 5. ビルドの確認

```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release

# ドキュメント生成
cargo doc --open
```

## テストの説明

### ユニットテスト

#### edge_solver.rs
- `test_ep_only_example`: 置換のみ（向きなし）
- `test_with_orientation`: 置換と向きの両方
- `test_flip_only`: 向きのみ（置換なし）
- `test_already_solved`: 完成状態
- `test_complex_case`: 複雑なケース
- `test_debug_step_by_step`: デバッグ出力付き

### 統合テスト

#### edge_solver_integration.rs
- `test_edge_solver_integration`: 基本的な統合テスト
- `test_edge_nearby_search_integration`: 近傍探索の動作確認
- `test_edge_wrong_operation_detector_integration`: 誤操作検出
- `test_edge_swap_orientation_logic`: eo変化ロジックの検証
- `test_edge_swap_orientation_edge_cases`: エッジケースの検証
- `test_edge_flip_operation`: フリップ操作の検証

#### corner_edge_integration.rs
- `test_corner_and_edge_solver_together`: コーナーとエッジ両方の解法
- `test_complex_scramble`: 複雑なスクランブルの解法
- `test_corner_only_scramble`: コーナーのみのスクランブル
- `test_edge_only_scramble`: エッジのみのスクランブル

## 推奨テスト順序

1. **基本動作確認**
   ```bash
   cargo test --lib test_ep_only_example
   cargo test --lib test_with_orientation
   ```

2. **エッジソルバーの完全テスト**
   ```bash
   cargo test --lib edge_solver
   ```

3. **統合テスト**
   ```bash
   cargo test --test edge_solver_integration
   ```

4. **コーナー＆エッジ統合**
   ```bash
   cargo test --test corner_edge_integration
   ```

5. **全体テスト**
   ```bash
   cargo test
   ```

## トラブルシューティング

### テストが失敗する場合

1. **ビルドエラー**
   ```bash
   cargo clean
   cargo build
   ```

2. **詳細なエラー情報**
   ```bash
   cargo test -- --nocapture --test-threads=1
   ```

3. **特定のテストのみデバッグ**
   ```bash
   cargo test test_edge_swap_orientation_logic -- --nocapture
   ```

## 期待される結果

すべてのテストが成功した場合、以下のような出力が表示されます：

```
running X tests
test test_edge_solver_integration ... ok
test test_edge_nearby_search_integration ... ok
test test_edge_wrong_operation_detector_integration ... ok
...

test result: ok. X passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 次のステップ

テストが成功したら：
1. サンプルプログラムを実行して動作を確認
2. 独自のスクランブルでテスト
3. operations_to_turnsのエッジ版実装（オプション）

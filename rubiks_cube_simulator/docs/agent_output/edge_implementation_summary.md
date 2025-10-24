# Edge Solver Implementation Summary

## 実装完了したファイル

### 1. インスペクション (src/inspection/)

#### edge_solver.rs
- `EdgeSwapOperation`: エッジの2点交換操作（eo考慮版）
- `EdgeFlipOperation`: エッジの向き変更操作
- `EdgeOperation`: Swap/Flipを統合した操作型
- `EdgeInspection`: ep/eoを完成状態に戻すための操作列を計算

**特徴:**
- バッファーピース: インデックス8 (DB)
- TARGET_STICKERS: BL/LB, BR/RB, FR/RF, FL/LF, UB/BU, UR/RU, UF/FU, UL/LU, DB/BD, DR/RD, DF/FD, DL/LD
- FLIP_EXISTANCE: "not flipped" (0), "flipped" (1)
- eo変化ロジック: 交換時に両方のエッジに同じorientationを加算

### 2. エクスプローラー (src/explorer/)

#### edge_modifier.rs
- `EdgeSwapModifier`: Swap操作の変更
- `EdgeFlipModifier`: Flip操作の変更
- `EdgeModifier`: 操作変更を統合
- `ModifiedEdgeSequence`: 変更された操作列を管理

#### edge_nearby_search.rs
- `NearbyEdgeOperationSearch`: 操作列から1ステップ変更したバリエーションを生成
- 代替操作: 12 targets × 2 orientations = 24通り

#### edge_wrong_operation_detector.rs
- `WrongEdgeOperationDetector`: ユーザーの操作ミスを検出
- 誤った最終状態から適用した操作列を推測

### 3. サンプルプログラム (examples/)

#### edge_bld_workflow.rs
- 基本的なエッジBLDワークフローの例
- スクランブル → 解法計算 → 適用 → 検証

#### edge_bld_workflow_detailed.rs
- 詳細なステップバイステップの実行例
- 各操作の前後状態を表示

### 4. テスト (tests/)

#### edge_solver_integration.rs
- 統合テスト
  - edge_solver: 複雑なケースでの解法
  - nearby_search: バリアント生成
  - wrong_operation_detector: 誤操作検出
  - swap_orientation_logic: eo変化ロジックの検証

### 5. モジュール設定

#### src/inspection/mod.rs
- `edge_solver`モジュールを追加
- エッジ関連の型を公開

#### src/explorer/mod.rs
- エッジ関連のモジュールを追加
- 公開API更新

#### src/lib.rs
- エッジ関連の型をライブラリAPIに追加

## コーナーとの主な違い

1. **バッファーピース**
   - コーナー: 0 (UBL)
   - エッジ: 8 (DB)

2. **向きの値域**
   - コーナー: co ∈ {0, 1, 2} (3方向)
   - エッジ: eo ∈ {0, 1} (2方向)

3. **向き変化ロジック**
   - コーナー: 非対称 `(co[t2] + ori) % 3`, `(co[t1] - ori + 3) % 3`
   - エッジ: 対称 `(eo[t2] + ori) % 2`, `(eo[t1] + ori) % 2`

4. **代替操作の数**
   - コーナー: 8 targets × 3 orientations = 24通り
   - エッジ: 12 targets × 2 orientations = 24通り

5. **表示**
   - コーナー: "counter-clockwise", "clockwise"
   - エッジ: "not flipped", "flipped"

## 実装の確認方法

```bash
# テスト実行
cargo test edge_solver
cargo test edge_solver_integration

# サンプル実行
cargo run --example edge_bld_workflow
cargo run --example edge_bld_workflow_detailed
```

## 次のステップ

ドキュメント016に記載されている通り、以下も実装可能:
- operations_to_turnsのエッジ版（JSONからの操作→回転列変換）
- エッジ用の3style CSVパーサー
- 回転列に対する分岐探索

これらはコーナー実装を参考に同様のパターンで実装できます。

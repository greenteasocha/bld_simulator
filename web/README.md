# Cross Solver WebUI

ブラウザ上で動作するルービックキューブBLD（目隠し）ソルバーのWebインターフェース。

## 🆕 V2: 構造化データ対応

V2では、Rustの構造体をそのままJavaScript/TypeScriptで扱えるようになりました！

### V1 vs V2 の違い

| 機能 | V1 (`index.html`) | V2 (`index-v2.html`) |
|------|------------------|---------------------|
| データ形式 | 文字列 | 構造化データ（JSON） |
| 表示カスタマイズ | ❌ 不可 | ✅ 自由にカスタマイズ可能 |
| インタラクティブ性 | 低 | 高 |
| UIデザイン | シンプル | カード型、統計表示 |
| 推奨用途 | シンプルな表示 | 拡張・カスタマイズ |

## セットアップ

### 前提条件

1. Rustプロジェクトをwasmにビルド済みであること
```bash
# プロジェクトルートで実行
wasm-pack build --target web
```

2. Node.jsがインストールされていること

### 起動方法

```bash
# web/ ディレクトリに移動
cd web

# サーバーを起動（デフォルト: ポート8080）
npm run serve

# または特定のポートで起動
npm run serve:3000
```

起動後、ブラウザで以下にアクセス：

**V1 (従来版):**
```
http://localhost:8080
```

**V2 (構造化データ版):**
```
http://localhost:8080/index-v2.html
```

## 使い方

### 基本的な使い方

1. **スクランブル入力**: テキストボックスにスクランブル手順を入力
   - 例: `R U R' U'`
   - 例: `R U2 R' D R U' R' D'`

2. **解くボタン**: ボタンをクリックまたはEnterキーで解法を計算

3. **結果表示**: V2では以下の情報がカード形式で表示されます
   - 統計情報（操作数の概要）
   - Corner Operations（コーナー操作）
   - Edge Operations（エッジ操作）
   - Move Sequences（実行手順）

### V2の特徴

#### 📊 統計カード
- 総操作数
- コーナー操作数
- エッジ操作数
- 手順数

#### 🎴 操作カード
各操作がカード形式で表示され、以下の情報が含まれます：
- **Swap**: 交換する2つのステッカー
- **Twist**: 回転するコーナーと方向
- **Flip**: フリップするエッジ

#### 🎨 カラーコーディング
- 紫（Swap）: 2点交換操作
- ピンク（Twist）: 回転操作
- 明るいピンク（Flip）: フリップ操作

## 開発者向け

### TypeScript型定義

`src/types.ts`に型定義があります：

```typescript
import type { 
    CornerOperation, 
    EdgeOperation, 
    BldSolutionDataV2 
} from './types';

// 型安全にデータを扱える
function processOperation(op: CornerOperation) {
    if (op.type === 'Swap') {
        console.log('Swap:', op.Swap.target1, op.Swap.target2);
    }
}
```

### カスタム表示の実装例

```javascript
// 操作をリスト形式で表示
function renderAsList(operations) {
    return operations.map((op, i) => {
        const formatted = formatCornerOperation(op);
        return `${i + 1}. ${formatted.details}`;
    }).join('\n');
}

// 操作をグラフ形式で可視化
function visualizeOperations(operations) {
    // D3.js や Chart.js などで可視化
}

// アニメーション付きで表示
function animateOperations(operations) {
    operations.forEach((op, i) => {
        setTimeout(() => {
            // 操作を順番に表示
        }, i * 1000);
    });
}
```

### 新しいWASM関数の使用

```javascript
import { solve_bld_with_default_moveset_v2 } from '/pkg/bld_simulator.js';

const result = solve_bld_with_default_moveset_v2(cpArray, coArray, epArray, eoArray);

// 構造化データとして受け取れる
console.log(result.solution.corner_operations); // Array of CornerOperation
console.log(result.solution.edge_operations);   // Array of EdgeOperation
```

## ファイル構成

```
web/
├── index.html          # V1 - シンプル版
├── index-v2.html       # V2 - 構造化データ版（推奨）
├── serve.js           # 簡易HTTPサーバー
├── demo.js            # CLIデモスクリプト
├── package.json       # Node.js設定
├── src/
│   ├── types.ts       # TypeScript型定義
│   └── index.ts       # TypeScriptソース（CLI用）
└── dist/
    └── index.js       # コンパイル済みJS（CLI用）
```

## API リファレンス

### Rust側の構造体

#### CornerSwapOperation
```rust
pub struct CornerSwapOperation {
    pub target1: usize,
    pub target2: usize,
    pub orientation: u8,
}
```

#### CornerTwistOperation
```rust
pub struct CornerTwistOperation {
    pub target: usize,
    pub orientation: u8,
}
```

#### EdgeSwapOperation
```rust
pub struct EdgeSwapOperation {
    pub target1: usize,
    pub target2: usize,
    pub orientation: u8,
}
```

#### EdgeFlipOperation
```rust
pub struct EdgeFlipOperation {
    pub target: usize,
}
```

### TypeScript型

```typescript
type CornerOperation = 
    | { type: 'Swap'; Swap: { target1: number; target2: number; orientation: number } }
    | { type: 'Twist'; Twist: { target: number; orientation: number } };

type EdgeOperation = 
    | { type: 'Swap'; Swap: { target1: number; target2: number; orientation: number } }
    | { type: 'Flip'; Flip: { target: number } };
```

## トラブルシューティング

### WASMファイルが見つからない

```
Error: WebAssembly module not found
```

**解決方法**: プロジェクトルートで以下を実行
```bash
wasm-pack build --target web
```

### V2で新しい関数が見つからない

```
Error: solve_bld_with_default_moveset_v2 is not a function
```

**解決方法**: WASMを再ビルドしてください
```bash
wasm-pack build --target web
```

### ポートが使用中

```
Error: Port 8080 is already in use
```

**解決方法**: 別のポートを指定
```bash
node serve.js 3000
```

## ブラウザ対応

- Chrome/Edge: ✅ 完全対応
- Firefox: ✅ 完全対応
- Safari: ✅ 完全対応
- モバイルブラウザ: ✅ 対応

WebAssemblyをサポートする全てのモダンブラウザで動作します。

## ライセンス

プロジェクトのルートライセンスに従います。

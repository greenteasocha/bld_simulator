# Cross Solver WebUI

ブラウザ上で動作するルービックキューブBLD（目隠し）ソルバーのWebインターフェース。

## 特徴

- 🎨 **モダンなUI**: 使いやすいグラデーションデザイン
- ⚡ **高速**: WebAssemblyによる高速計算
- 🔍 **詳細な解法表示**: Corner/Edge操作と手順を分かりやすく表示
- 📱 **レスポンシブ**: モバイルデバイスにも対応

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
# または
node serve.js 3000
```

起動後、ブラウザで以下にアクセス：
```
http://localhost:8080
```

## 使い方

1. **スクランブル入力**: テキストボックスにスクランブル手順を入力
   - 例: `R U R' U'`
   - 例: `R U2 R' D R U' R' D'`

2. **解くボタン**: ボタンをクリックまたはEnterキーで解法を計算

3. **結果表示**: 以下の情報が表示されます
   - Corner Operations（コーナー操作）
   - Edge Operations（エッジ操作）
   - All Operations（全操作の順序）
   - Move Sequences（実行手順）

## ファイル構成

```
web/
├── index.html          # WebUIのメインHTML
├── serve.js           # 簡易HTTPサーバー
├── demo.js            # CLIデモスクリプト
├── package.json       # Node.js設定
├── src/
│   └── index.ts      # TypeScriptソース（CLI用）
└── dist/
    └── index.js      # コンパイル済みJS（CLI用）
```

## 開発

### CLIデモの実行

```bash
# TypeScriptをコンパイル
npm run build

# デモを実行
npm start

# または
node demo.js
node demo.js "R U2 R' D R U' R' D'"
```

### WebUIの編集

`index.html`を編集後、ブラウザをリロードするだけで変更が反映されます。
サーバーの再起動は不要です。

## トラブルシューティング

### WASMファイルが見つからない

```
Error: WebAssembly module not found
```

**解決方法**: プロジェクトルートで以下を実行
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

### CORS エラー

WASMファイルを読み込む際にCORSエラーが発生する場合は、必ず`serve.js`経由でアクセスしてください。
ファイルシステムから直接HTMLを開く（`file://`プロトコル）とCORSエラーになります。

## ブラウザ対応

- Chrome/Edge: ✅ 完全対応
- Firefox: ✅ 完全対応
- Safari: ✅ 完全対応
- モバイルブラウザ: ✅ 対応

WebAssemblyをサポートする全てのモダンブラウザで動作します。

## ライセンス

プロジェクトのルートライセンスに従います。

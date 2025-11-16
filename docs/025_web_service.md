Rust で作った資産をもとに Web サービスを公開したい。
現状は TUI ツールで、画面表示/ユーザー入力 と コアロジックの部分が分かれているので再利用は可能。
どのような方法がある？クライアント上で独立して動作させたいので、Rust での Web サーバーの形はとらない。もし必要であれば typescript など多言語へのコンバートを考える。



選択肢を教えてRustで作ったコアロジックを活かしてWebサービスを公開する選択肢をいくつか提案します。クライアント側で独立動作させる前提で整理しますね。
1. WebAssembly (Wasm) への変換
最も自然な選択肢です。RustのコアロジックをそのままWebブラウザで動かせます。

wasm-bindgen + wasm-pack: Rust→Wasm→JSのバインディングを自動生成
TypeScriptから呼び出し可能なAPIとして公開
パフォーマンスも良好で、既存のRustコードをほぼそのまま使える
フロントエンドフレームワーク（React, Vue, Svelteなど）と組み合わせ可能

bash# 例
wasm-pack build --target web
メリット: Rustコードの再利用率が高い、高速、型安全性維持
デメリット: ファイルI/Oなど一部の機能は制限される
2. Tauri (デスクトップアプリ経由でWeb公開)

RustバックエンドとWebフロントエンド（HTML/CSS/JS）のハイブリッド
デスクトップアプリとして配布しつつ、将来的にWeb版も検討可能
コアロジックはRustのまま、UIだけWeb技術で構築

メリット: Rustの全機能が使える、デスクトップアプリとしても配布可能
デメリット: 純粋なWebサービスではない（インストールが必要）
3. TypeScriptへの移植
コアロジックをTypeScriptで書き直す選択肢です。
メリット: Web開発の標準スタック、デプロイが容易、エコシステムが豊富
デメリット: 移植コストが高い、パフォーマンス低下の可能性、型安全性がRustより劣る
4. 静的サイトジェネレーター + Wasm

Wasm化したロジックを静的HTMLに組み込み
GitHub Pages、Netlify、Vercelなどで無料ホスティング可能
サーバーレスで完全にクライアント側で動作

メリット: 運用コスト0、高速、シンプル
デメリット: データの永続化が必要な場合は別途検討が必要
推奨アプローチ
TUIツールでロジックが分離されているなら、WebAssembly (wasm-pack) が最適だと思います。理由は：

既存のRustコードをほぼそのまま活用できる
TypeScriptから型安全に呼び出せる
パフォーマンスが高い
段階的な移行が可能（UI部分だけ先にWeb化）

現在のTUIツールの規模や、ファイルI/O・システムコールなどの依存度によって最適な選択肢が変わりますが、まずはWasmで試してみて、制約があればその時に他の選択肢を検討するのが良いでしょう。
どの方向性で進めたいか、もう少し詳しく教えていただければ、具体的な実装方法もお伝えできます！




============================================
ここまで過去の会話記録
ここから指示
============================================



ここから、必要なファイルを参照した上で方針を提示して。このファイルに追記してかまわない

============================================
プロジェクト分析と実装方針
============================================

## 現状の把握

### プロジェクト構造
- **コアロジック**: `src/cube/`, `src/parser/`, `src/inspection/`, `src/explorer/`, `src/workflow/`
  - ルービックキューブの状態管理
  - 手順のパース・展開
  - 3-style データベース読み込み
  - 最適化アルゴリズム（nearby search）
- **TUI部分**: `src/display/`, `src/cli_state_input.rs`, `src/main.rs`
  - ratatui による UI レンダリング
  - キーボード入力処理
  - 視覚化

### 依存関係
- **ratatui**: TUI専用（Web移植不可）
- **serde/serde_json**: Wasm対応可能
- **csv**: Wasm対応可能
- **コアロジック**: 標準ライブラリ中心で Wasm 対応可能

### 分離状況
✅ **良好な分離**: TUIとコアロジックが明確に分離されている
- `lib.rs` でコアモジュールを公開
- TUI部分は `main.rs` のみに集約

## 推奨方針: WebAssembly + TypeScript

### 選択理由
1. ✅ **既存コードの再利用率が最も高い**
   - コアロジック(cube, parser, inspection, explorer, workflow)をそのまま活用
   - TUI部分のみWebフロントエンドで再実装
2. ✅ **パフォーマンス維持**
   - 計算集約的な処理（nearby search, state manipulation）をRustのまま高速実行
3. ✅ **型安全性の継続**
   - wasm-bindgenでTypeScriptの型定義を自動生成
4. ✅ **段階的な移行が可能**
   - まずコアロジックのみWasm化
   - UI実装は既存のTUIロジックを参考に段階的に構築

## 実装計画

### Phase 1: Wasmモジュール作成

#### 1.1 Cargo.toml の修正
```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
```

#### 1.2 Wasmエントリポイント作成
`src/wasm.rs`:
```rust
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct WasmCubeState {
    inner: crate::RubiksCube,
}

#[wasm_bindgen]
impl WasmCubeState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: crate::RubiksCube::solved(),
        }
    }
    
    #[wasm_bindgen]
    pub fn apply_scramble(&mut self, scramble: &str) -> Result<(), JsValue> {
        // パーサーを使用してスクランブル適用
        Ok(())
    }
    
    #[wasm_bindgen]
    pub fn get_state(&self) -> JsValue {
        // 状態をJSONでシリアライズ
        serde_wasm_bindgen::to_value(&self.inner.state()).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn search_solutions(&self) -> JsValue {
        // nearby search実行
        // 結果をJSONで返す
        JsValue::NULL // TODO: 実装
    }
}
```

#### 1.3 ビルド設定
```bash
wasm-pack build --target web --out-dir web/pkg
```

### Phase 2: TypeScript/React フロントエンド

#### 2.1 プロジェクト構造
```
web/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── src/
│   ├── App.tsx           # メインアプリ
│   ├── components/
│   │   ├── CubeVisualization.tsx   # 3D cube表示
│   │   ├── StateEditor.tsx         # CP/CO/EP/EO編集
│   │   ├── ScrambleInput.tsx       # スクランブル入力
│   │   └── ResultsPanel.tsx        # 検索結果表示
│   ├── hooks/
│   │   ├── useCubeState.ts         # Wasmとの状態管理
│   │   └── useSolutionSearch.ts    # 検索処理
│   └── wasm/
│       └── index.ts                # Wasm初期化
└── public/
    └── resources/                  # CSV/JSONデータ
```

#### 2.2 技術スタック
- **ビルドツール**: Vite（高速、Wasm対応良好）
- **UIフレームワーク**: React 18 + TypeScript
- **3D可視化**: Three.js or react-three-fiber
- **スタイリング**: Tailwind CSS
- **状態管理**: Zustand (軽量)
- **デプロイ**: Vercel / Netlify / GitHub Pages

#### 2.3 Wasm統合例
```typescript
// src/wasm/index.ts
import init, { WasmCubeState } from '../../pkg';

let wasmModule: typeof import('../../pkg') | null = null;

export async function initWasm() {
  if (!wasmModule) {
    wasmModule = await init();
  }
  return wasmModule;
}

export { WasmCubeState };

// src/hooks/useCubeState.ts
import { useState, useEffect } from 'react';
import { initWasm, WasmCubeState } from '../wasm';

export function useCubeState() {
  const [cube, setCube] = useState<WasmCubeState | null>(null);
  
  useEffect(() => {
    initWasm().then(() => {
      setCube(new WasmCubeState());
    });
  }, []);
  
  const applyScramble = (scramble: string) => {
    cube?.apply_scramble(scramble);
    // 再レンダリングトリガー
  };
  
  return { cube, applyScramble };
}
```

### Phase 3: 静的ホスティング

#### 3.1 GitHub Pages設定
```yaml
# .github/workflows/deploy.yml
name: Deploy to GitHub Pages
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - run: cargo install wasm-pack
      - run: wasm-pack build --target web
      - uses: actions/setup-node@v3
      - run: cd web && npm install && npm run build
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./web/dist
```

## 移行ステップ

### ステップ1: 最小限のWasm化（1-2日）
- [ ] `wasm-bindgen` 依存追加
- [ ] 基本的な `WasmCubeState` 実装
- [ ] 状態取得・変更のAPI作成
- [ ] wasm-packでビルド成功

### ステップ2: シンプルなWeb UI（3-5日）
- [ ] Vite + React プロジェクト作成
- [ ] Wasm初期化とロード
- [ ] スクランブル入力フォーム
- [ ] 2D cube表示（簡易版）
- [ ] 基本的な状態編集UI

### ステップ3: フル機能実装（1-2週間）
- [ ] 3D cube可視化
- [ ] TUIと同等の状態編集機能
- [ ] Nearby search統合
- [ ] 結果表示パネル
- [ ] CSVデータ読み込み

### ステップ4: デプロイ＆最適化（2-3日）
- [ ] GitHub Actions CI/CD
- [ ] パフォーマンスチューニング
- [ ] PWA化（オフライン対応）
- [ ] ドキュメント整備

## 代替案の比較

### A. TypeScript完全移植
- ❌ 移植コスト高（全コードを書き直し）
- ❌ パフォーマンス低下の可能性
- ✅ エコシステムの恩恵

### B. Tauri
- ✅ Rustコード全てそのまま
- ❌ デスクトップアプリ（Webブラウザ不可）
- ❌ インストール必要

### C. Wasm + TypeScript（推奨）
- ✅ コア再利用
- ✅ パフォーマンス維持
- ✅ Webネイティブ
- ✅ デプロイ簡単

## リソース参照

### 公式ドキュメント
- wasm-bindgen: https://rustwasm.github.io/wasm-bindgen/
- wasm-pack: https://rustwasm.github.io/wasm-pack/
- Rust and WebAssembly Book: https://rustwasm.github.io/book/

### 類似プロジェクト例
- Lichess (Rust+Wasm chess engine): https://github.com/lichess-org/lila
- egui_demo_app (Rust GUI in browser): https://github.com/emilk/egui

## 次のアクション

優先度順:
1. **Phase 1 の実装開始** - Wasmビルドを動かす
2. **最小限のUIプロトタイプ** - スクランブル適用＋表示
3. **段階的な機能追加** - TUIの機能を1つずつ移植

質問がある場合:
- Wasm APIの設計について
- 特定の機能の実装方法
- デプロイ戦略の詳細


=============================================

Phase 1だが、wasm やフロントエンドについて全く知識がないので流れを教えてほしい

- rust を wasm にビルドすると何が生成される？
- typescript からはどう利用する？
- フロントエンドのプロジェクト構造はどうなる？
- プロジェクトは同居できる？
- ビルドやデプロイはどうする？

=============================================
Phase 1: Wasm完全ガイド（初心者向け）
=============================================

## 🎯 全体の流れ

```
[Rust コード]  →  wasm-pack  →  [Wasmバイナリ + JSグルーコード]  →  [TypeScript/React アプリ]
     ↓                              ↓                                    ↓
  src/lib.rs                    pkg/ ディレクトリ                   web/src/ ディレクトリ
```

---

## 📦 Q1: Rustをwasmにビルドすると何が生成される？

### コマンド実行
```bash
wasm-pack build --target web --out-dir pkg
```

### 生成されるファイル
```
pkg/
├── bld_simulator.d.ts          # TypeScript型定義ファイル
├── bld_simulator.js            # JavaScriptグルーコード（wasmとJSの橋渡し）
├── bld_simulator_bg.wasm       # 実際のWebAssemblyバイナリ（バイナリファイル）
├── bld_simulator_bg.wasm.d.ts  # wasm用の型定義
└── package.json                # npmパッケージの情報
```

### 各ファイルの役割

#### 1. `bld_simulator_bg.wasm` （メインの成果物）
- Rustコードがコンパイルされたバイナリ形式
- ブラウザが直接実行できる低レベルコード
- サイズは数百KB～数MB（最適化により変動）
- **これがRustで書いたロジックの本体**

#### 2. `bld_simulator.js` （JSグルーコード）
```javascript
// 自動生成される例（簡略版）
export function init() {
  // wasmファイルをロードして初期化
  return fetch('bld_simulator_bg.wasm')
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes))
}

export class WasmCubeState {
  constructor() {
    // wasmの関数を呼び出してRustのオブジェクトを作成
  }
  apply_scramble(scramble) {
    // wasmの関数を呼び出し
  }
}
```
- **JavaScriptからwasmを使いやすくするラッパー**
- メモリ管理、型変換、関数呼び出しを自動処理

#### 3. `bld_simulator.d.ts` （型定義）
```typescript
export function init(): Promise<void>;

export class WasmCubeState {
  constructor();
  apply_scramble(scramble: string): void;
  get_state(): any;
  free(): void;  // メモリ解放用
}
```
- **TypeScriptでコード補完と型チェックを可能にする**
- IDEが関数のシグネチャを理解できる

---

## 💻 Q2: TypeScriptからはどう利用する？

### ステップ1: wasmの初期化
```typescript
// web/src/wasm/index.ts
import init, { WasmCubeState } from '../../../pkg/bld_simulator';

let initialized = false;

export async function initWasm() {
  if (!initialized) {
    await init();  // wasmファイルをロード＆初期化
    initialized = true;
  }
}

export { WasmCubeState };
```

### ステップ2: Reactコンポーネントで使用
```typescript
// web/src/App.tsx
import { useEffect, useState } from 'react';
import { initWasm, WasmCubeState } from './wasm';

function App() {
  const [cube, setCube] = useState<WasmCubeState | null>(null);
  const [state, setState] = useState<any>(null);

  // 初回レンダリング時にwasmを初期化
  useEffect(() => {
    initWasm().then(() => {
      const newCube = new WasmCubeState();  // Rustのオブジェクト作成
      setCube(newCube);
      setState(newCube.get_state());  // 状態取得
    });
  }, []);

  const handleScramble = (scramble: string) => {
    if (cube) {
      cube.apply_scramble(scramble);  // Rustの関数を呼び出し
      setState(cube.get_state());     // 更新された状態を取得
    }
  };

  return (
    <div>
      <input type="text" onBlur={(e) => handleScramble(e.target.value)} />
      <pre>{JSON.stringify(state, null, 2)}</pre>
    </div>
  );
}
```

### データのやり取り
```
[TypeScript]                [Wasm境界]              [Rust]
   ↓                             ↓                     ↓
string "R U R'"  →  JSグルーコード  →  &str として受け取り
   ↑                             ↑                     ↑
JSON object      ←  JSグルーコード  ←  serde_json::Value
```

---

## 🏗️ Q3: フロントエンドのプロジェクト構造はどうなる？

### 理想的なディレクトリ構造
```
cross-solver/                    # Rustプロジェクトルート
├── Cargo.toml                   # Rustの依存関係
├── src/
│   ├── lib.rs                   # Rustのライブラリエントリポイント
│   ├── wasm.rs                  # Wasm用API定義（新規作成）
│   ├── cube/
│   ├── parser/
│   └── ...
├── pkg/                         # wasm-pack の出力先（gitignore）
│   ├── bld_simulator.js
│   ├── bld_simulator_bg.wasm
│   └── ...
└── web/                         # フロントエンドプロジェクト（新規作成）
    ├── package.json             # npm依存関係
    ├── tsconfig.json            # TypeScript設定
    ├── vite.config.ts           # Viteビルド設定
    ├── index.html               # エントリHTML
    ├── src/
    │   ├── main.tsx             # Reactエントリポイント
    │   ├── App.tsx              # メインコンポーネント
    │   ├── wasm/
    │   │   └── index.ts         # Wasm初期化ロジック
    │   ├── components/
    │   │   ├── CubeVisualization.tsx
    │   │   ├── StateEditor.tsx
    │   │   └── ScrambleInput.tsx
    │   └── hooks/
    │       └── useCubeState.ts
    └── public/
        └── resources/           # CSV/JSONファイル
```

### 各ディレクトリの責務

#### `src/` (Rust)
- **役割**: コアロジックの実装
- **言語**: Rust
- **変更頻度**: 低（既存コードはほぼそのまま）

#### `pkg/` (生成物)
- **役割**: wasmビルドの出力先
- **管理**: `.gitignore` に追加（自動生成されるため）
- **使用**: `web/` からimportして使用

#### `web/` (フロントエンド)
- **役割**: UI実装
- **言語**: TypeScript + React
- **変更頻度**: 高（UI開発のメイン作業場所）

---

## 🤝 Q4: プロジェクトは同居できる？

### ✅ 同居可能（推奨）

同じリポジトリ内にRustとフロントエンドを配置するのが一般的です。

### メリット
1. **バージョン管理が容易**: コアロジックとUIが同期
2. **CI/CDが簡単**: 1つのリポジトリで完結
3. **開発効率**: コードジャンプが楽

### `.gitignore` の設定
```gitignore
# Rust
/target/
Cargo.lock

# Wasm出力
/pkg/

# Node/Frontend
/web/node_modules/
/web/dist/
/web/.vite/
```

### `Cargo.toml` の設定
```toml
[package]
name = "bld_simulator"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]  # cdylib: wasm用, rlib: Rust用

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
ratatui = { version = "0.28", features = ["crossterm"] }  # TUI用（wasm以外）
csv = "1.1"
```

### `web/package.json` の設定
```json
{
  "name": "bld-simulator-web",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "react": "^18.3.0",
    "react-dom": "^18.3.0"
  },
  "devDependencies": {
    "@types/react": "^18.3.0",
    "@types/react-dom": "^18.3.0",
    "@vitejs/plugin-react": "^4.3.0",
    "typescript": "^5.6.0",
    "vite": "^5.4.0",
    "vite-plugin-wasm": "^3.3.0"
  }
}
```

---

## 🔨 Q5: ビルドやデプロイはどうする？

### ローカル開発の流れ

#### 1. Wasmビルド（Rustコード変更時のみ）
```bash
# プロジェクトルートで実行
wasm-pack build --target web --out-dir pkg

# 初回のみ wasm-pack のインストール
cargo install wasm-pack
```

#### 2. フロントエンド開発サーバー起動
```bash
cd web
npm install        # 初回のみ
npm run dev        # 開発サーバー起動（http://localhost:5173）
```

#### 3. 変更時の再ビルド
```bash
# Rustコード変更 → wasmビルド必要
wasm-pack build --target web --out-dir pkg

# TypeScript/React変更 → Viteが自動リロード（再ビルド不要）
```

### 本番ビルド
```bash
# 1. Wasmビルド（最適化あり）
wasm-pack build --target web --out-dir pkg --release

# 2. フロントエンドビルド
cd web
npm run build  # web/dist/ に静的ファイル生成
```

### デプロイ（GitHub Pagesの例）

#### 手動デプロイ
```bash
# ビルド
wasm-pack build --target web --out-dir pkg --release
cd web && npm run build

# GitHub Pagesへデプロイ
npx gh-pages -d web/dist
```

#### 自動デプロイ（GitHub Actions）
```yaml
# .github/workflows/deploy.yml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      # リポジトリをチェックアウト
      - uses: actions/checkout@v4
      
      # Rustをセットアップ
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      
      # wasm-packをインストール
      - run: cargo install wasm-pack
      
      # Wasmビルド
      - run: wasm-pack build --target web --out-dir pkg --release
      
      # Node.jsをセットアップ
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      # フロントエンドビルド
      - run: cd web && npm ci && npm run build
      
      # GitHub Pagesへデプロイ
      - uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./web/dist
```

### デプロイ先の選択肢

#### GitHub Pages（無料）
- **URL**: `https://username.github.io/repo-name/`
- **設定**: リポジトリ設定でPages有効化
- **制約**: 静的サイトのみ、500MB以下

#### Vercel（無料枠あり）
- **URL**: `https://project-name.vercel.app`
- **設定**: GitHubリポジトリと連携
- **メリット**: 自動デプロイ、CDN最適化

#### Netlify（無料枠あり）
- **URL**: `https://project-name.netlify.app`
- **設定**: ドラッグ＆ドロップでデプロイ可能
- **メリット**: フォーム処理、リダイレクト設定

---

## 🚀 実践：最初の一歩

### ステップバイステップガイド

#### 1. wasm-pack インストール
```bash
cargo install wasm-pack
```

#### 2. `Cargo.toml` 修正
```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
```

#### 3. `src/wasm.rs` 作成（最小限）
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

#### 4. `src/lib.rs` に追加
```rust
#[cfg(target_arch = "wasm32")]
pub mod wasm;
```

#### 5. wasmビルド
```bash
wasm-pack build --target web --out-dir pkg
```

#### 6. フロントエンド作成
```bash
npm create vite@latest web -- --template react-ts
cd web
npm install
npm install vite-plugin-wasm vite-plugin-top-level-await
```

#### 7. `web/vite.config.ts` 設定
```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
})
```

#### 8. `web/src/App.tsx` でテスト
```typescript
import { useEffect, useState } from 'react'
import init, { greet } from '../../pkg/bld_simulator'

function App() {
  const [message, setMessage] = useState('')

  useEffect(() => {
    init().then(() => {
      setMessage(greet('Wasm'))
    })
  }, [])

  return <div>{message}</div>
}

export default App
```

#### 9. 開発サーバー起動
```bash
cd web
npm run dev
# → http://localhost:5173 を開く
# → "Hello, Wasm!" が表示されれば成功！
```

---

## 📚 まとめ

### ビルドフロー
```
Rustコード変更
   ↓
wasm-pack build  →  pkg/ に wasm + JS 生成
   ↓
TypeScript import
   ↓
npm run dev  →  ブラウザで実行
```

### ファイルの流れ
```
src/lib.rs  →  [wasm-pack]  →  pkg/bld_simulator.js
                                pkg/bld_simulator_bg.wasm
                                     ↓
                            web/src/wasm/index.ts で import
                                     ↓
                            web/src/App.tsx で使用
                                     ↓
                            [Vite]  →  ブラウザ
```

### 開発時の心得
- **Rust変更時**: `wasm-pack build` 必須
- **TS/React変更時**: Viteが自動リロード
- **デバッグ**: ブラウザのDevToolsでconsole.log
- **型定義**: `.d.ts` ファイルが自動生成されるのでIDEが補完してくれる

次は実際にコードを書いてみましょう！
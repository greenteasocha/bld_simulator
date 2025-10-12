Vec<CornerOperation> から Vec<Move> への変換関数を実装したい。

まず Swap の変換を考える。
Swap の変換方法は2通りある。
連続する2つの Swap を一連の Move に変換する方法と、1つのSwapを一連の Move に変換する方法である。


# 2つの Swap を Move に変換する方法
はじめに Swap 2つを Move に変換する方法を考える。
それぞれの Swap について、TARGET_STICKER を定義する。
```
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
表の中から、SwapOperation の target2: orientation を用いる。
例えば、Swap: UFR ↔ DBR (ori: 2) ならば、target2=5, orientation=2 なので、TARGET_STICKERS[5][2] = "BDR" となる。

これにより 2つの TARGET_STICKER を得ることができるので、resources/ufr_expanded.json から値を取得する。

例えば 
- Swap: UFR ↔ DBR (ori: 2)
- Swap: UFR ↔ DFR (ori: 1)
からは TARGET_STICKER BDR および RDF が得られるため、JSON からは
 BDR → RDF の値である D' R U R' D R U' R' を取得できる。
二つの Target の順番は変えてはいけないことに注意する。

# 1つの Swap を Move に変換する方法
次に 1つの Swap を Move に変換する方法を考える。

TARGET_STICKER を1つ得られるので、それをキーにして ufr_parity.json から値を取得する。
例えば Swap: UFR ↔ DBR (ori: 2) ならば TARGET_STICKER は BDR なので、JSON からは BDR の値である U2 D' R' F R2 U' R' U' R U R' F' R U R' U D を取得できる。

# Twist の変換

```
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
Twist は 1つの Twist → Move の変換しかない。
TwistOperation の target: orientation を用いて、TARGET_STICKERS から TARGET_STICKER を得る。
json は　ufr_twist.json から取得する。

例えば Twist: UFL (counter-clockwise) ならば target=3, orientation=1 なので、TARGET_STICKERS[3][1] = "FUL" となり、最終的に得られる Move 列は R' D R D' R' D R U' R' D' R D R' D' R U となる。


# 全体の変換
これらを組み合わせて、Vec<CornerOperation> の要素を先頭から操作し、次の優先度で変換を試みる。
1. 連続する2つの Swap を Move に変換
2. 1つの Swap を Move に変換
3. 1つの Twist を Move に変換
一つのステップで変換された Vec<Move> は、MoveSequence という型として扱う。
変換結果は出力用の Vec<MoveSequence> に追加される。


# 実装箇所
実装は src/inspection ディレクトリに新しいファイルを作成してください。
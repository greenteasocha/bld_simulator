nearby_search の拡張を考えます。

現状の実装では与えられた操作列のうちいずれか一つのみを任意の異なる操作に変更したバリエーションを出力しています。
これを任意の2つの操作を異なる操作に変更したバリエーションも出力できるようにします。

例えば与えられた操作列
Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ 6 (ori: 0)
の場合、出力として

Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ **3** (ori: 0)
Step 3: Swap: 0 ↔ **5** (ori: **1**)

等が存在します。
target のみが2つ変わる、orientation のみが2つ変わる、target と orientation がそれぞれ1つずつ変わる、等のパターンも存在することに注意してください。

基本的にある一つの操作を変更する際の選択肢は今の実装を踏襲し、for loop による探索の枠組みのみを変更して2段階の分岐を可能にしてください。

まずは実装を行わず、どのような方針があるかを提示してください。


===============================================

追記

2回の for loop を用いる方法では、Edge,Cornerともに「Twist または Flipの操作は Operation で置き換えない」というルールが無視されています。
これを修正してください。

また、追加の機能として、Twist/Flip の場合は異なる Twist/Flip に置き換えるというルールも追加してください。
Operation の置き換えと Twist/Flip の置き換えは別々に扱い、2つの変更のうち1つが Operation の置き換え、もう1つが Twist/Flip の置き換え、というパターンも含めてください。

例:
Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ 6 (ori: 0)
Step 4: Twist: 2 (Clockwise)
↓
Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ **3** (ori: 0)
Step 4: Twist: **4** (**Counter-Clockwise**)


===============================================

追記2

Swap と Twist/Flip を別々に処理する必要はありません。
Swap とTwist/Flipを区別しない操作列の長さをNとしたとき、i,j ... N について、
Step i および j が Swap なのか Twist/Flip なのかを Match で判断し、可能性のある置き換え候補をすべて試すという方法をとってください。

Swap の置き換え候補は generate_alternatives で提供していますが、Twist/Flip の置き換え候補はまだ提供していないので、ここに書きます。

Twist
```
let mut alternatives = Vec::new();

        for target in 0..8 {
            for orientation in 1..3 {
                alternatives.push(CornerTwistOperation::new(target2, orientation));
            }
        }

        alternatives
```

Flip
```
let mut alternatives = Vec::new();

        for target in 0..12 {
            alternatives.push(EdgeFlipOperation::new(target));
        }

        alternatives
```



===============================================

追記3

rubiks_cube_simulator/src/explorer/wrong_operation_detector.rs で先ほど作成した2つの操作変更を検出するロジックを利用してください。


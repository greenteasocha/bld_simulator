# 概要
bld_inspection.md でシミュレーションした2点交換を、より汎用性高く使えるように独立した操作として定義する。

# 構造体定義
cornerSwap
{
    target1
    target2
    orientation
}

cornerTwist
{
    target
    orientation
}

両方の構造体とも、メソッドとして引数、帰り値がともに state である apply を持つ。
また、それぞれの構造体は std::fmt::Display を持つ。
CornerOperations の std::fmt::Display は、それぞれを呼び出す形にする。


# cornerswap.apply




# bld_inspection の改善点
solve_corner_permutation_with_orientation では、完成までのCornerOperationsを返す。
引数の State は更新しない。

テストでは初期の state に対して、 solver から受け取った operation をそれぞれ applu したものが完成状態と同じかどうかをテストする
/mnt/c/wslhome/projects/cross-solver/rubiks_cube_simulator/docs/bld_exec_simulation.md



# NearbyOperationSearch の実装

## 概要
与えられた State と、その解法となる Operations から途中で1手だけ別のものにした動きをシミュレーションします。例を与えます

正しい解法が
Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ 6 (ori: 0)
の場合、出力として

Step 1: Swap: 0 ↔ **3** (ori: 0)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ 6 (ori: 0)

Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ **5** (ori: 0)
Step 3: Swap: 0 ↔ 6 (ori: 0)

Step 1: Swap: 0 ↔ 1 (ori: 0)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ **2** (ori: 0)
等があります。target2 のみに変更の可能性があることに注意してください。

また、orientation のみの変更もあり得ることに注意してください
Step 1: Swap: 0 ↔ 1 (ori: **1**)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ 6 (ori: 0)

単一の Step で target と orientation が同時に代わる可能性があることにも注意してください
Step 1: Swap: 0 ↔ **3** (ori: **1**)
Step 2: Swap: 0 ↔ 2 (ori: 0)
Step 3: Swap: 0 ↔ 6 (ori: 0)

## 処理詳細

i = 0...len(operations) について、以下の操作を行います。
operations[i] が twist なら、スキップします

operations[i] が swap の場合、operations[i] を以下の alternatives と交換した operations を全て出力します

alternatives = []
for i = 0..7, j = 0..2 
alternatives.push(Swap(target1: 0, target2: i, co:j))


# usecase の作成
完成状態にするつもりが想定せず近傍の state にたどり着いてしまったユーザーに、どの操作列で近傍の state にたどり着いてしまったのかを提示します。

- 初期の initial_state を与えられてそれに対する解法 operations を計算
- 解法の近傍 operations を列挙
- 誤りの state (wrongly_solved_state) を受け取る
- initial_state に適用して wrongly_solved_state に到達する operation 画れば提示

表示方法
```
Initial State: {initial_state}
Collect solution: {operation}

Did you applied {wrong_operation}?
```
現在 corner と edge に独立して実装されている nearby search を、両方を含む操作列に対して適用できるに混合版として拡張してください。

## 混合 nearby search の実装

既存の nearby_search とは別のファイルに実装してください。
大まかな枠組みとしては同じで、与えられた操作列のうち最大2つの操作を異なる操作に変更したバリエーションをすべて出力します。

引数は操作列とします。これには
- Swap (edge)
- Flip (edge)
- Swap (corner)
- Twist (corner)
の操作が含まれます。

それぞれの操作については、同種の操作に置き換えることができます。
- Swap(edge) は別の Swap(edge) に置き換え可能
- Twist(corner) は別の Twist(corner) に置き換え可能
... などです。

それぞれの種類についてどのような置き換えが可能かは既存の nearby_search の実装を参考にしてください。


## 混合 nearby search の使用
新しい wolkflow として混合 nearby search を呼び出すものを追加してください。

また、rubiks_cube_simulator/examples/bld_workflow_example.rs からその wolkflow を呼び出すコードを追加してください。
rubiks_cube_simulator/examples/bld_workflow_example_with_nearby_search.rs という名前で新規作成してください。

1: スクランブルに対する正しい操作列の出力
cp: [0, 1, 7, 3, 4, 5, 2, 6],
co: [0, 0, 1, 0, 0, 0, 2, 0],
ep: [0, 1, 2, 3, 4, 7, 5, 6, 8, 9, 10, 11],
eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],

に対する正しい操作列を出力

2. 正しい操作列からの分岐のうち特定の state にたどり着くものの出力
cp: [0, 1, 2, 3, 4, 5, 6, 7],
co: [0, 0, 0, 0, 0, 0, 0, 0],
ep: [0, 1, 2, 3, 4, 5, 11, 6, 8, 9, 10, 7],
eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],

にある分岐が存在するか探索

の流れで実装してください。
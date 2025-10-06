impl std::fmt::Display for CornerSwapOperation を修正する。

target sticker を以下のように定める

target_sticker = 
[
    0 => ["UBL", "BUL", "LUB"],
    1 => ["UBR", "RUB", "BUR"],
    2 => ["UFR", "FUR", "RUF"],
    3 => ["UFL", "LUF", "FUL"],
    4 => ["DBL", "LDB", "BDL"],
    5 => ["DBR", "BDR", "RDB"],
    6 => ["DFR", "RDF", "FDR"],
    7 => ["DFL", "FDL", "LDF"],
]

最終的な表示は
```
Swap: UBL ↔ {target_sticker[CornerTwistOperation.target2][j CornerTwistOperation.test_with_orientation]}
```
とする。例: "Swap: UBL ↔ RUF"






また、impl std::fmt::Display for CornerSwapOperation を修正する。
target_sticker = {
    UBL, UBR, UFR, UFL,DBL, DBR, DFR, DFL
}

target_rotation_direction = {
    0 => "noop",    // 回転なし
    1 => "clockwise", // 時計回り
    2 => "counter-clockwise", // 反時計回り
}
    

最終的な表示は
```
Twist: target_sticker[CornerTwistOperation.target] (target_rotation_direction[CornerTwistOperation.orientation])
```
とする。例: "Twist: UBL (clockwise)"


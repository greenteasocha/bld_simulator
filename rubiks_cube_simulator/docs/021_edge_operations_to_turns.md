rubiks_cube_simulator/src/inspection/operations_to_turns.rs を edge 煮も拡張する。

- 2つの Swap 操作を回転の列に変換する
- 1つの Flip 操作を回転の列に変換する

Swap からの変換は
rubiks_cube_simulator/resources/uf.json　
を読み込んで参照する。

json のキーは target と flip を用いて以下のテーブルから決める
[
   ["BL","LB"],
   ["BR","RB"],
   ["FR","RF"],
   ["FL","LF"],
   ["UB","BU"],
   ["UR","RU"],
   ["UF","FU"],
   ["UL","LU"],
   ["DB","BD"],
   ["DR","RD"],
   ["DF","FD"],
   ["DL","LD"],
]

Flip からの変換は
rubiks_cube_simulator/resources/uf_flip.json
を読み込んで参照する。  

json のキーは target を用いて以下のテーブルから決める
[
    "BL",
    "BR",
    "FR",
    "FL",
    "UB",
    "UR",
    "UF",
    "UL",
    "DB",
    "DR",
    "DF",
    "DL"
]

あとはコーナーと似たようにやる。
1つの Swap からの変換は存在しない。
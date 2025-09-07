キューブの各面の色を示す構造体の定義を考える。

struct cube_stickers

cube_stickers has edges and corners

corners composed by 6 corner_face (Up, Down, Left, Right, Front, Back)
each corner_face composed by 4 corner_stircer (Upper-right, Upper-left, lower_right, lower_left)
(Caution, corner_sticker is defined independently from the name of Face. 
e.g. Down face sticker of DBR piece is cube_stickers -> corner_face::Down -> Corner_Sticker::Upper-Right)

edges composed by 6 edge_face
each edge_face composed by 4 edge_faces_stircer

edge_face and corner_face has a property "color"




次に、state から各面各ステッカーの色を決めるような変換手続きについて考える

initialize all cube colors as void

adapt corner_procesure for every corner state (permutation and orientation)

each corner state has "original_piece" and "affection_target_sticker"

original_piece will be calculated by state, and affection_target_sticker is fixed all time.

mapping of original_piece is here.
```
match corner_permutation[i]

            0 => [CubeColor::White, CubeColor::Blue, CubeColor::Orange], // UBL
            1 => [CubeColor::White, CubeColor::Red, CubeColor::Blue],    // UBR
            2 => [CubeColor::White, CubeColor::Green, CubeColor::Red],   // UFR
            3 => [CubeColor::White, CubeColor::Orange, CubeColor::Green], // UFL
            4 => [CubeColor::Yellow, CubeColor::Orange, CubeColor::Blue], // DBL
            5 => [CubeColor::Yellow, CubeColor::Blue, CubeColor::Red],   // DBR
            6 => [CubeColor::Yellow, CubeColor::Red, CubeColor::Green],  // DFR
            7 => [CubeColor::Yellow, CubeColor::Green, CubeColor::Orange], // DFL
```

Next, piece should be rotated in terms of its value of corner_orientation of state.

Given original piece as below.
[CubeColor::White, CubeColor::Blue, CubeColor::Orange]

Rotating will proceed like:
[CubeColor::White, CubeColor::Blue, CubeColor::Orange]
=> [CubeColor::Orange, CubeColor::White, CubeColor::Blue]
=> [CubeColor::Blue, CubeColor::Orange, CubeColor::White]
=> [CubeColor::White, CubeColor::Blue, CubeColor::Orange]
...

In general, "rotated_original_piece" is defined as:
```
o = corner_orientation[i]
[original_piece[(3-o)%3], original_piece[(4-o) % 3], original_piece[(2-o)]]
(o shuold be 0,1 or 2)

```


On the other hand, mapping of affection_target_sticker is like this 
```
match [i]

            0 => [{CornerFace::Up, CornerSticker::UpperLeft},{CornerFace::Left, CornerSticker::UpperLeft},{CornerFace::Back, CornerSticker::UpperRight}], // UBL
            1 => [{CornerFace::Up, CornerSticker::UpperRight},{CornerFace::Back, CornerSticker::UpperLeft},{CornerFace::Right, CornerSticker::UpperRight}], // UBR
            ...
```

Then, overright target part of cube_sticker, which should be void befor overrided.
Each corner piece has 3 target. Target and value to override are defined in the former steps.


Show an example in situation that:
rotated_original_piece ... [CubeColor::Blue, CubeColor::Orange, CubeColor::White]
affection_target_sticker ... [{CornerFace::Up, CornerSticker::UpperLeft},{CornerFace::Left, CornerSticker::UpperLeft},{CornerFace::Back, CornerSticker::UpperRight}]

Then,
Paint Cube -> Up -> UpperLeft in Blue
Paint Cube -> Left -> UpperLeft in Orange
Paint Cube -> Back -> UpperRight in White


That is all for a piece.
Paint all corner is done by the steps in enumerate i over len(corner_state).



Process of painting edges is similar as corner. Using differnt mappings.
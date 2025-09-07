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
match [i] {
    // 上面のコーナー (時計回り)
    0 => [
        {CornerFace::Up, CornerSticker::UpperLeft},
        {CornerFace::Back, CornerSticker::UpperRight}, 
        {CornerFace::Left, CornerSticker::UpperLeft}
    ], // UBL
    
    1 => [
        {CornerFace::Up, CornerSticker::UpperRight},
        {CornerFace::Right, CornerSticker::UpperRight},
        {CornerFace::Back, CornerSticker::UpperLeft}
    ], // UBR
    
    2 => [
        {CornerFace::Up, CornerSticker::LowerRight},
        {CornerFace::Front, CornerSticker::UpperRight},
        {CornerFace::Right, CornerSticker::UpperLeft}
    ], // UFR
    
    3 => [
        {CornerFace::Up, CornerSticker::LowerLeft},
        {CornerFace::Left, CornerSticker::UpperRight},
        {CornerFace::Front, CornerSticker::UpperLeft}
    ], // UFL
    
    // 下面のコーナー (時計回り)
    4 => [
        {CornerFace::Down, CornerSticker::UpperLeft},
        {CornerFace::Left, CornerSticker::LowerLeft},
        {CornerFace::Back, CornerSticker::LowerRight}
    ], // DBL
    
    5 => [
        {CornerFace::Down, CornerSticker::UpperRight},
        {CornerFace::Back, CornerSticker::LowerLeft},
        {CornerFace::Right, CornerSticker::LowerRight}
    ], // DBR
    
    6 => [
        {CornerFace::Down, CornerSticker::LowerRight},
        {CornerFace::Right, CornerSticker::LowerLeft},
        {CornerFace::Front, CornerSticker::LowerRight}
    ], // DFR
    
    7 => [
        {CornerFace::Down, CornerSticker::LowerLeft},
        {CornerFace::Front, CornerSticker::LowerLeft},
        {CornerFace::Left, CornerSticker::LowerRight}
    ], // DFL
}
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



Process of painting edges is similar as corner. Using different mappings.

Edge affection_target_sticker mapping:
```
match [i] {
    // 中間層のエッジ
    0 => [
        {EdgeFace::Back, EdgeSticker::Right},
        {EdgeFace::Left, EdgeSticker::Left}
    ], // BL 
    
    1 => [
        {EdgeFace::Back, EdgeSticker::Left},
        {EdgeFace::Right, EdgeSticker::Right}
    ], // BR 
    
    2 => [
        {EdgeFace::Front, EdgeSticker::Right},
        {EdgeFace::Right, EdgeSticker::Left}
    ], // FR
    
    3 => [
        {EdgeFace::Front, EdgeSticker::Left},
        {EdgeFace::Left, EdgeSticker::Right}
    ], // FL
    
    // 上面のエッジ (時計回り)
    4 => [
        {EdgeFace::Up, EdgeSticker::Upper},
        {EdgeFace::Back, EdgeSticker::Upper}
    ], // UB
    
    5 => [
        {EdgeFace::Up, EdgeSticker::Right},
        {EdgeFace::Right, EdgeSticker::Upper}
    ], // UR
    
    6 => [
        {EdgeFace::Up, EdgeSticker::Lower},
        {EdgeFace::Front, EdgeSticker::Upper}
    ], // UF
    
    7 => [
        {EdgeFace::Up, EdgeSticker::Left},
        {EdgeFace::Left, EdgeSticker::Upper}
    ], // UL
    
    // 下面のエッジ (時計回り)
    8 => [
        {EdgeFace::Down, EdgeSticker::Upper},
        {EdgeFace::Back, EdgeSticker::Lower}
    ], // DB
    
    9 => [
        {EdgeFace::Down, EdgeSticker::Right},
        {EdgeFace::Right, EdgeSticker::Lower}
    ], // DR
    
    10 => [
        {EdgeFace::Down, EdgeSticker::Lower},
        {EdgeFace::Front, EdgeSticker::Lower}
    ], // DF
    
    11 => [
        {EdgeFace::Down, EdgeSticker::Left},
        {EdgeFace::Left, EdgeSticker::Lower}
    ], // DL
}
```

Edge original_piece mapping:
```
match edge_permutation[i] {
    0 => [CubeColor::Blue, CubeColor::Orange],    // BL 
    1 => [CubeColor::Blue, CubeColor::Red],       // BR 
    2 => [CubeColor::Green, CubeColor::Red],      // FR
    3 => [CubeColor::Green, CubeColor::Orange],   // FL
    4 => [CubeColor::White, CubeColor::Blue],     // UB
    5 => [CubeColor::White, CubeColor::Red],      // UR
    6 => [CubeColor::White, CubeColor::Green],    // UF
    7 => [CubeColor::White, CubeColor::Orange],   // UL
    8 => [CubeColor::Yellow, CubeColor::Blue],    // DB
    9 => [CubeColor::Yellow, CubeColor::Red],     // DR
    10 => [CubeColor::Yellow, CubeColor::Green],  // DF
    11 => [CubeColor::Yellow, CubeColor::Orange], // DL
}
```

Edge orientation rotation:
```
o = edge_orientation[i]
if o == 1 {
    [original_piece[1], original_piece[0]]  // flip
} else {
    original_piece  // no change
}
```

## Complete conversion process workflow

### Phase 1: Initialization
1. Create cube_stickers structure with all stickers set to void/empty
2. Initialize 6 faces (Up, Down, Left, Right, Front, Back)
3. Each face has:
   - 4 corner stickers (UpperLeft, UpperRight, LowerLeft, LowerRight)
   - 4 edge stickers (Upper, Right, Lower, Left)
   - 1 center sticker (固定色)

### Phase 2: Center stickers painting
```
For each face:
    face.center_sticker = fixed_center_color[face]
    // Up: White, Down: Yellow, Left: Orange, Right: Red, Front: Green, Back: Blue
```

### Phase 3: Corner stickers painting
```
For i = 0 to 7:
    // Step 3.1: Get original piece colors
    original_corner = corner_permutation[i]
    base_colors = corner_color_mapping[original_corner]  // 3-element array
    
    // Step 3.2: Apply orientation rotation
    o = corner_orientation[i]
    rotated_colors = [base_colors[(3-o)%3], base_colors[(4-o)%3], base_colors[(2-o)%3]]
    
    // Step 3.3: Get target sticker positions
    target_stickers = corner_affection_mapping[i]  // 3-element array of {face, position}
    
    // Step 3.4: Paint target stickers
    For j = 0 to 2:
        face = target_stickers[j].face
        position = target_stickers[j].position
        color = rotated_colors[j]
        cube_stickers[face][position] = color
```

### Phase 4: Edge stickers painting
```
For i = 0 to 11:
    // Step 4.1: Get original piece colors
    original_edge = edge_permutation[i]
    base_colors = edge_color_mapping[original_edge]  // 2-element array
    
    // Step 4.2: Apply orientation flip
    o = edge_orientation[i]
    rotated_colors = if o == 1 { [base_colors[1], base_colors[0]] } else { base_colors }
    
    // Step 4.3: Get target sticker positions
    target_stickers = edge_affection_mapping[i]  // 2-element array of {face, position}
    
    // Step 4.4: Paint target stickers
    For j = 0 to 1:
        face = target_stickers[j].face
        position = target_stickers[j].position
        color = rotated_colors[j]
        cube_stickers[face][position] = color
```

### Phase 5: Validation
```
// Verify all stickers are painted (no void stickers remain)
total_stickers = 6 * (4_corners + 4_edges + 1_center) = 54
painted_stickers = count_non_void_stickers(cube_stickers)
assert(painted_stickers == 54)
```

### Phase 6: Output conversion
```
// Convert cube_stickers to display format (3x3 grid per face)
For each face:
    display_grid[face] = convert_stickers_to_3x3_grid(cube_stickers[face])
    // Layout: [0,0]=corner_UL, [0,1]=edge_U, [0,2]=corner_UR, etc.
```

## Summary of data flow
```
Input: State {cp[8], co[8], ep[12], eo[12]}
↓
Corner processing: 8 corners × 3 stickers = 24 stickers painted
↓  
Edge processing: 12 edges × 2 stickers = 24 stickers painted
↓
Center processing: 6 centers × 1 sticker = 6 stickers painted
↓
Total: 54 stickers painted
↓
Output: CubeDisplay with 6 faces × 3x3 grids
```
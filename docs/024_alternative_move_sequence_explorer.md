MoveSequenceCollection で得られる手順の近傍探索を実装する。

rubiks_cube_simulator/src/explorer/mixed_nearby_search.rs と似たような実装になる。

MoveSequenceCollection に対して一部変更する対象は、NotationMove である。
MoveSequenceCollection → MoveSequence → NotationMove の構造になっているため、少し特殊になる。

まずは MoveSequence の拡張として、ModifiedMoveSequence を実装する。　
オリジナルの MoveSequence と、変更内容 (Vec{Step, NotationMove}) を持つ構造体とする。
ModifiedMoveSequence は要求に応じて MoveSequence を出力できる。
また、Display も拡張する。変更された Move は表示時に ** で強調表示する。


次に探索について
まずは 1段階の探索を実装する。
すべての MoveSequence に対して、それぞれが持つすべての NotationMove を変更したバリエーションを全て出力する。

例えば 5個のNotationMove を持つ MoveSequence を 3個持つ Collection の場合、変更対象は 15個となる。

それぞれの NotationMove に対して、変更候補を生成する AlternativeGenerator を実装する。
あり得るすべての NotationMove を候補とすると膨大なので、以下のように限定する。
同一グループに含まれる NotationMove のみを候補とする。
U : [U, U', U2, UWide, UWide', U2Wide]
D : [D, D', D2, DWide, DWide', D2Wide]
L : [L, L', L2, LWide, LWide', L2Wide]
R : [R, R', R2, RWide, RWide', R2Wide]
F : [F, F', F2, FWide, FWide', F2Wide]
B : [B, B', B2, BWide, BWide', B2Wide]
M : [M, M', M2, M2']
S : [S, S', S2, S2']
E : [E, E', E2, E2']

例えば U2 から U を候補として選べることにも注意。

また、いずれの NotetionMove からも NOOP として "" も候補として選べるようにする。


これらを利用して、NearbySequenceSearch を実装する。
最後に、NearbySequenceSearchWorkflow を実装する。
- オリジナルの MoveSequenceCollection を受け取る
- (完成状態でない) 2つの State Before, After を受け取る
- どれか一つの NotationMove を変更した ModifiedMoveSequenceCollection を探索する
- 探索された選択肢のうち、Before → After を満たすものを出力する

テストケース

Before State:
cp: [0, 1, 6, 3, 4, 5, 7, 2]
co: [0, 0, 1, 0, 0, 0, 0, 2]
ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

After State:
cp: [0, 6, 2, 3, 4, 1, 7, 5]
co: [0, 2, 0, 0, 0, 2, 1, 1]
ep: [0, 5, 9, 3, 4, 2, 6, 7, 8, 1, 10, 11]
eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

Original Move Sequence:
{R U R' D R U' R' D'}

Target Alternative Move Sequence:
{R2 U R' D R U' R' D'}



================================================================

追記
MoveSequence に対応する ModifiedMoveSequence のように、
MoveSequenceCollection に対応する ModifiedMoveSequenceCollection も実装が必要。

ModifiedMoveSequenceCollection は、オリジナルの MoveSequenceCollection と、
変更内容 (Vec{index of MoveSequence, ModifiedMoveSequence}) を持つ構造体とする。
ModifiedMoveSequenceCollection は要求に応じて、 MoveSequenceCollection を経由して MoveSequence を出力できる。

rubiks_cube_simulator/src/workflow/nearby_sequence_search_workflow.rs の find_alternatives における
// Before → After を満たすものをフィルタ
のステップでは 単一の ModifiedMoveSequence で after を満たすものをフィルタリングしているが、そうではなく、条件を満たす ModifiedMoveSequenceCollection をフィルタリングするように変更する。
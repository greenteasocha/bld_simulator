# CSV のパース
resources/original.csv を読み込む。
各セルの値を二次元のJSONに出力する。

同じ列の1行目のが1階層目、同じ行の1列目を2階層目のキーとして値を格納する。
例:
{
    "UBR": {
        "RDB": "U, R D R'"
    }
}


# 手順のパース

次に、各値について次の変換ルールを適用する。
複数適用可能な場合は、上のルールから順に適用する。

各要素は以下のように定義する

move :: = "U"|"U2"|"U'"|"D"|"D2"|"D'"|"R"|"R2"|"R'"|"L"|"L2"|"L'"|"F"|"F2"|"F'"|"B"|"B2"|"B'"

sequence :: = move [" " <sequence>]

reversed-sequence(sequence) = [reversed_move(m) for m in reverse(sequence)]

reversed_move(move) = case {
    "U" => "U'"
    "U'" => "U" 
    "U2” => "U2"    
    "D" => "D'"
    "D'" => "D" 
    "D2” => "D2"    
    "R" => "R'"         
    "R'" => "R" 
    "R2” => "R2"    
    "L" => "L'"         
    "L'" => "L"         
    "L2” => "L2"
    "F" => "F'"         
    "F'" => "F" 
    "F2” => "F2"    
    "B" => "B'"         
    "B'" => "B" 
    "B2” => "B2"        
}

doubled(move) = case {
    "U" => "U2"
    "U'" => "U2" 
    "D" => "D2"
    "D'" => "D2"    
    "R" => "R2"         
    "R'" => "R2"    
    "L" => "L2"         
    "L'" => "L2"         
    "F" => "F2"         
    "F'" => "F2" 
    "B" => "B2"         
    "B'" => "B2" 
}



1. <sequence_a>, <sequence_b> -> <sequence_a> <sequence_b> <reversed_sequence_a> <reversed_sequence_b>

2. <move>/<sequence>, -> <move> <sequence> <doubled_move> <reversed_sequence> <move>

3. <sequence_a>: <sequence_b> -> <sequence_a> <sequence_b> <reversed_sequence_a>

例:
```
U, R D R' -> <U> <R D R'> reverse<U'> reverse<R D' R'>
          -> <U> <R D R'> <U> reverse<R'> reverse<D'> reverse<R>
          -> U R D R' U R D' R'


D/R' U' R -> <D> <R' U' R> double<D> reverse<R' U R> <D>
          -> <D> <R' U' R> <D2> reverse<R> reverse<U> reverse<R'> <D>
          -> D R' U' R D2 R' U R D

R' D': U/R D R' -> R' D': <U> <R D R'> double<U> reverse<R D R'> <U> 
                 -> R' D': U R D R' U2 R D' R' U 
                 -> <R' D'> <U R D R' U2 R D' R' U> reverse<R' D'> 
                 -> R' D' U R D R' U2 R D' R' U D R
                
U R U': D, R' U' R -> U R U': <D>, <R' U' R>, reverse<D>, reverse<R' U' R>
                   -> U R U': D R' U' R D' R' U R
                   -> <U R U'> <D R' U' R D' R' U R> reverse <U R U'>
                   -> U R U' D R' U' R D' R' U R U R' U'
```


# 手順のパース - 形式的な定義

:::note info
LLM による定義の推敲
:::

## 構文定義（EBNF）

Move ::= "U" | "U2" | "U'" 
       | "D" | "D2" | "D'" 
       | "R" | "R2" | "R'" 
       | "L" | "L2" | "L'" 
       | "F" | "F2" | "F'" 
       | "B" | "B2" | "B'"

Sequence ::= Move { " " Move }*

Notation ::= CommaNotation
           | SlashNotation  
           | ColonNotation

CommaNotation ::= Sequence "," Sequence
SlashNotation ::= Move "/" Sequence
ColonNotation ::= Sequence ":" Sequence

## 補助関数の定義

reversed_move: Move → Move
  定義:
    reversed_move("U")  = "U'"
    reversed_move("U'") = "U"
    reversed_move("U2") = "U2"
    [他の面も同様に定義]

reversed_sequence: Sequence → Sequence
  定義:
    reversed_sequence(ε) = ε  (空シーケンス)
    reversed_sequence(m seq) = reversed_sequence(seq) reversed_move(m)
    ※ ε は空、m は Move、seq は Sequence

doubled_move: Move → Move
  定義:
    doubled_move("U")  = "U2"
    doubled_move("U'") = "U2"
    [他の面も同様に定義]

## 変換規則

規則1（カンマ記法）:
  入力: seq_a, seq_b
  出力: seq_a seq_b reversed_sequence(seq_a) reversed_sequence(seq_b)

規則2（スラッシュ記法）:
  入力: m/seq
  出力: m seq doubled_move(m) reversed_sequence(seq) m
  ただし m ∈ Move

規則3（コロン記法）:
  入力: seq_a: seq_b
  出力: seq_a seq_b reversed_sequence(seq_b)

## 適用順序
複数の規則が適用可能な場合、規則1 → 規則2 → 規則3 の順に適用する。
ここまででコーナーについて以下を実装した

- 与えられた co, cp から完成状態までに必要な2点交換を列挙する
- 2点交換が完了した後に必要なコーナーツイストを列挙する
- 2点交換とツイストを組み合わせた一連の手順を生成する

- 与えられた操作(2点交換、ツイスト)列のうち一部を異なる操作に置き換えるとどのようなstateに変化するかシミュレーションする
- 完成状態に到達するはずのどの操作列にたいして、どの操作を別の操作に置き換えると与えられた別の(完成状態ではない)Stateに到達するかを探索する

- json ファイルから、一つの操作から回転(U,D,F,B,R,Lなど)列への変換を読み込む
- 与えられた操作列を回転列に変換する
- 操作列の分岐探索のように、回転列に対しても特定の State にたどり着くための分岐の探索を行う。


上記をエッジについても実装する

参考にするファイル
- src/inspection/
- src/parser/
- src/explorer/



コーナーとの違いとして、eo, ep の値に対しての表記が異なる。

SwapOperation については、
 TARGET_STICKERS = [
            ["BL", "LB"],
            ["BR", "RB"],
            ["FR", "RF"],
            ["FL", "LF"],
            ["UB", "BU"],
            ["UR", "RU"],
            ["UF", "FU"],
            ["UL", "LU"],
            ["DB", "BD"],
            ["DR", "RD"],
            ["DF", "FD"],
            ["DL", "LD"],
        ];


TwistOperation については、
    TARGET_STICKERS = ["BL", "BR", "FR", "FL", "UB", "UR", "UF", "UL", "DB", "DR", "DF", "DL"];

    FLIP_EXISTANCE = [
            "not flipped",    
            "flipped", 
        ];

とする。FLIP_EXISTANCE はコーナーにおける ROTATION_DIRECTIONS に対応する。
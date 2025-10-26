Corner workflow と Edge workflow を統合してルービックキューブを解くユースケースを作成する。

# 統合ワークフロー概要
引数: state
出力: 操作列(Corner + Edge)
      操作列から生成した Move Sequence


# ワークフロー詳細
1. Corner workflow を実行し、Corner 用の操作列を取得する。
2. 交換分析の有無を判断する。
    - Cornerの出力に含まれる SwapOperation の数が奇数の場合のみ、Edge workflow の交換分析モードをオンにする。
    - TwistOperation の数は考慮に入れない
3. Edge workflow を実行し、Edge 用の操作列を取得する。
4. Corner 用と Edge 用の操作列を結合し、最終的な操作列を生成する。
    - 空の操作列に Edge の操作を全て push する。
    - その後、Corner の操作を全て push する。

6. Corner 用と Edge 用の操作列を結合し、最終的な Move Sequence を生成する。
    - 空の Move Sequence に Edge の操作列から生成される Move Sequence を全て push する。
    - その後、Corner の操作列から生成される Move Sequence を全て push する。


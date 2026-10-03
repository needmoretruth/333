### What the shared parts of the commands say.

commands-clock-at-zero = { $epoch }。このマシンの時計は 1970 年を指しているので、このノードは
    時の始まりにいると思っています。時計が合うまで、誰もファイルを
    渡さず、誰も証言しません。
    .keyword = エポック

commands-called-first = 最初に作った鍵が選ばれました。
    .keyword = 選出

commands-called = { $not_called ->
       *[other] { $not_called } 個の鍵を作り、選びませんでした。選んだのはこの鍵です。
    }
    .keyword = 選出

commands-torn = 書きかけのエントリ { $bytes } バイトを記録から取り除きました
    .keyword = 破損

commands-record = { $epochs ->
       *[other] { $epochs } エポックに応答済み。どれも書き換えられません
    }
    .keyword = 記録

commands-witnessed = { $statements ->
       *[other] 他の鍵がこのノードについて署名した言明 { $statements } 件。
            そのエポックが過ぎても保管します。ほかに期間を越えて
            残るものがないからです。
    }
    .keyword = 証言

commands-unseen = どのエポックでも、このノードについて署名されたものはありません。
    外へはつながりますが外からは届かず、数えられるのは後者です。
    質問するよう抽選で選ばれた相手が、ここへたどり着く必要があります。
    原因は二つです。ポート 3333 をこのマシンへ送らないルーターと、
    誰にも渡していないアドレスです。`run --tor` ならどちらも要りません。
    onion アドレスはどんなルーターの後ろからでも届き、
    このクライアントは Tor を内蔵しています。
    .keyword = 不可視

commands-roll-alone = 1 人、つまりこのノード
    .keyword = 名簿

commands-roll = { $members } 人
    .keyword = 名簿

commands-known = { $addresses } 人が探す場所を示しています
    .keyword = 既知

commands-holding = ファイルを持ち、渡すことができます
    .keyword = 保持

commands-keeping = すべてを永久に。このノードの得にはなりません。言明はそれぞれ
    自分の署名を持ち、どこに保管しても同じように検証できます。
    公式の保管庫も保管係もありません。
    .keyword = 保管

commands-ignored = 読めなかった加入 { $admissions } 件
    .keyword = 無視

commands-learned-where = さらに { $addresses } 人の居場所
    .keyword = 学習

commands-rejoined = さらに { $members } 人の名前。{ $were } 人を知るノードから。
    二つだった数え方が一つになりました。
    .keyword = 再結合

commands-learned-names = さらに { $members } 人の名前
    .keyword = 学習

commands-heard = { $speakers } 人が話しています
    .keyword = 聴取

commands-carried = { $statements ->
       *[other] まだ開いているエポックについての言明 { $statements } 件
    }
    .keyword = 運搬

commands-exchange = { $node }  エポック { $epoch }  { $clocks }  ({ $liveness })
    .keyword = 証言

commands-answered-the-challenge = こちらが選んだ課題に答えました
commands-spoke-first = 先に話しました。話したことしか証明しません

commands-clocks-together = 時計は一致
commands-clocks-ahead = 相手の時計が { $apart } 進んでいます
commands-clocks-behind = 相手の時計が { $apart } 遅れています
commands-hours-and-minutes = { $hours } 時間 { $minutes } 分
commands-minutes-and-seconds = { $minutes } 分 { $seconds } 秒
commands-seconds = { $seconds } 秒

commands-waking = Tor。隠れた道が開くまで少しかかります。
    .keyword = 起動

commands-waking-through = Tor、{ $bridges ->
       *[other] { $bridges } 本のブリッジ経由
    }。隠れた道が開くまで少しかかります。
    .keyword = 起動

commands-no-tor = { $seconds } 秒たっても Tor に接続できません
commands-starting-tor = Tor クライアントを起動中

# What a handover puts a signature under, read back.
commands-signed-giving = あなた: エポック { $epoch } にあなたへファイルを渡しました。
    相手: エポック { $epoch } にあなたからファイルを受け取りました。
    二つの手で書かれ、どちらの手も取り消せません。
    .keyword = 署名

commands-signed-taking = 相手: エポック { $epoch } にあなたへファイルを渡しました。
    あなた: エポック { $epoch } にあなたからファイルを受け取りました。
    二つの手で書かれ、どちらの手も取り消せません。
    .keyword = 署名

commands-brimming = { $statements ->
       *[other] 言明 { $statements } 件が一回に収まらず、次を待っています
    }
    .keyword = 満杯

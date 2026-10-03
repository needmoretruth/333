### `333 run`: the door, who gets in and who is turned away.

serve-door-over-tor = tor 経由

serve-door-full = { $caller }: この扉は満員です
    .keyword = 断り

serve-door-silence-exchange = その状態が { $seconds } 秒続いたので手を離しました
    .keyword = 沈黙

serve-door-silence-greeting = { $seconds } 秒間一言もなかったので、扉をまた空けました
    .keyword = 沈黙

serve-door-knock = このノードが自分の玄関に着きました
    .keyword = 訪問

serve-door-broken-heartbeat = { $caller } が鼓動の途中で止まりました: { $why }
    .keyword = 中断

serve-door-broken-exchange = { $caller } が交換の途中で止まりました: { $why }
    .keyword = 中断

serve-door-refused = { $caller }: { $why }
    .keyword = 拒否

serve-door-failed-answering = { $caller } に応答中: { $why }
    .keyword = 失敗

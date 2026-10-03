### `333 run`: keeping the hours, at every epoch boundary.

hours-failed-marking = このエポックを記録中: { $why }
    .keyword = 失敗

hours-failed-sources = アドレスの出所を書き留め中: { $why }
    .keyword = 失敗

hours-not-leaving = このアドレスは { $place } に残しません。ここからしかこのノードに
    届かず、知らない人がかけると、その人自身の何かにつながります。
    .keyword = 集合

hours-sealing = このノードのアドレスを封印中

hours-failed-keeping-address = このノード自身のアドレスを保管中: { $why }
    .keyword = 失敗

hours-failed-saying-where = このノードの居場所を伝えています: { $why }
    .keyword = 失敗

hours-forgot = { $epochs ->
       *[other] { $epochs } エポック
    }。今それについて何を言っても判定は変わりません。
    .keyword = 忘却

hours-failed-forgetting = 古い言明を忘れています: { $why }
    .keyword = 失敗

hours-minutes = { $minutes ->
       *[other] { $minutes } 分
    }

hours-minutes-and-seconds = { $minutes } 分 { $seconds } 秒
hours-hours-and-minutes = { $hours } 時間 { $minutes } 分

hours-epochs-answered-for = { $epochs ->
       *[other] { $epochs } エポック応答済み
    }

### `333 run`: the door, who gets in and who is turned away.

serve-door-over-tor = 經由 tor

serve-door-full = { $caller }：這扇門已滿
    .keyword = 拒之

serve-door-silence-exchange = 這樣持續了 { $seconds } 秒，所以我們放手
    .keyword = 沉默

serve-door-silence-greeting = { $seconds } 秒一言不發，所以門又空出來了
    .keyword = 沉默

serve-door-knock = 這個節點走到了自己的門前
    .keyword = 敲門

serve-door-broken-heartbeat = { $caller } 在心跳完成前停了：{ $why }
    .keyword = 中斷

serve-door-broken-exchange = { $caller } 在交換完成前停了：{ $why }
    .keyword = 中斷

serve-door-refused = { $caller }：{ $why }
    .keyword = 拒絕

serve-door-failed-answering = 正在應答 { $caller }：{ $why }
    .keyword = 失敗

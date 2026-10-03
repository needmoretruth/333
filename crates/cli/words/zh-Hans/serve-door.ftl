### `333 run`: the door, who gets in and who is turned away.

serve-door-over-tor = 经由 tor

serve-door-full = { $caller }：这扇门已满
    .keyword = 拒之

serve-door-silence-exchange = 这样持续了 { $seconds } 秒，所以我们放手
    .keyword = 沉默

serve-door-silence-greeting = { $seconds } 秒一言不发，所以门又空出来了
    .keyword = 沉默

serve-door-knock = 这个节点走到了自己的门前
    .keyword = 敲门

serve-door-broken-heartbeat = { $caller } 在心跳完成前停了：{ $why }
    .keyword = 中断

serve-door-broken-exchange = { $caller } 在交换完成前停了：{ $why }
    .keyword = 中断

serve-door-refused = { $caller }：{ $why }
    .keyword = 拒绝

serve-door-failed-answering = 正在应答 { $caller }：{ $why }
    .keyword = 失败

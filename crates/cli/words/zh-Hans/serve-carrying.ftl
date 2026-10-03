### `333 run`: carrying out what the node was told, from its screen or another terminal.

serve-carrying-not-written-who = 正在记下谁应答了：{ $why }
    .keyword = 失败

serve-carrying-unheard = { $why }
    .keyword = 未听到

serve-carrying-unbegun = { $why }
    .keyword = 未开始

serve-carrying-refused = { $why }
    .keyword = 拒绝

serve-carrying-holding-the-file = 名册上 { $roll } 位同伴，文件在这里
    .keyword = 持有

serve-carrying-holding-no-file = 名册上 { $roll } 位同伴，这个节点还没拿到文件
    .keyword = 持有

serve-carrying-unread-holding = 这个节点持有的东西：{ $why }
    .keyword = 无法读

serve-carrying-already-up = 隐藏地址已经开着。`tor off` 关闭它。
    .keyword = 已开启

serve-carrying-unraised = { $why }
    .keyword = 未开启

serve-carrying-tor-off = onion 地址现在停止应答。关于它已说过的话仍然有效，直到说出后
    两个纪元被遗忘。
    .keyword = 隐藏

serve-carrying-none-up = 没有开着的隐藏地址可关。
    .keyword = 已开启

serve-carrying-too-late = Tor 已在运行，现在加的网桥改变不了它已建立的连接。请改为带着
    它重启节点。
    .keyword = 太晚

serve-carrying-bridged = { $bridges ->
       *[other] { $bridges } 座网桥
    }会在 Tor 下次启动时使用。
    .keyword = 网桥

serve-carrying-helper = 任何混淆网桥都会运行 { $program }。
    .keyword = 网桥

serve-carrying-not-an-address = { $typed } 不是地址：{ $why }
    .keyword = 无法读

serve-carrying-stopping = 另一个终端要求的。
    .keyword = 停止中

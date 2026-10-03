### `333 run`: keeping the hours, at every epoch boundary.

hours-failed-marking = 正在记录本纪元：{ $why }
    .keyword = 失败

hours-failed-sources = 正在记下地址的来源：{ $why }
    .keyword = 失败

hours-not-leaving = 不在 { $place } 留下这个地址。它只能从这里到达这个节点，陌生人
    拨它会连到自己那边的东西。
    .keyword = 汇合

hours-sealing = 正在封存这个节点的地址

hours-failed-keeping-address = 正在保存这个节点自己的地址：{ $why }
    .keyword = 失败

hours-failed-saying-where = 正在说明这个节点在哪里：{ $why }
    .keyword = 失败

hours-forgot = { $epochs ->
       *[other] { $epochs } 个纪元
    }。现在关于它们说什么都改变不了判定。
    .keyword = 遗忘

hours-failed-forgetting = 正在遗忘旧声明：{ $why }
    .keyword = 失败

hours-minutes = { $minutes ->
       *[other] { $minutes } 分钟
    }

hours-minutes-and-seconds = { $minutes } 分 { $seconds } 秒
hours-hours-and-minutes = { $hours } 小时 { $minutes } 分

hours-epochs-answered-for = { $epochs ->
       *[other] 已应答 { $epochs } 个纪元
    }

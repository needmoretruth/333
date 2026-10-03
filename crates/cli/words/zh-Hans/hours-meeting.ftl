### `333 run`: leaving this node's address at a meeting point, and reading everyone else's.

hours-meeting-unreadable = 无法读取 { $place }：{ $why }
    .keyword = 汇合

hours-meeting-read-failed-inside = 读取 { $place } 时在这个节点内部失败：{ $why }
    .keyword = 汇合

hours-meeting-left = 已在 { $place } 留下这个节点的地址
    .keyword = 汇合

hours-meeting-stopped = { $place } 应答前这个节点已停止
    .keyword = 汇合

hours-meeting-leaving-failed-inside = 在 { $place } 留下这个节点的地址时，在这个节点内部
    失败：{ $why }
    .keyword = 汇合

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place } 每个网络地址每分钟只收一条声明，而这个地址不到
    一分钟前刚发过一条。{ $holding }
    这个节点会在 { $when } 再次留下地址。
    .keyword = 汇合

hours-meeting-full = { $place } 已收满一天的声明，UTC 午夜后再收。仍然可以读取。{ $holding }
    这个节点会在 { $next_epoch } 再次留下地址。
    .keyword = 汇合

hours-meeting-full-until = { $place } 已收满一天的声明，UTC 午夜（{ $midnight }后）再收。
    仍然可以读取。{ $holding }
    这个节点会在 { $next_epoch } 再次留下地址。
    .keyword = 汇合

hours-meeting-holds-from = 它还保留着这个节点纪元 { $epoch } 的地址。
hours-meeting-holds-nothing = 它没有保留这个节点的任何东西。

hours-meeting-at-the-next-epoch = 下一个纪元，{ $wait }后
hours-meeting-in = { $wait }后

hours-meeting-did-not-reach = 这个节点的地址没有到达 { $place }：{ $why }
    .keyword = 汇合

hours-meeting-not-taken = { $place } 没有接受这个节点的地址：{ $why }
    .keyword = 汇合

hours-meeting-seconds = { $seconds ->
       *[other] { $seconds } 秒
    }

hours-meeting-nobody = 没有人在 { $place } 说自己在哪里
    .keyword = 汇合

hours-meeting-newer = { $fresh ->
       *[other] { $place } 有 { $fresh } 个更新的地址
    }
    .keyword = 汇合

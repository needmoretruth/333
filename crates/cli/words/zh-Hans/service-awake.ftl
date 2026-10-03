### `333 service`: the line a running node writes to say it is still awake.

service-awake-failed = 正在 { $root } 写入节点醒着的记录：{ $why }。在这恢复之前，这台
    机器上没有任何东西能知道它在运行。
    .keyword = 失败

service-awake-never-kept = 没有运行。服务已安装，但节点从未报告过醒着。`333 service status`
    会说明原因。
    .keyword = 节点

service-awake-not-kept-since = 自 { $at } 起没有运行，{ $ago }前。`333 service status` 会说明原因。
    .keyword = 节点

service-awake-under-a-minute = 不到一分钟

service-awake-minutes = { $minutes ->
       *[other] { $minutes } 分钟
    }

service-awake-epochs = { $epochs ->
       *[other] { $epochs } 个纪元
    }

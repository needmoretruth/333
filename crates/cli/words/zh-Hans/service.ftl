### `333 service`: running the node through logouts and reboots, when asked to.

service-mind = { $node } 位于这个系统会清空的地方，而这个节点的名字别处都没有。
    服务会在那里运行它，直到被清空。
    .keyword = 注意

service-runs = { $command }
    .keyword = 节点

service-undo-partial = `333 service uninstall` 会移除这些步骤里已经做了的部分。
    .keyword = 撤销

service-no-receipt-directory = 这个系统没有给出存放凭据的配置目录

service-wrote-receipt = { $path }，`333 service uninstall` 靠它知道要撤销什么。
    .keyword = 已写入

service-undo = `333 service uninstall` 会停止节点并撤销以上全部。
    节点自己的目录两者都不会碰。
    .keyword = 撤销

service-uninstalled = 不再由服务运行。{ $node } 保持节点离开时的样子：`333 run` 手动
    运行它，`333 start` 重新设置服务。
    .keyword = 节点

service-none-installed = 没有为这个用户通过 `333 service install` 安装过。
    .keyword = 服务

service-state = { $state }
    .keyword = 服务

service-node = { $node }
    .keyword = 节点

service-last-awake = 最后一次是 { $at }，{ $ago }前
    .keyword = 醒着

service-never-awake = 在这个目录里从没报告过
    .keyword = 醒着

service-said-nothing = 没有保留下来的
    .keyword = 输出

service-said-last = { $lines ->
       *[other] 最后 { $lines } 行：
    }
    .keyword = 输出

service-no-manager = 这个系统没有 `333 service` 会询问的服务管理器。`333 run --plain`
    可在这里让程序持续运行的东西下运行节点。

service-not-installed-here = 未安装：这里没有它认识的服务管理器

# Said by every service manager's own file.

service-creating = 正在创建 { $path }
service-writing = 正在写入 { $path }
service-removing = 正在移除 { $path }

service-wrote = { $path }
    .keyword = 已写入

service-removed = { $path }
    .keyword = 已移除

service-left = { $path }。不是 `333 service install` 写的。
    .keyword = 保留

service-failed = { $why }
    .keyword = 失败

service-not-installed = 未安装
service-running = 运行中
service-starting = 启动中

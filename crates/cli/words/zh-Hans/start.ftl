### `333 start` and `333 restart`: running this node in the background.

start-no-node = { $home } 里还没有节点

start-no-node-next = `333 join <invitation>` 用运行 333 的人给的邀请加入。
    `333 begin` 独自开始。

start-in-a-terminal = 已经在终端里运行。在那里停止它，或用 `333 stop`，然后
    `333 start` 会在后台运行它。
    .keyword = 运行

start-already = 已经在后台运行。
    .keyword = 运行

start-started = 在后台，现在和每次重启后。`333 status` 显示状况；`333 stop` 停止它。
    .keyword = 已启动

start-elsewhere = 这台机器的后台服务运行的是 { $other } 里的节点。
    `333 service uninstall` 移除它，然后再 `333 start`。

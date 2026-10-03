### `333 run`: orders from other terminals on this machine.

serve-told-not-here = 在这个系统上，命令只能通过屏幕到达这个节点。在旁边启动第二个
    333 会被拒绝。
    .keyword = 命令

serve-told-cannot = 无法从其他终端接收：{ $why }
    .keyword = 命令

serve-told-not-private = 无法把套接字设为私有

serve-told-taking = 来自这台机器的任何终端，用屏幕上的说法：
    `333 say 7`、`333 join <invitation>`、`333 tell 'tor on'`、`333 stop`。
    这个节点执行它们，并在那里回应。
    .keyword = 命令

serve-told-taking-light = 来自这台机器的任何终端：
    `333 say 7`、`333 join <invitation>`、`333 tell 'tor on'`、`333 stop`。
    这个节点执行它们，并在那里回应。
    .keyword = 命令

serve-told-old-socket = 一个旧套接字挡着，会留在那里：{ $why }
    .keyword = 命令

serve-told-not-a-socket = { $path } 存在但不是套接字，所以不动它；在它被移走之前，无法从
    其他终端把任何东西交给这个节点。
    .keyword = 命令

serve-told-no-longer = 不再从其他终端接收：{ $why }
    .keyword = 命令

serve-told-not-the-owner = 只有这个节点目录的所有者才能吩咐它
    .keyword = 拒绝

serve-told-too-long = 这比任何命令都长。最长的是 { $bytes } 字节。
    .keyword = 无法读

serve-told-other-version = 这个节点讲 { $ours }，却被用 { $theirs } 提问。提问的 333 与正在
    运行的版本不同；请用那一个。
    .keyword = 拒绝

serve-told-unread = { $why }
    .keyword = 无法读

serve-told-asked = { $order }，来自另一个终端
    .keyword = 请求

serve-told-unheard = 已经没有东西在执行命令
    .keyword = 未听到

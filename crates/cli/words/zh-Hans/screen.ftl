### The screen: what it says in the log pane about what was typed into it.

screen-asked = { $typed }
    .keyword = 请求
screen-unheard = 已经没有东西在执行命令
    .keyword = 未听到
screen-unread = { $why }
    .keyword = 无法读
screen-refused = { $why }
    .keyword = 拒绝

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = 有个按键无法读取：{ $why }。之后的按键也许可以。
    .keyword = 键盘
screen-keyboard-gone = 无法读取键盘（{ $why }），所以屏幕关闭了，节点也随之停止。
    `333 run --plain` 可以不用键盘运行节点。

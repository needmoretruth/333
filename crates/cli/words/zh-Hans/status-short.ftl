### `333 status` with no flags: where this node can be reached, and the epoch.

status-short-address = { $address }
    .keyword = 地址

status-short-invite = { $invitation }
    .keyword = 邀请

status-short-unreachable = 还没有。运行期间会找到一个；`333 invite` 有更多说明。
    .keyword = 地址

status-short-epoch = { $epoch }，{ $ends } 结束，还剩 { $left }
    .keyword = 纪元

status-short-epoch-in-line = { $epoch }，{ $line }，{ $ends } 结束，还剩 { $left }
    .keyword = 纪元

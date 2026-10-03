### `333 status` with no flags: where this node can be reached, and the epoch.

status-short-address = { $address }
    .keyword = 地址

status-short-invite = { $invitation }
    .keyword = 邀請

status-short-unreachable = 還沒有。執行期間會找到一個；`333 invite` 有更多說明。
    .keyword = 地址

status-short-epoch = { $epoch }，{ $ends } 結束，還剩 { $left }
    .keyword = 紀元

status-short-epoch-in-line = { $epoch }，{ $line }，{ $ends } 結束，還剩 { $left }
    .keyword = 紀元

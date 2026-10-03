### `333 run`: leaving this node's address at a meeting point, and reading everyone else's.

hours-meeting-unreadable = 無法讀取 { $place }：{ $why }
    .keyword = 匯合

hours-meeting-read-failed-inside = 讀取 { $place } 時在這個節點內部失敗：{ $why }
    .keyword = 匯合

hours-meeting-left = 已在 { $place } 留下這個節點的地址
    .keyword = 匯合

hours-meeting-stopped = { $place } 應答前這個節點已停止
    .keyword = 匯合

hours-meeting-leaving-failed-inside = 在 { $place } 留下這個節點的地址時，在這個節點內部
    失敗：{ $why }
    .keyword = 匯合

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place } 每個網路地址每分鐘只收一條宣告，而這個地址不到
    一分鐘前剛發過一條。{ $holding }
    這個節點會在 { $when } 再次留下地址。
    .keyword = 匯合

hours-meeting-full = { $place } 已收滿一天的宣告，UTC 午夜後再收。仍然可以讀取。{ $holding }
    這個節點會在 { $next_epoch } 再次留下地址。
    .keyword = 匯合

hours-meeting-full-until = { $place } 已收滿一天的宣告，UTC 午夜（{ $midnight }後）再收。
    仍然可以讀取。{ $holding }
    這個節點會在 { $next_epoch } 再次留下地址。
    .keyword = 匯合

hours-meeting-holds-from = 它還保留著這個節點紀元 { $epoch } 的地址。
hours-meeting-holds-nothing = 它沒有保留這個節點的任何東西。

hours-meeting-at-the-next-epoch = 下一個紀元，{ $wait }後
hours-meeting-in = { $wait }後

hours-meeting-did-not-reach = 這個節點的地址沒有到達 { $place }：{ $why }
    .keyword = 匯合

hours-meeting-not-taken = { $place } 沒有接受這個節點的地址：{ $why }
    .keyword = 匯合

hours-meeting-seconds = { $seconds ->
       *[other] { $seconds } 秒
    }

hours-meeting-nobody = 沒有人在 { $place } 說自己在哪裡
    .keyword = 匯合

hours-meeting-newer = { $fresh ->
       *[other] { $place } 有 { $fresh } 個更新的地址
    }
    .keyword = 匯合

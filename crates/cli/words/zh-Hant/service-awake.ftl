### `333 service`: the line a running node writes to say it is still awake.

service-awake-failed = 正在 { $root } 寫入節點醒著的記錄：{ $why }。在這恢復之前，這台
    機器上沒有任何東西能知道它在執行。
    .keyword = 失敗

service-awake-never-kept = 沒有執行。服務已安裝，但節點從未報告過醒著。`333 service status`
    會說明原因。
    .keyword = 節點

service-awake-not-kept-since = 自 { $at } 起沒有執行，{ $ago }前。`333 service status` 會說明原因。
    .keyword = 節點

service-awake-under-a-minute = 不到一分鐘

service-awake-minutes = { $minutes ->
       *[other] { $minutes } 分鐘
    }

service-awake-epochs = { $epochs ->
       *[other] { $epochs } 個紀元
    }

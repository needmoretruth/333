### `333 service`: running the system's own programs, and saying so.

service-programs-ran = { $command }
    .keyword = 已執行

service-programs-did-not-succeed = `{ $command }` 沒有成功：{ $why }
service-programs-ended-with = 以 { $status } 結束
service-programs-no-such = 這個系統上沒有 { $program }
service-programs-not-started = 無法啟動 { $program }：{ $why }

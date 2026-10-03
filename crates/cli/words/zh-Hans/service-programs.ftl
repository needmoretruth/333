### `333 service`: running the system's own programs, and saying so.

service-programs-ran = { $command }
    .keyword = 已运行

service-programs-did-not-succeed = `{ $command }` 没有成功：{ $why }
service-programs-ended-with = 以 { $status } 结束
service-programs-no-such = 这个系统上没有 { $program }
service-programs-not-started = 无法启动 { $program }：{ $why }

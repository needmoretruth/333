### `333 service`: running the system's own programs, and saying so.

service-programs-ran = { $command }
    .keyword = 実行

service-programs-did-not-succeed = `{ $command }` が成功しませんでした: { $why }
service-programs-ended-with = { $status } で終了しました
service-programs-no-such = このシステムに { $program } はありません
service-programs-not-started = { $program } を起動できませんでした: { $why }

### `333 service` on Windows: Task Scheduler.

service-schtasks-cannot-pass = `{ $word }` に " か % が含まれ、cmd.exe はそのまま渡せません
service-schtasks-no-user = Windows がこのユーザーを示しませんでした
    (%USERDOMAIN% と %USERNAME%)

service-schtasks-logon = Windows はログインしている間、ログインした時からノードを
    動かします。起動時から動くサービスには専用のアカウントが要り、
    ノードはあなた自身のディレクトリにあります。
    出力は { $log } にあります。
    .keyword = ログオン

service-schtasks-ready = 停止中。333 秒以内に再び起動します
service-schtasks-disabled = 無効: 自分では再び起動しません

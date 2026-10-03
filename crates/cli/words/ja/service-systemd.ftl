### `333 service` on Linux: systemd.

service-systemd-no-configuration = このシステムはこのユーザーの設定ディレクトリを示しません

service-systemd-no-session = systemd はここでこのユーザーのセッションを保っていません
    ({ $why })。このユーザーがコンソールか ssh でログインすると始まり、
    su や sudo では始まりません。このユーザーでログインして実行し直して
    ください。

service-systemd-wrote-over = { $path }、元のものと置き換え
    .keyword = 書込

service-systemd-linger-already = { $user } ではすでに有効です。ログアウト後もノードを動かし続け、
    誰もログインしていなくても起動時に動かします。
    .keyword = linger

service-systemd-linger-on = { $user } で有効にしました。ログアウト後もノードを動かし続け、
    誰もログインしていなくても起動時に動かします。
    .keyword = linger

service-systemd-linger-not-on = 無効: { $why }。これがないとログアウトでノードが止まり、
    再起動後はログインを待ちます。
    `sudo loginctl enable-linger { $user }` で有効になります。
    .keyword = linger

service-systemd-linger-off = インストール前のとおり、また無効にしました。
    .keyword = linger

service-systemd-linger-left = そのままにしました。インストールで有効にしたものではありません。
    .keyword = linger

service-systemd-not-answering = 不明: systemd がこのユーザーについて応答しません
service-systemd-restarting = 停止中。止まってから 333 秒後に再び起動します
service-systemd-stopping = 停止しています
service-systemd-failed = 失敗 ({ $result })
service-systemd-stopped = 停止
service-systemd-at-every-boot = { $state }、毎回の起動時に開始
service-systemd-not-again = { $state }、{ $file_state }: 自分では再び起動しません

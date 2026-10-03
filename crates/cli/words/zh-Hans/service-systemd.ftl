### `333 service` on Linux: systemd.

service-systemd-no-configuration = 这个系统没有给出这个用户的配置目录

service-systemd-no-session = systemd 在这里没有为这个用户维持会话（{ $why }）。这个用户在
    控制台或通过 ssh 登录时才会开始，su 或 sudo 不行。以这个用户
    登录后再运行一次。

service-systemd-wrote-over = { $path }，替换了原来的
    .keyword = 已写入

service-systemd-linger-already = 已为 { $user } 开启。注销后节点继续运行，开机时无人登录也会启动。
    .keyword = linger

service-systemd-linger-on = 已为 { $user } 开启。注销后节点继续运行，开机时无人登录也会启动。
    .keyword = linger

service-systemd-linger-not-on = 未开启：{ $why }。没有它，节点会在你注销时停止，重启后等你再
    登录。`sudo loginctl enable-linger { $user }` 可开启。
    .keyword = linger

service-systemd-linger-off = 已重新关闭，与安装前一样。
    .keyword = linger

service-systemd-linger-left = 保持原样。安装时并没有开启它。
    .keyword = linger

service-systemd-not-answering = 未知：systemd 没有为这个用户应答
service-systemd-restarting = 已停止，停止 333 秒后会再次启动
service-systemd-stopping = 正在停止
service-systemd-failed = 失败（{ $result }）
service-systemd-stopped = 已停止
service-systemd-at-every-boot = { $state }，每次开机时启动
service-systemd-not-again = { $state }，{ $file_state }：不会自己再启动

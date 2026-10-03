### `333 service` on Windows: Task Scheduler.

service-schtasks-cannot-pass = `{ $word }` 含有 " 或 %，cmd.exe 无法原样传递
service-schtasks-no-user = Windows 没有说明这是哪个用户（%USERDOMAIN% 和 %USERNAME%）

service-schtasks-logon = 你登录期间，Windows 从登录起运行节点。开机就运行的服务需要专用
    账户，而节点住在你自己的目录里。
    它的输出在 { $log }。
    .keyword = 登录

service-schtasks-ready = 已停止，333 秒内会再次启动
service-schtasks-disabled = 已禁用：不会自己再启动

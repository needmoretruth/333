### `333 service` on macOS: launchd.

service-launchd-no-home = 这个系统没有给出这个用户的主目录
service-launchd-asking-who = 正在问 `id -u` 这是哪个用户

service-launchd-login = launchd 从你登录到注销一直运行节点，重启后你登录时再重新开始。
    它的输出在 { $log }。
    .keyword = 登录

service-launchd-ended-with = { $state }，上次以 { $code } 结束
service-launchd-loaded = 已加载，launchd 没有说它在做什么

### `333 run`: the socket listener.

serve-socket-port-taken = 这台机器上已有东西在监听端口 { $port }：另一个节点或别的程序。
    停掉它，或用 --bind 指定别的端口。

serve-socket-not-here = { $ip } 不是这台机器的地址。--bind 0.0.0.0:{ $port } 会监听所有地址。

serve-socket-privileged = 端口 { $port } 小于 1024，只有这台机器的管理员能监听。请用
    --bind 指定更大的端口：--bind { $suggested }。

serve-socket-listening-on = 正在监听 { $bind }

serve-socket-accepting = 正在接受对方

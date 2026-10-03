### `333 run`: the socket listener.

serve-socket-port-taken = 這台機器上已有東西在監聽埠 { $port }：另一個節點或別的程式。
    停掉它，或用 --bind 指定別的埠。

serve-socket-not-here = { $ip } 不是這台機器的地址。--bind 0.0.0.0:{ $port } 會監聽所有地址。

serve-socket-privileged = 埠 { $port } 小於 1024，只有這台機器的管理員能監聽。請用
    --bind 指定更大的埠：--bind { $suggested }。

serve-socket-listening-on = 正在監聽 { $bind }

serve-socket-accepting = 正在接受對方

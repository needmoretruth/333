### `333 run`: the socket listener.

serve-socket-port-taken = このマシンでは、すでに何かがポート { $port } で待ち受けています。
    別のノードかほかのプログラムです。それを止めるか、--bind で別の
    ポートを指定してください。

serve-socket-not-here = { $ip } はこのマシンのアドレスではありません。--bind 0.0.0.0:{ $port }
    ならすべてで待ち受けます。

serve-socket-privileged = ポート { $port } は 1024 未満で、このマシンの管理者しか待ち受け
    できません。それより大きいポートを --bind で: --bind { $suggested }。

serve-socket-listening-on = { $bind } で待ち受け中

serve-socket-accepting = 相手を受け入れています

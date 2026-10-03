### Reading an address somebody typed: what was wrong with one, and how one is written.

typed-address-refused = { $why }。地址是 host:port，如 node.example:3333；邀请是 333: 加
    地址，如 333:node.example:3333。
typed-address-refused-announce = { $why }。地址是 host:port，如 node.example:3333。
typed-address-refused-bind = { $typed } 不是可监听的地址。它应是 IP 地址加端口，如 0.0.0.0:3333。
    只给地址就监听 3333 端口，只给 :port 就监听所有地址。

typed-address-no-tag = 邀请以 333: 开头
typed-address-too-long = 邀请最多 { $most } 个字符，这个有 { $length } 个
typed-address-not-canonical = 我们每一位都是一个地方，只有一种写法，这张邀请是 { $canonical }
typed-address-wrong-tag = 邀请以 333: 开头，不是 { $number }:
typed-address-empty = 没有给出地址
typed-address-bad-port = { $port } 不是端口，端口是 1 到 65535 的数
typed-address-unclosed = 以 [ 开头的地址必须以 ] 结束
typed-address-scheme = { $scheme }:// 属于网址，不是这里的地址
typed-address-not-a-host = “{ $host }”既不是主机名也不是 IP 地址
typed-address-not-an-onion = { $host } 不是 onion 地址，onion 地址在 .onion 前有 { $letters } 个
    字母和数字

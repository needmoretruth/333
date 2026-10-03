### Reading an address somebody typed: what was wrong with one, and how one is written.

typed-address-refused = { $why }。地址是 host:port，如 node.example:3333；邀請是 333: 加
    地址，如 333:node.example:3333。
typed-address-refused-announce = { $why }。地址是 host:port，如 node.example:3333。
typed-address-refused-bind = { $typed } 不是可監聽的地址。它應是 IP 地址加埠，如 0.0.0.0:3333。
    只給地址就監聽 3333 埠，只給 :port 就監聽所有地址。

typed-address-no-tag = 邀請以 333: 開頭
typed-address-too-long = 邀請最多 { $most } 個字元，這個有 { $length } 個
typed-address-not-canonical = 我們每一位都是一個地方，只有一種寫法，這張邀請是 { $canonical }
typed-address-wrong-tag = 邀請以 333: 開頭，不是 { $number }:
typed-address-empty = 沒有給出地址
typed-address-bad-port = { $port } 不是埠，埠是 1 到 65535 的數
typed-address-unclosed = 以 [ 開頭的地址必須以 ] 結束
typed-address-scheme = { $scheme }:// 屬於網址，不是這裡的地址
typed-address-not-a-host = “{ $host }”既不是主機名也不是 IP 地址
typed-address-not-an-onion = { $host } 不是 onion 地址，onion 地址在 .onion 前有 { $letters } 個
    字母和數字

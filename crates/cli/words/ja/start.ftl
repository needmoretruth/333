### `333 start` and `333 restart`: running this node in the background.

start-no-node = { $home } にはまだノードがありません

start-no-node-next = `333 join <invitation>` で、333 を動かしている人の招待状を
    使って参加します。`333 begin` で一人で始めます。

start-in-a-terminal = すでに端末で動いています。そこで止めるか `333 stop` で止めると、
    `333 start` でバックグラウンドで動かせます。
    .keyword = 動作

start-already = すでにバックグラウンドで。
    .keyword = 動作

start-started = バックグラウンドで。今も、再起動後も。様子は `333 status`、
    止めるには `333 stop`。
    .keyword = 開始

start-elsewhere = このマシンのバックグラウンドサービスは { $other } のノードを
    動かしています。`333 service uninstall` で取り除き、もう一度
    `333 start` を。

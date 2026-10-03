### `333 status`: where the others are, as far as this node knows.

status-known-another-copy = 这个名字的另一个副本

status-known-sighting = 一条用这个节点的密钥签署、却不是这个节点做出的声明说，它在
    纪元 { $said_in } 位于 { $address }。
    它在纪元 { $epoch } { $arrived }到达。

status-known-either = 要么这个目录被复制并启动了副本，要么别人拿到了密钥。同名的两个
    节点，在任何一个被问到的每个纪元都会互相矛盾。停掉其中一个；
    搬迁节点要用 `333 pack`。这里不会替你停掉任何一个副本：旧声明
    谁都能重放，一个看到它就停止的节点，可以被任何持有其密钥副本的
    人关掉。

status-known-nowhere = 还没有可敲的门。交给 `333 ping` 或 `333 join` 的邀请会被保存，
    之后节点就去敲那里。
    .keyword = 已知

status-known-held = { $held ->
       *[other] { $held } 个地址
    }，按最初听说的地方分
    .keyword = 已知

status-known-by-hand = 手动
status-known-this-network = 本网络
status-known-meeting-point = 汇合点
status-known-from-us = 来自 { $peers } 位同伴
status-known-not-noted = 未记录

status-known-where-heard = 从哪里听说，与那里有没有人应答无关。
status-known-sources-lists = `333 status --sources` 会列出它们。

status-known-sources = 来源
status-known-nobody-answered = 这里还没有人应答过
status-known-at = 位于
status-known-first = 最早
status-known-last = 最后
status-known-from-before = 在这个节点开始记录来源之前就持有的
status-known-when = { $from }，纪元 { $epoch }

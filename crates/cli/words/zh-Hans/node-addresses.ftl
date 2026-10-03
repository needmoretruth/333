### Where the others are, and another copy of this node's name.

node-addresses-reading = 正在读取地址
node-addresses-keeping = 正在保存地址
node-addresses-reading-own = 正在读取这个节点的地址
node-addresses-keeping-own = 正在保存这个节点的地址

node-addresses-another-copy = 这个名字的副本在外面。一条用这个节点的密钥签署、却不是这个
    节点做出的声明说，它在纪元 { $said_in } 位于 { $address }。
    它{ $from }到达。要么这个目录被复制并启动了副本，要么别人拿到了
    密钥。同名的两个节点，在任何一个被问到的每个纪元都会互相矛盾。
    停掉其中一个；搬迁节点要用 `333 pack`。在你决定之前，这个节点
    继续运行。
    .keyword = 副本

node-addresses-unread = 无法读取各地址来源的记录，所以重新开始一份。这个节点做的任何
    决定都不读它。
    .keyword = 无法读

node-addresses-copies = { $copies ->
       *[other] { $copies } 条用这个节点的密钥签署、却不是它做出的声明
    }在窗口期内到达了它。这个名字的另一个副本一直在运行。
    `333 status` 会显示它自称在哪里。
    .keyword = 副本

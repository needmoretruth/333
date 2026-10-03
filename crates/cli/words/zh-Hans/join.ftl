### `333 join`: asking a node that holds the file to hand it over.

join-name = { $name }
    .keyword = 名字

join-knocking = { $address }
    .keyword = 敲门

join-silence = 在 { $address } 没有联系到任何人。这不能证明 333 已经结束。本客户端
    带的是文件的哈希而不是文件：只能从持有它的人那里进来。
    .keyword = 沉默

join-knocking-on = 正在敲 { $address } 的门
join-exchanging = 正在交换心跳
join-asking = 正在索取文件
join-no-answer = { $seconds } 秒后 { $address } 仍无应答

join-given = 来自 { $giver }
    .keyword = 收到

join-joined = 纪元 { $epoch }
    .keyword = 加入

join-holding = 文件，并且可以传下去
    .keyword = 持有

join-roll = { $members } 位同伴
    .keyword = 名册

join-counted = 从纪元 { $epoch } 起，一个纪元也不会提前：隔两个边界，视你在本纪元
    何时到达，约 { $least } 到 { $most } 分钟后。在那之前，有问必答。
    这段时间得到的见证，就是你曾在这里的全部证明。
    .keyword = 计数

join-vigil = 从现在起用 `333 start` 让它一直运行。没人能联系到的节点无法被
    见证，而这一段只会被见证一次，或者永远不会。
    .keyword = 节点

join-already-given = 这个节点已持有文件，由 { $giver } 在纪元 { $epoch } 交给它。
    没有要索取的，也没有索取。
join-same-handover = 这个节点和 { $peer } 已在纪元 { $epoch } 传过文件。同一纪元
    再交回来，只是从另一边读同一次交接，不会接纳任何人，所以没有索取。

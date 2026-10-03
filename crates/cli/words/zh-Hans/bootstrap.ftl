### `333 bootstrap`: beginning a line of your own, when there is nobody to join.

bootstrap-name = { $name }
    .keyword = 名字

bootstrap-vigil = `333 start` 让它运行，它就会应答。
    .keyword = 节点

bootstrap-already-has-it = 这个节点已经有文件了，无需开始。

bootstrap-stop = { $already ->
       *[other] 已有 { $already } 位同伴
    }在 { $meet } 说明了在哪里能找到他们。
    现在独自开始，会无故在他们的谱系旁另起一条谱系。请在浏览器中
    打开 { $board }，选一张邀请，用它运行 `333 join`。
    如果读完这些仍要开始，请加 `--anyway`。
    .keyword = 停止

bootstrap-not-the-file = 收到的不是那个文件

bootstrap-begun = 文件在这个节点的目录中，这个节点是它自己谱系的开端。没有人
    为交接签名，因为没有人交接过，任何读这个节点记录的人都能看到。

    这是创始者的位置，不是普通的位置。名册接纳收到文件的人，所以
    没有从任何人那里收到文件的节点不在任何名册上：没有人会来问它，
    它也永远不会被抽中去问别人。它仍可以去找被抽中来问它的人，
    以此得到见证。

    之后你把文件交给谁，谁就按普通方式加入，你们双方都签名，
    并从那一刻起被计数。
    .keyword = 已开始

bootstrap-reading-the-board = 正在读取 { $place } 的公告板

bootstrap-asking = 向 { $meet } 索取文件
    .keyword = 索取

bootstrap-asking-for-the-file = 正在向 { $meet } 索取文件

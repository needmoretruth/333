### Where this node lives, and whether it still lives here.

-dwelling-would-be-one = 会让同一个
-dwelling-anywhere-this = 在任何地方解包，这条命令

dwelling-made-at = 创建于
dwelling-unpacked-at = 解包于
dwelling-moved-to = 据称移到了
dwelling-found-at = 本客户端首次打开于

dwelling-writing = 正在写入 { $file }
dwelling-putting-in-place = 正在放置 { $file }
dwelling-resolving = 正在解析 { $dir }

dwelling-elsewhere = 这个节点曾{ $how } { $was }，
    现在位于 { $now }。
    如果目录只是移动或改名，那就没事。如果是复制的，而原来的仍在
    运行，那就是同一个名字在两个地方，彼此的记录会互相矛盾。
    等只剩一个时，在这里说明：{ $settle }
    .keyword = 住处

dwelling-unread-moment = 无法读取的时刻
dwelling-unread-file = 名字无法读取的文件

dwelling-packed = 这个节点在 { $at } 为搬家打包进了 { $into }。
    那个文件在哪里解包，它就住在哪里。在这里也运行它，{ -dwelling-would-be-one }
    名字出现在两个地方，所以 { $home } 中没有任何东西会作为它行动。

    如果放弃了搬家，那个文件也没有{ -dwelling-anywhere-this }
    会把它放回原处：{ $undo }

dwelling-unmarking = 正在去掉这个节点已打包的标记

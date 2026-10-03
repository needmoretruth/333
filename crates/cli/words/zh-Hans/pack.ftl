### `333 pack`: writing this node into one file, to be carried to another machine.

pack-name-the-file = 请指定打包这个节点的文件：333 pack <FILE>

pack-no-node = { $root } 里没有可打包的节点。什么都没写。

pack-already-exists = { $file } 已存在。{ -pack-never-over }
-pack-never-over = 打包只写新文件，从不覆盖旧文件；换个名字。

pack-not-marked = 已写入 { $file }，但这个目录无法标记为已打包。{ -pack-until-it-is }
-pack-until-it-is = 在标记之前，这个节点同时住在两处：{ -pack-delete-that-file }
-pack-delete-that-file = 在这里运行任何东西之前，删掉那个文件。

pack-creating = 正在创建 { $file }

pack-name = { $name }
    .keyword = 名字

pack-record-none = 还没有
    .keyword = 记录

pack-record = { $epochs ->
       *[other] { $epochs } 个纪元，一起带走
    }
    .keyword = 记录

pack-witnessed = { $statements ->
       *[other] 其他密钥为它签署的声明 { $statements } 条，一起带走
    }
    .keyword = 见证

pack-holding = 文件，一起带走
    .keyword = 持有

pack-onion-key = 它 onion 地址的密钥，所以地址也一起带走
    .keyword = 隐藏

pack-carrying = 这个节点，写进 { $file }。
    那个文件就是这个节点：谁持有它，谁就能以这个名字应答。带走、
    解包，然后删掉；它不是要留着的备份。它没有加密，因为密码会是
    又一样可能丢的东西，而丢了密码就和丢了文件一样会丢掉名字。
    和这个目录一样，只有你能读它。
    .keyword = 携带

pack-packed = { $bytes } 字节。
    { $root } 中的任何东西都不会再作为这个节点行动。
    .keyword = 已打包

pack-next = 在另一台机器上：333 unpack { $carried }
    如果放弃搬家：{ $undo }
    .keyword = 下一步

pack-not-packed = 这个节点没有打包，所以 { $root } 里没有可撤销的
    .keyword = 这里

pack-restored = 这个节点又住回了 { $root }。
    它被打包进的文件仍是这个名字。如果它在别处解包过，两者中的一个
    必须先删掉，才能运行任何一个；如果没有，就删掉 { $file }
    .keyword = 已恢复

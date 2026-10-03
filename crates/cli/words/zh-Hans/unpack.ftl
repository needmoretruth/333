### `333 unpack`: putting a packed node into this machine's node directory.

-unpack-nothing = 没有解包任何东西。
-unpack-so-nothing = 所以没有解包任何东西。

unpack-kept = 这个目录里已经住着一个节点。要在它旁边解包，用 --data-dir 给它
    一个自己的目录。

unpack-seed-in = { $file } 中的{ $seed }

unpack-not-one-node = 该文件自称包含 { $claimed }，里面的密钥却是 { $name }。{ -unpack-not-one }
-unpack-not-one = 它不是同一个节点，没有解包任何东西。

unpack-taken = 另一个 333 占用了解包的目标目录。{ -unpack-nothing }

unpack-elsewhere = 333 --data-dir <另一个目录> unpack { $file }

unpack-could-not-open = { $target } 里已经住着一个节点，而且{ -unpack-could-not-be-opened } 要在它旁边解包：{ $elsewhere }
-unpack-could-not-be-opened = 无法打开它看里面有什么。{ -unpack-nothing }

unpack-no-record = 还没有记录
unpack-epochs-of-record = { $epochs ->
       *[other] { $epochs } 个纪元的记录
    }
unpack-holding = 持有文件
unpack-not-holding = 未持有文件

unpack-occupied = { $target } 里已经住着一个节点：
    { $name }，{ $epochs }，{ $holding }。
    在上面解包会永远丢掉这一切，{ -unpack-so-nothing }
    要在它旁边解包，给它一个自己的目录：
    { $elsewhere }

unpack-holds-files = { $target } 里有文件但没有节点。{ -unpack-its-own } 解包到别处：{ $elsewhere }
-unpack-its-own = 节点要解包到它自己的目录，{ -unpack-so-nothing }

unpack-opening-the-record = 正在打开记录
unpack-torn = 该文件中的记录残缺，所以不是完整的节点。{ -unpack-nothing }
unpack-reading-the-record = 正在读取记录
unpack-does-not-verify = 该文件中的记录无法通过校验。{ -unpack-nothing }
unpack-another-key = 该文件中的记录由另一把密钥写成。{ -unpack-nothing }

unpack-not-a-place = { $target } 不是可以放节点的目录
unpack-making-room = 正在 { $target } 腾出位置
unpack-putting = 正在把节点放进 { $target }

unpack-name = { $name }
    .keyword = 名字

unpack-record-none = 还没有
    .keyword = 记录

unpack-record = { $epochs ->
       *[other] { $epochs } 个纪元，已校验
    }
    .keyword = 记录

unpack-holding-the-file = 文件
    .keyword = 持有

unpack-onion-key = 它 onion 地址的密钥，所以地址也一起来了
    .keyword = 隐藏

unpack-unpacked = 放进 { $target }，
    来自 { $packed } 打包的文件。
    现在它就是这个节点，那个文件也是：删掉 { $file }
    .keyword = 已解包

unpack-next = { $serve }
    .keyword = 下一步

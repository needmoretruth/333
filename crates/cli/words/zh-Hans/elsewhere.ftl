### What a command says when another 333 already has this node's directory.

elsewhere-the-vigil = 在这个目录中运行的节点
elsewhere-the-vigil-by-number = 在这个目录中运行的节点（进程 { $pid }）
elsewhere-another = 另一个 333
elsewhere-another-by-number = 另一个 333（进程 { $pid }）

elsewhere-done = 由 { $vigil } 执行了。
    .keyword = 完成

elsewhere-failed = { $vigil } 没有执行。
    .keyword = 失败

elsewhere-finding-its-name = { $who } 还在寻找这个节点的名字。
    有了名字后请再运行一次。
    .keyword = 占用

elsewhere-already-keeping = { $who } 已在这里运行，一个目录就是一个节点。
    可以从这里吩咐它：`333 say 7`、`333 join <invitation>`、
    `333 tell 'tor on'`、`333 stop`。第二个节点需要自己的目录，
    用 --data-dir 指定。
    .keyword = 占用

elsewhere-keeping = { $who } 正在这里运行，而且
    { $why }
    .keyword = 占用

elsewhere-nobody-to-tell = 这个目录中没有运行的节点，所以没有可以吩咐的对象。
    用 `333 start` 或 `333 run` 运行它之后，这条命令就能用了。
    .keyword = 无人

elsewhere-busy = { $who } 占着这个节点的目录，
    但它不是可以接手这件事的运行中节点。这里没有读写任何东西。
    等它结束后请再运行一次。
    .keyword = 占用

elsewhere-busy-cannot-be-handed = { $who } 占着这个节点的目录。
    在这个系统上，还无法从另一个终端把东西交给运行中的 333，
    所以这里没有读写任何东西。请在它的屏幕里 `:` 之后输入，
    或者停掉它再运行一次。
    .keyword = 占用

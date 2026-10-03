### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph. Between two Chinese or Japanese letters a break stays a break.

help-about = 333 的一个节点。被问时应答，保存自己的记录，并把文件传下去。

help-id = 显示这个节点的名字，首次运行时创建

help-bootstrap = 没有人能把文件交给你时，开始一条新的谱系
help-bootstrap-long = 没有人能把文件交给你时，开始一条新的谱系。

    通常的加入方式是用邀请运行 `333 join`。这条命令先查看汇合点，
    那里有人就拒绝。没有人时，它获取文件，用本客户端带的哈希
    校验，然后写下。你的节点就成为自己谱系的创始者，开端没有任何
    人的签名，读它记录的任何人都能看到。

help-serve = 在这个终端运行这个节点，直到你停止它
help-serve-long = 在这个终端运行这个节点，直到你停止它。

    它应答心跳和提问，交换所知，并在每个纪元去问抽中由它提问的
    节点。在终端中它会打开屏幕；`q`、Ctrl-C 或在另一个终端运行
    `333 stop` 都能停止它。`333 start` 在后台做同样的事。

help-serve-long-light = 在这个终端运行这个节点，直到你停止它。

    它应答心跳和提问，交换所知，并在每个纪元去问抽中由它提问的
    节点，一行一行地输出。Ctrl-C 或在另一个终端运行 `333 stop` 都能
    停止它。`333 start` 在后台做同样的事。

help-say = 每个纪元说一次 333 中的一个。传出去的是编号

help-status = 显示这个节点是否在运行、别人在哪里能找到它，以及有多少同伴在应答

help-join = 用邀请从持有文件的节点那里接收文件

help-languages = 列出语言，或为这个节点的所有命令保存一种语言

help-ping = 联系另一个节点，与它交换一次心跳

help-pack = 把这个节点写进一个文件，带到另一台机器
help-pack-long = 把这个节点写进一个文件，带到另一台机器。

    全部带走：名字、记录、别人为它签署的内容、文件，以及它 onion
    地址的密钥。之后这个目录拒绝运行它，名字就不会出现在两个
    地方。文件没有加密：谁持有它，谁就是这个节点。带走、解包、
    删除。

help-unpack = 把打包的节点放进这台机器的节点目录
help-unpack-long = 把打包的节点放进这台机器的节点目录。

    已有节点的地方会拒绝。文件读完、密钥和记录校验通过之前，
    什么都不写。

help-moved = 说明这个节点的目录是移动或改名的，不是复制的
help-moved-long = 说明这个节点的目录是移动或改名的，不是复制的。

    节点发现自己在新位置时，每次运行都会提醒，直到输入这条命令，
    因为原件仍在运行的副本会让一个名字出现在两个地方。

help-tell = 用屏幕上的说法给运行中的节点下命令
help-tell-long = 用屏幕上的说法给运行中的节点下命令。

    `tor on`、`tor off`、`bridge <line>`、`helper <program>`，以及
    屏幕在 `:` 之后接受的所有词。运行中的节点执行它，回应显示在
    这里。`say`、`join`、`ping`、`begin`、`status` 和 `stop` 不用
    这条命令也能传给运行中的节点。

help-tell-light = 给运行中的节点下命令
help-tell-long-light = 给运行中的节点下命令。

    `tor on`、`tor off`、`bridge <line>` 和 `helper <program>`。
    运行中的节点执行它，回应显示在这里。`say`、`join`、`ping`、
    `begin`、`status` 和 `stop` 不用这条命令也能传给运行中的节点。

help-service = 直接管理后台服务（`start` 和 `stop` 用的就是它）
help-service-long = 直接管理后台服务（`start` 和 `stop` 用的就是它）。

    你不要求就什么都不装，写的每个文件、跑的每条命令都会随时
    显示，`333 service uninstall` 会全部移除。

help-service-install = 用这些 run 选项安装后台服务并启动
help-service-install-long = 用这些 run 选项安装后台服务并启动。

    服务按给定的选项原样为这个节点的目录运行 `333 run`，旁边每小时
    检查一次，节点停了就在这台机器上提示。`333 start` 不带选项做
    同样的事。

help-service-uninstall = 停止后台服务并移除它装的所有东西

help-service-status = 服务管理器的说法、节点最后一次报告醒着的时间，以及最后几行

help-service-check = 节点停了就在这台机器上提示。服务每小时运行一次；一切正常时
    什么都不说

help-data-dir = 存放这个节点全部所有物的目录：名字，以及使用 Tor 时 Tor 的状态

help-timeout = 每个联网步骤等待的秒数
help-timeout-long = 每个联网步骤等待的秒数。

    这是上限，不是延迟。按启动 Tor 来定，这是唯一可能要几分钟的
    步骤。

help-dangerously-trust-directory-permissions = 接受这台机器上其他人能进入的目录
help-dangerously-trust-directory-permissions-long = 接受这台机器上其他人能进入的目录。

    目录里有这个节点名字的唯一副本，所以权限宽松的目录默认会被
    拒绝。这是给测试目录和属主奇怪的容器用的。

help-keep-everything = 永久保存所有声明，而不只是判定所用的窗口期
help-keep-everything-long = 永久保存所有声明，而不只是判定所用的窗口期。

    这不会改变任何人的状态：每条声明无论存在哪里，验证结果都一样。

help-bridges = 一行网桥，用于封锁了 Tor 常规入口的网络
help-bridges-long = 一行网桥，用于封锁了 Tor 常规入口的网络。

    每座拿到的网桥给一次，照拿到时的样子原样给出。这里不会获取
    网桥：网桥是人有意分发的，为的是没有哪份清单能被直接收集并
    封锁。

help-bridge-helper = 讲混淆网桥协议的程序，按名字或路径
help-bridge-helper-long = 讲混淆网桥协议的程序，按名字或路径。

    只有网桥行需要、且路径上的不是 `lyrebird` 时才需要。不随附，
    因为冻结的副本很快就会过时。

help-language = 使用的语言标签：`ko`、`es`、`zh-Hant`
help-language-long = 使用的语言标签：`ko`、`es`、`zh-Hant`。

    不指定时，先看 `THE333_LANGUAGE`，再看 `333 language <TAG>`
    保存的语言，最后是英语。不使用系统语言。`333 language` 列出有
    词语的语言，在 `<data-dir>/words/<tag>/` 放一个目录的词库，
    不用编译就能加一种语言。333 的词语本身永不翻译。

help-count-in = 用十进制、十二进制或 twelve-ascii 计数
help-count-in-long = 用十进制、十二进制或 twelve-ascii 计数。

    显示的每个数都用它写，输入的每个数都按它读：十二进制的
    `say 238` 就是十进制的 `say 332`。名字、地址、端口和版本永不
    换算，网上传的内容也不变。不指定时，先看 `THE333_COUNT_IN`，
    再用十进制。

help-bootstrap-meet = 独自开始之前去哪里找人

help-bootstrap-anyway = 即使已有人也开始

help-serve-bind = 监听的地址和端口

help-serve-tor = 同时开一个 onion 地址，让别人不知道位置也能联系到这个节点。
    唤醒 Tor 需要几秒到几分钟

help-serve-no-direct = 完全不开套接字。只能与 --tor 一起用；你的地址完全不上网

help-serve-announce = 告诉其他节点用来联系这个节点的地址
help-serve-announce-long = 告诉其他节点用来联系这个节点的地址。

    套接字自己说不出时需要：监听所有网卡时，或在转发端口的设备
    后面时。

help-serve-no-mdns = 不在本地网络中宣告这个节点在这里
help-serve-no-mdns-long = 不在本地网络中宣告这个节点在这里。

    否则宣告出去的是这台机器上有东西讲 333 以及端口，不是这个
    节点的名字。同一个家里的两个节点就是这样不用邀请互相找到的。

help-serve-no-router = 不请路由器把端口转给这台机器
help-serve-no-router-long = 不请路由器把端口转给这台机器。

    家用路由器会丢弃里面没人请求过的东西，直到里面的程序通过
    UPnP-IGD、PCP 或 NAT-PMP 请它转发端口。这会改变网络，所以发生
    时会显示。`--no-upnp` 是它的旧名字。

help-serve-meet = 去哪里找没人介绍给它的节点
help-serve-meet-long = 去哪里找没人介绍给它的节点。

    一个固定地址，存放关于节点在哪里的签名声明。在那里读到的
    一切都在这里验证。

help-serve-no-meet = 完全不用汇合点
help-serve-no-meet-long = 完全不用汇合点。

    这样只有拿到邀请的人和这个网络上的节点能联系到这个节点，
    其他人都不行。

help-serve-plain = 逐行输出，而不画屏幕
help-serve-plain-long = 逐行输出，而不画屏幕。

    不在终端时总是逐行输出；这个选项让终端里也这样。

help-serve-plain-light = 逐行输出，这个版本总是如此
help-serve-plain-long-light = 逐行输出，这个版本总是如此。

    这个版本没有屏幕。接受这个选项，是为了同一行命令在两个版本
    里都能用。

help-say-index = 哪一个，0 到 { $last }，按计数的进制（--count-in）输入。词语还没写

help-status-sources = 列出这个节点持有的所有地址：属于谁、最早在哪里何时听说、
    最后在哪里

help-status-json = 这个节点观察到的内容，以 JSON 供程序读取。不含任何地址或端口

help-join-address = 已持有文件的人给的邀请（`333:host:port`）

help-ping-address = 邀请（`333:host:port`）或地址：`host`、`host:port`、
    `[::1]:port` 或 `某个.onion`（经由 Tor）

help-pack-file = 要写的文件。它必须还不存在

help-pack-undo = 搬家放弃时，撤销这里的打包
help-pack-undo-long = 搬家放弃时，撤销这里的打包。

    仅当文件从未在任何地方解包时：若解包过，这会变成两个。

help-unpack-file = `333 pack` 写出的文件

help-tell-order = 命令，按在屏幕里输入的样子

help-tell-order-light = 命令，写法如 `tor on` 或 `bridge <line>`

help-service-install-flags = `run` 的选项，按你在它后面输入的样子

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = 显示帮助
help-print-help-more = 显示帮助（用 '--help' 看更多）
help-print-help-summary = 显示帮助（用 '-h' 看摘要）
help-print-version = 显示版本
help-print-this = 显示这条消息或给定子命令的帮助
help-print-for = 显示子命令的帮助

help-start = 在后台运行这个节点，现在和每次重启后

help-stop = 停止这个节点，重启后也保持停止

help-restart = 停止这个节点，再在后台运行它

help-logs = 显示这个节点在后台运行时写的最后几行

help-logs-follow = 新行一来就继续显示（由 systemd 保存时）

help-invite = 显示别人通过这个节点加入所用的邀请

help-status-all = 显示这个节点知道的一切，并说明每部分的含义

help-languages-tag = 要保存的语言标签：`ko`、`en`。`en` 回到英语

help-start-example = 示例：333 start

help-stop-example = 示例：333 stop

help-restart-example = 示例：333 restart

help-status-example = 示例：333 status --all

help-logs-example = 示例：333 logs -f

help-id-example = 示例：333 name

help-invite-example = 示例：333 invite

help-bootstrap-example = 示例：333 begin

help-serve-example = 示例：333 run --tor

help-say-example = 示例：333 say 7

help-join-example = 示例：333 join 333:192.0.2.7:3333

help-languages-example = 示例：333 language zh-Hans

help-ping-example = 示例：333 ping 333:192.0.2.7:3333

help-pack-example = 示例：333 pack node.333

help-unpack-example = 示例：333 unpack node.333

help-moved-example = 示例：333 moved

help-tell-example = 示例：333 tell tor on

help-service-example = 示例：333 service status

help-service-install-example = 示例：333 service install --tor

help-service-uninstall-example = 示例：333 service uninstall

help-service-status-example = 示例：333 service status

help-service-check-example = 示例：333 service check

help-start-flags = `run` 的选项，按你在它后面输入的样子。之后每次启动都沿用，
    直到给出别的选项

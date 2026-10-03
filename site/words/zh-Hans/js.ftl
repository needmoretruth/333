# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = 复制
js-copied = 已复制
js-selected = 已选中
js-state-awake = 本站节点醒着
js-state-not-running = 本站节点没有运行
js-in-hours = { $h } 小时 { $m } 分钟后
js-in-minutes = { $m } 分钟后
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = 本谱系第 { $n } 个纪元

## The network

js-network-state-founder = 不在任何名册上
js-network-state-ok = 本纪元应答
js-network-state-quiet = 本纪元沉默
js-network-state-later = 从之后的纪元起计数
js-network-state-seen = 见过，不在名册上
js-network-awake = 醒着
js-network-not-running = 没有运行
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · 你的
js-network-find-bad = 节点的名字是十六进制，至少输入前 6 个字符。
js-network-find-none = 本站节点没见过叫这个名字的节点。
js-network-find-many = 有 { $count } 个节点以此开头。请多输入一些名字。
js-network-find-marked = 已在此设备上标记为你的。
js-network-select = 选择一个节点，查看本站节点对它的了解。
js-network-role-founder = 本谱系的创始者
js-network-role-site = 本站节点
js-network-role-yours = 你的，在此设备上
js-network-role-none = 网络上的一个节点
js-network-col-name = 名字
js-network-col-state = 本纪元
js-network-col-given = 收到文件
js-network-col-counted = 计数起点
js-network-col-answered = 上次应答
js-network-col-said = 已说
js-network-col-reached = 到达方式
js-network-row-said = 本纪元已说
js-network-row-handed = 把文件交给了
js-network-row-testimony = 见证
js-network-given-by = 纪元 { $epoch }，来自 { $sponsor }
js-network-given-founder = 无人交给。它开始了这条谱系。
js-network-given-none = 不在名册上
js-network-epoch = 纪元 { $epoch }
js-network-epoch-now = { $epoch }（本纪元）
js-network-epoch-ago = { $epoch }（{ $ago }前）
js-network-more = 另有 { $count } 个在下表中
js-network-nothing = 无
js-network-reach-direct = 直接
js-network-reach-tor = 经由 Tor
js-network-reach-tor-short = Tor
js-network-reach-unknown = 未知
js-network-testimony = 被问 { $asked } 次，问了 { $asking } 次（最近 3 个纪元）
js-network-copy-name = 复制名字
js-network-select-name = 选中上面的名字
js-network-mine = 这是我的节点
js-network-tag-founder = 创始者
js-network-tag-site = 本站
js-network-tag-yours = 你的
js-network-empty = 本站节点还没见过其他节点。
js-network-this-node = 这个节点
js-network-yes = 是
js-network-no = 否
js-network-none = 无

## Where we are

js-map-watch = 实时查看
js-map-stop = 停止查看
js-map-read-at = 读取于 { $read_at } UTC。
js-map-unreadable = 刚才无法读取公告板。
js-map-tor = Tor
js-map-nowhere = 无法定位的地方
js-map-nobody = 没有人说自己在哪里。
js-map-all = 所有说明位置的
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = 在网络上，但没说在哪里
js-map-dot = { $count } 个节点

## The board

js-board-said = 纪元 { $epoch } 由 { $node } 说出
js-board-site = 本站节点
js-board-tor = 经由 Tor

## Take the program

js-start-machine-linux-x86_64 = x86-64 上的 Linux
js-start-machine-linux-aarch64 = 64 位 ARM 上的 Linux
js-start-machine-linux-armv6 = 32 位 ARM 上的 Linux
js-start-machine-macos-aarch64 = Apple 芯片的 Mac
js-start-machine-macos-x86_64 = Intel 芯片的 Mac
js-start-machine-windows-x86_64 = Windows
js-start-phone = 这看起来是手机或平板，而这个程序是给一直开着的电脑用的。请在这里选择那台电脑。
js-start-unknown = 这个浏览器没有说明它运行在什么上。请在这里选择你的机器。
js-start-sure = 这个浏览器说它运行在{ $machine }上，所以这里选了它。
js-start-mac = 这个浏览器说它在 Mac 上，但没说是哪种芯片，所以这里选了 Apple 芯片。安装程序会直接询问机器本身。
js-start-linux = 这个浏览器说它在 Linux 上，但没说是哪种处理器，所以这里选了 x86-64。安装程序会直接询问机器本身。
js-start-chosen = 已在上方选择

## The story on the home page, drawn

js-story-file = 333.txt · 3 字节
js-story-gave = 我把它交给了你
js-story-received = 我从你那里收到了它
js-story-signed = 已签名
js-story-minutes = 333 分钟
js-story-epochs = 333 个纪元
js-story-now = 现在
js-story-answering = 应答中
js-story-roll = 名册上
js-story-years = { $years } 年

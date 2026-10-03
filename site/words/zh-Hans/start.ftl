# Take the program: the installers, every file, and the first commands.

start-meta-title = 获取程序 · 333
start-meta-description = 在 Linux、macOS、Windows 或树莓派上安装 333 客户端，用该版本的 SHA256SUMS 校验，然后接收文件。

start-heading = 获取程序
start-lede = 333 是一个小程序，你让它在一台用不上的电脑上一直运行，每当它的另一个副本来敲门，它就在那里应答。
start-pick-file = 这台机器用的文件是 <code data-name="standard">333-x86_64-linux</code>。Light 是没有终端屏幕的同一个客户端，文件是 <code data-name="light">333-light-x86_64-linux</code>。两者都自带 Tor。
start-machine = 你的机器
start-pick-linux-x86_64 = Linux，台式机或服务器
start-pick-linux-aarch64 = 64 位的树莓派 3、4、5，或其他 64 位 ARM Linux
start-pick-linux-armv6 = 树莓派 Zero，或任何 32 位的树莓派
start-pick-macos-aarch64 = Mac，Apple 芯片
start-pick-macos-x86_64 = Mac，Intel
start-pick-windows = Windows
start-form = 形态
start-standard = Standard
start-light = Light
start-unix-title = 在 Linux 或 Mac 上
start-unix-installer = 在终端里运行。安装程序为它所在的机器挑选文件，用该版本的 <code>SHA256SUMS</code> 校验，放到 <code>~/.local/bin/333</code>。它不启动任何东西，也不要密码。<a href="/install.sh">先读一读</a>，它很短。
start-unix-light = 要装 Light，把结尾的 <code>sh</code> 换成 <code>sh -s -- --light</code>。
start-by-hand = 或者不用安装程序，手动来
start-unix-by-hand = 同样的步骤，一步一步。<code>grep</code> 那一行会在移动任何东西之前打印 <code>OK</code>，没打印的话后面什么都不会运行。
start-unix-mac = 以上是 x86-64 上的 Linux。在 Mac 上用 <code>shasum -a 256</code> 代替 <code>sha256sum</code>，文件用下表中 Mac 的那个；其他地方用表中的名字。
start-windows-title = 在 Windows 上
start-windows-installer = 在 PowerShell 里运行。安装程序下载 Windows 文件，用该版本的 <code>SHA256SUMS</code> 校验，放到 <code>%LOCALAPPDATA%\Programs\333\333.exe</code>。它不启动任何东西，不需要管理员，只有加上 <code>-AddToPath</code> 才会把那个文件夹加入 PATH。<a href="/install.ps1">先读一读</a>，它很短。
start-windows-light = 要装 Light，改为这样运行。<code>-AddToPath</code> 同样加在末尾。
start-windows-by-hand = 同样的步骤，一步一步。只有文件的哈希与 <code>SHA256SUMS</code> 给出的一致，才会把它放到位，最后一行会说明结果。
start-files-title = 所有文件
start-files-intro = 给上面猜错了的人，或者更想用浏览器下载文件的人。Standard 有终端屏幕，Light 没有，两者都自带 Tor。大小从 Mac 上的 15 MB 到 64 位 ARM 上的 24 MB。
start-files-machine = 你的机器
start-files-forms = Standard，然后 Light
start-row-linux-x86_64 = Linux，台式机或服务器
start-row-linux-aarch64 = 64 位的树莓派 3、4、5，以及其他 64 位 ARM Linux
start-row-linux-armv6 = 树莓派 Zero，或任何 32 位的树莓派
start-row-macos-aarch64 = Mac，Apple 芯片
start-row-macos-x86_64 = Mac，Intel
start-row-windows = Windows
start-files-after = 用浏览器下载的文件，同样要用 <a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a> 校验、设为可执行、放进 PATH，这正是上面的命令所做的。这里没有任何文件用开发者证书签名，所以 Mac 或 Windows 可能拒绝打开从浏览器来的文件，直到你另行允许。如果都不合适，或者你不想运行别人构建的文件，<a href="https://github.com/needmoretruth/333#install">代码仓库</a>里写了如何在 Ubuntu、Debian、Fedora、Arch、macOS 和 Windows 上构建。
start-then-title = 然后，按顺序
start-step-1 = <b>接收文件。</b>本站节点全天醒着：
start-step-1-after = 第一次 <code>join</code> 还会生成你节点的名字，一把名字以 333 开头的密钥，需要一点时间。<a href="{ $base }/333">公告板</a>上的任何地址都一样能用。
start-step-2 = <b>启动它。</b>
start-step-2-after = 从现在起它在后台运行，注销后、重启后都是。如果它显示 <code>shut</code>，说明你的路由器把别人挡在外面：<code>333 start --tor</code> 不需要改路由器。
start-step-3 = <b>查看它。</b><code>333 status</code> 说明它是否在运行、别人在哪里能联系到它，以及我们中有多少在应答。<code>333 logs</code> 显示它写下的内容。
start-step-4 = 随时可以<b>停止它</b>：<code>333 stop</code>。重启后它保持停止，直到你再次运行 <code>333 start</code>。
start-watch = 想看它工作，<code>333 run</code> 会在这个终端里带着屏幕运行它；<code>q</code> 停止它。只输入 <code>333</code> 会说明你的节点处于什么状态、接下来输入什么，<code>333 help</code> 列出所有命令。
start-begin = 如果公告板上没有人、本站节点也不在了，总得有人做第一个：<code>333 begin</code> 在有人时会拒绝，否则开始一条你自己的谱系，没有人为它签名，任何读你记录的人都看得到。
start-rule = 在你启动之前，什么都不会开始。
start-rule-detail = 安装程序只放好一个文件，什么都不运行。你的节点第一次应答别人、第一次在汇合点留下地址，是在你亲自运行 <code>333 start</code> 或 <code>333 run</code> 的时候，那条命令就是你同意的时刻。

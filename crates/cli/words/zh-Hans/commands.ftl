### What the shared parts of the commands say.

commands-clock-at-zero = { $epoch }。这台机器的时钟显示 1970 年，所以这个节点以为自己在
    时间的起点。时钟校准之前，没有人会交给它任何东西，也没有人
    为它见证。
    .keyword = 纪元

commands-called-first = 选中了创建的第一个密钥。
    .keyword = 选中

commands-called = { $not_called ->
       *[other] 创建了 { $not_called } 个密钥但未选中，选中的是这一个。
    }
    .keyword = 选中

commands-torn = 从记录中删去了一条未写完条目的 { $bytes } 字节
    .keyword = 破损

commands-record = { $epochs ->
       *[other] 已应答 { $epochs } 个纪元，都不可再改
    }
    .keyword = 记录

commands-witnessed = { $statements ->
       *[other] 其他密钥对这个节点签署的声明 { $statements } 条。所属纪元
            过去后仍会保存，因为它们别的部分都不会留过窗口期。
    }
    .keyword = 见证

commands-unseen = 在任何纪元里，都没有人签署过关于这个节点的东西。向外连接可以，
    被外面连上不行，而只有后者才算数：被抽中来提问的人必须能到达
    这里。原因有两个：路由器没有把 3333 端口转给这台机器，或者地址
    没有给过任何人。`run --tor` 两者都不需要：onion 地址在任何路由器
    后面都能到达，而本客户端自带 Tor。
    .keyword = 隐藏

commands-roll-alone = 1 位同伴，就是这个节点
    .keyword = 名册

commands-roll = { $members } 位同伴
    .keyword = 名册

commands-known = { $addresses } 位同伴指明了去哪里找
    .keyword = 已知

commands-holding = 文件，并且可以传下去
    .keyword = 持有

commands-keeping = 全部，永久保存。这对这个节点没有好处：每条声明都带着自己的
    签名，无论存在哪里，验证结果都一样。没有官方档案，也没有档案员。
    .keyword = 保存

commands-ignored = { $admissions } 条无法读取的加入记录
    .keyword = 忽略

commands-learned-where = 又得知 { $addresses } 位同伴在哪里
    .keyword = 得知

commands-rejoined = 又从一个知道 { $were } 位的节点得知 { $members } 位同伴的名字。
    原来是两份计数，现在合成了一份。
    .keyword = 合并

commands-learned-names = 又得知 { $members } 位同伴的名字
    .keyword = 得知

commands-heard = { $speakers } 位同伴发言
    .keyword = 听到

commands-carried = { $statements ->
       *[other] 关于仍未结束的纪元的声明 { $statements } 条
    }
    .keyword = 携带

commands-exchange = { $node }  纪元 { $epoch }  { $clocks }  ({ $liveness })
    .keyword = 见证

commands-answered-the-challenge = 回答了我们选的挑战
commands-spoke-first = 先开了口，这只能证明它开了口

commands-clocks-together = 时钟一致
commands-clocks-ahead = 对方时钟比我们快 { $apart }
commands-clocks-behind = 对方时钟比我们慢 { $apart }
commands-hours-and-minutes = { $hours } 小时 { $minutes } 分
commands-minutes-and-seconds = { $minutes } 分 { $seconds } 秒
commands-seconds = { $seconds } 秒

commands-waking = Tor。隐藏的路需要一点时间才能打开。
    .keyword = 唤醒

commands-waking-through = Tor，经由 { $bridges ->
       *[other] { $bridges } 座网桥
    }。隐藏的路需要一点时间才能打开。
    .keyword = 唤醒

commands-no-tor = { $seconds } 秒后仍未连上 Tor
commands-starting-tor = 正在启动 Tor 客户端

# What a handover puts a signature under, read back.
commands-signed-giving = 你说：我在纪元 { $epoch } 把文件交给了你。
    对方说：我在纪元 { $epoch } 从你那里收到了文件。
    这由两只手写下，哪只手都收不回。
    .keyword = 已签名

commands-signed-taking = 对方说：我在纪元 { $epoch } 把文件交给了你。
    你说：我在纪元 { $epoch } 从你那里收到了文件。
    这由两只手写下，哪只手都收不回。
    .keyword = 已签名

commands-brimming = { $statements ->
       *[other] { $statements } 条声明一轮装不下，等下一轮
    }
    .keyword = 满溢

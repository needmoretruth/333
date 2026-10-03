### `333 status`: what this node saw of everybody else, what was said, and whether
### anybody is here.

status-name = { $name }
    .keyword = 名字

status-epoch = { $epoch }
    .keyword = 纪元

status-epoch-in-line = { $epoch }，{ $line }
    .keyword = 纪元

status-the-line = 本谱系第 { $nth }{ $kind ->
       *[other] 个
    }

status-answering = 应答中
status-silent = 沉默
status-roll = 名册

status-seen = 第一个数是这个节点持有其纪元 { $before } 或 { $now } 签名的所有人。
    这是这个节点看到的。别人看到的是别的。

status-seen-without-tor = 这个版本走不了隐藏的路，所以隐藏的同伴都不在这个数里，以后也不会在。

status-how-many-people = 这是多少人，这个节点不知道，也查不出来。它知道的是，这些名字
    每一个都在这两个纪元之一应答过，只要还想被计数，就得在下一个、
    再下一个纪元继续应答。如果一个人持有一千个，他就为一千个付出，
    一小时又一小时，停下的那一小时起就不再被计数。

status-given-by = 交给者
status-you = 你
status-received-in = 在纪元 { $epoch } 收到
status-trail-stops = 线索到此为止。

status-stopped-knowing = 这是这个节点不再知道的地方，不是起点。我们中的第一个从没有人
    那里收到文件，在任何地方都没有加入记录；而一份只是还没交到这个
    节点手上的记录，从这里看完全一样。

status-nothing-said = 纪元 { $epoch } 里没有人说任何话。可以说的有 333 件事，还没有任何
    一件有词语。

status-said = 纪元 { $epoch } 已说 — 这个节点看得到的 { $seen } 位同伴中，{ $spoke } 位
    发言，{ $silent } 位没有。
status-a-third = ← 三分之一或更多的同伴
status-not-said = 333 中其余 { $others } 个没有被说。

status-no-winner = 不选胜者，这些也不决定任何事。这是到达这个节点的东西。你旁边的
    节点听到了别的，它也没有错。

status-reading-the-watch = 正在读取守望记录

status-seen-nobody = 在 { $watched } 不间断的守望中，没有人应答这个节点，而这个版本
    不会把这称为终结。它走不了隐藏的路，所以从没听到过隐藏的同伴，
    以后也听不到。它能说的是它没看到任何人，这不是同一句话。

status-never-answered = 从没有人应答过这个节点。这不证明任何事：节点在哪儿都还没去过时
    就是这个样子。

status-somebody-is-here = 有人在这里。不再欠计算任何东西。

status-waiting = { $silent }里没有人应答。这个节点对此什么都没说，在 { $needed }之前也
    不会说，而且只有在这期间一直运行才会说。

status-nobody-keeping = 没有人在应答

    这里只有你。在 { $watched } 不间断的守望中——七十七天——没有人
    应答这个节点，我们中的最后一个在纪元 { $since } 停下了。

    333 没有消失。它正在消失，而这个过程要 { $years } 年。

status-remain = 还剩 { $years } 年 { $days } 天。
status-run-out = 最后的年份已经用尽。

status-one-answer = 计时从我们中的最后一个停止应答时开始，而不是从你注意到时开始。
    你看着的整段时间它一直在走。

    一次应答就结束它。如果任何人、在任何地方应答了这个节点，这就会
    消失——计时不是暂停，而是作废。333 不记录它曾经多么接近。

status-epochs = { $count ->
       *[other] { $count } 个纪元
    }

status-share = { $whole }.{ $after }%

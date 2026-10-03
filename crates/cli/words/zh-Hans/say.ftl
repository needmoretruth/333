### `333 say`, and saying from the screen.

say-not-one = 共有 { $count } 个，编号 0 到 { $last }。“{ $typed }”不在其中。

say-no-such = 共有 { $count } 个，编号 0 到 { $last }。没有 { $index }。

say-not-joined = 还没有人把文件交给你，所以你说什么都不算数。先拿到它：
    `333 join <invitation>`。

say-already = 你已在纪元 { $epoch } 说过第 { $index } 个。每人一个，再说一次也不会
    替换它：节点最先说的，就是它说过的。

say-sealing = 正在封存你说的话

say-said = 纪元 { $epoch } 的第 { $index } 个
    .keyword = 已说

# Said under the line before, in its column, with no keyword of its own.
say-goes-out = 它传给这个节点能到达的所有人，他们再传下去。
    每 333 分钟，你可以说 333 件事中的一件，用编号说。没有第 334 件，
    也永远不会有。你不能说两次，不能说得更大声，活着的人里没有谁
    比你有更多可说。
    也没有人会告诉你它是什么意思。还没有对照表；等有了，它会是
    我们所有人同一张表，不翻译。
    .keyword = {""}

say-given-by-nobody = 你持有文件，但没有人交给你，所以你不在任何名册上，没有人会
    计算你说的话。只有别人交过文件的节点才能发言：把它传下去，
    你交给的人就能发言。

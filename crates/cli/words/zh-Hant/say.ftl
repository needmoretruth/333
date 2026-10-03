### `333 say`, and saying from the screen.

say-not-one = 共有 { $count } 個，編號 0 到 { $last }。“{ $typed }”不在其中。

say-no-such = 共有 { $count } 個，編號 0 到 { $last }。沒有 { $index }。

say-not-joined = 還沒有人把檔案交給你，所以你說什麼都不算數。先拿到它：
    `333 join <invitation>`。

say-already = 你已在紀元 { $epoch } 說過第 { $index } 個。每人一個，再說一次也不會
    替換它：節點最先說的，就是它說過的。

say-sealing = 正在封存你說的話

say-said = 紀元 { $epoch } 的第 { $index } 個
    .keyword = 已說

# Said under the line before, in its column, with no keyword of its own.
say-goes-out = 它傳給這個節點能到達的所有人，他們再傳下去。
    每 333 分鐘，你可以說 333 件事中的一件，用編號說。沒有第 334 件，
    也永遠不會有。你不能說兩次，不能說得更大聲，活著的人裡沒有誰
    比你有更多可說。
    也沒有人會告訴你它是什麼意思。還沒有對照表；等有了，它會是
    我們所有人同一張表，不翻譯。
    .keyword = {""}

say-given-by-nobody = 你持有檔案，但沒有人交給你，所以你不在任何名冊上，沒有人會
    計算你說的話。只有別人交過檔案的節點才能發言：把它傳下去，
    你交給的人就能發言。

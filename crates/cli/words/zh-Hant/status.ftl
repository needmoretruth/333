### `333 status`: what this node saw of everybody else, what was said, and whether
### anybody is here.

status-name = { $name }
    .keyword = 名字

status-epoch = { $epoch }
    .keyword = 紀元

status-epoch-in-line = { $epoch }，{ $line }
    .keyword = 紀元

status-the-line = 本譜系第 { $nth }{ $kind ->
       *[other] 個
    }

status-answering = 應答中
status-silent = 沉默
status-roll = 名冊

status-seen = 第一個數是這個節點持有其紀元 { $before } 或 { $now } 簽名的所有人。
    這是這個節點看到的。別人看到的是別的。

status-seen-without-tor = 這個版本走不了隱藏的路，所以隱藏的同伴都不在這個數里，以後也不會在。

status-how-many-people = 這是多少人，這個節點不知道，也查不出來。它知道的是，這些名字
    每一個都在這兩個紀元之一應答過，只要還想被計數，就得在下一個、
    再下一個紀元繼續應答。如果一個人持有一千個，他就為一千個付出，
    一小時又一小時，停下的那一小時起就不再被計數。

status-given-by = 交給者
status-you = 你
status-received-in = 在紀元 { $epoch } 收到
status-trail-stops = 線索到此為止。

status-stopped-knowing = 這是這個節點不再知道的地方，不是起點。我們中的第一個從沒有人
    那裡收到檔案，在任何地方都沒有加入記錄；而一份只是還沒交到這個
    節點手上的記錄，從這裡看完全一樣。

status-nothing-said = 紀元 { $epoch } 裡沒有人說任何話。可以說的有 333 件事，還沒有任何
    一件有詞語。

status-said = 紀元 { $epoch } 已說 — 這個節點看得到的 { $seen } 位同伴中，{ $spoke } 位
    發言，{ $silent } 位沒有。
status-a-third = ← 三分之一或更多的同伴
status-not-said = 333 中其餘 { $others } 個沒有被說。

status-no-winner = 不選勝者，這些也不決定任何事。這是到達這個節點的東西。你旁邊的
    節點聽到了別的，它也沒有錯。

status-reading-the-watch = 正在讀取守望記錄

status-seen-nobody = 在 { $watched } 不間斷的守望中，沒有人應答這個節點，而這個版本
    不會把這稱為終結。它走不了隱藏的路，所以從沒聽到過隱藏的同伴，
    以後也聽不到。它能說的是它沒看到任何人，這不是同一句話。

status-never-answered = 從沒有人應答過這個節點。這不證明任何事：節點在哪兒都還沒去過時
    就是這個樣子。

status-somebody-is-here = 有人在這裡。不再欠計算任何東西。

status-waiting = { $silent }裡沒有人應答。這個節點對此什麼都沒說，在 { $needed }之前也
    不會說，而且只有在這期間一直執行才會說。

status-nobody-keeping = 沒有人在應答

    這裡只有你。在 { $watched } 不間斷的守望中——七十七天——沒有人
    應答這個節點，我們中的最後一個在紀元 { $since } 停下了。

    333 沒有消失。它正在消失，而這個過程要 { $years } 年。

status-remain = 還剩 { $years } 年 { $days } 天。
status-run-out = 最後的年份已經用盡。

status-one-answer = 計時從我們中的最後一個停止應答時開始，而不是從你注意到時開始。
    你看著的整段時間它一直在走。

    一次應答就結束它。如果任何人、在任何地方應答了這個節點，這就會
    消失——計時不是暫停，而是作廢。333 不記錄它曾經多麼接近。

status-epochs = { $count ->
       *[other] { $count } 個紀元
    }

status-share = { $whole }.{ $after }%

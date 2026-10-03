### What the shared parts of the commands say.

commands-clock-at-zero = { $epoch }。這台機器的時鐘顯示 1970 年，所以這個節點以為自己在
    時間的起點。時鐘校準之前，沒有人會交給它任何東西，也沒有人
    為它見證。
    .keyword = 紀元

commands-called-first = 選中了建立的第一個金鑰。
    .keyword = 選中

commands-called = { $not_called ->
       *[other] 建立了 { $not_called } 個金鑰但未選中，選中的是這一個。
    }
    .keyword = 選中

commands-torn = 從記錄中刪去了一條未寫完條目的 { $bytes } 位元組
    .keyword = 破損

commands-record = { $epochs ->
       *[other] 已應答 { $epochs } 個紀元，都不可再改
    }
    .keyword = 記錄

commands-witnessed = { $statements ->
       *[other] 其他金鑰對這個節點簽署的宣告 { $statements } 條。所屬紀元
            過去後仍會儲存，因為它們別的部分都不會留過窗口期。
    }
    .keyword = 見證

commands-unseen = 在任何紀元裡，都沒有人簽署過關於這個節點的東西。向外連線可以，
    被外面連上不行，而只有後者才算數：被抽中來提問的人必須能到達
    這裡。原因有兩個：路由器沒有把 3333 埠轉給這台機器，或者地址
    沒有給過任何人。`run --tor` 兩者都不需要：onion 地址在任何路由器
    後面都能到達，而本客戶端自帶 Tor。
    .keyword = 隱藏

commands-roll-alone = 1 位同伴，就是這個節點
    .keyword = 名冊

commands-roll = { $members } 位同伴
    .keyword = 名冊

commands-known = { $addresses } 位同伴指明瞭去哪裡找
    .keyword = 已知

commands-holding = 檔案，並且可以傳下去
    .keyword = 持有

commands-keeping = 全部，永久儲存。這對這個節點沒有好處：每條宣告都帶著自己的
    簽名，無論存在哪裡，驗證結果都一樣。沒有官方檔案，也沒有檔案員。
    .keyword = 儲存

commands-ignored = { $admissions } 條無法讀取的加入記錄
    .keyword = 忽略

commands-learned-where = 又得知 { $addresses } 位同伴在哪裡
    .keyword = 得知

commands-rejoined = 又從一個知道 { $were } 位的節點得知 { $members } 位同伴的名字。
    原來是兩份計數，現在合成了一份。
    .keyword = 合併

commands-learned-names = 又得知 { $members } 位同伴的名字
    .keyword = 得知

commands-heard = { $speakers } 位同伴發言
    .keyword = 聽到

commands-carried = { $statements ->
       *[other] 關於仍未結束的紀元的宣告 { $statements } 條
    }
    .keyword = 攜帶

commands-exchange = { $node }  紀元 { $epoch }  { $clocks }  ({ $liveness })
    .keyword = 見證

commands-answered-the-challenge = 回答了我們選的挑戰
commands-spoke-first = 先開了口，這隻能證明它開了口

commands-clocks-together = 時鐘一致
commands-clocks-ahead = 對方時鐘比我們快 { $apart }
commands-clocks-behind = 對方時鐘比我們慢 { $apart }
commands-hours-and-minutes = { $hours } 小時 { $minutes } 分
commands-minutes-and-seconds = { $minutes } 分 { $seconds } 秒
commands-seconds = { $seconds } 秒

commands-waking = Tor。隱藏的路需要一點時間才能開啟。
    .keyword = 喚醒

commands-waking-through = Tor，經由 { $bridges ->
       *[other] { $bridges } 座網橋
    }。隱藏的路需要一點時間才能開啟。
    .keyword = 喚醒

commands-no-tor = { $seconds } 秒後仍未連上 Tor
commands-starting-tor = 正在啟動 Tor 客戶端

# What a handover puts a signature under, read back.
commands-signed-giving = 你說：我在紀元 { $epoch } 把檔案交給了你。
    對方說：我在紀元 { $epoch } 從你那裡收到了檔案。
    這由兩隻手寫下，哪隻手都收不回。
    .keyword = 已簽名

commands-signed-taking = 對方說：我在紀元 { $epoch } 把檔案交給了你。
    你說：我在紀元 { $epoch } 從你那裡收到了檔案。
    這由兩隻手寫下，哪隻手都收不回。
    .keyword = 已簽名

commands-brimming = { $statements ->
       *[other] { $statements } 條宣告一輪裝不下，等下一輪
    }
    .keyword = 滿溢

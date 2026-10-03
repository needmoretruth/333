### `333 status`: where the others are, as far as this node knows.

status-known-another-copy = 這個名字的另一個副本

status-known-sighting = 一條用這個節點的金鑰簽署、卻不是這個節點做出的宣告說，它在
    紀元 { $said_in } 位於 { $address }。
    它在紀元 { $epoch } { $arrived }到達。

status-known-either = 要麼這個目錄被複制並啟動了副本，要麼別人拿到了金鑰。同名的兩個
    節點，在任何一個被問到的每個紀元都會互相矛盾。停掉其中一個；
    搬遷節點要用 `333 pack`。這裡不會替你停掉任何一個副本：舊宣告
    誰都能重放，一個看到它就停止的節點，可以被任何持有其金鑰副本的
    人關掉。

status-known-nowhere = 還沒有可敲的門。交給 `333 ping` 或 `333 join` 的邀請會被儲存，
    之後節點就去敲那裡。
    .keyword = 已知

status-known-held = { $held ->
       *[other] { $held } 個地址
    }，按最初聽說的地方分
    .keyword = 已知

status-known-by-hand = 手動
status-known-this-network = 本網路
status-known-meeting-point = 匯合點
status-known-from-us = 來自 { $peers } 位同伴
status-known-not-noted = 未記錄

status-known-where-heard = 從哪裡聽說，與那裡有沒有人應答無關。
status-known-sources-lists = `333 status --sources` 會列出它們。

status-known-sources = 來源
status-known-nobody-answered = 這裡還沒有人應答過
status-known-at = 位於
status-known-first = 最早
status-known-last = 最後
status-known-from-before = 在這個節點開始記錄來源之前就持有的
status-known-when = { $from }，紀元 { $epoch }

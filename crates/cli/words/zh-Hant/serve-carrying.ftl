### `333 run`: carrying out what the node was told, from its screen or another terminal.

serve-carrying-not-written-who = 正在記下誰應答了：{ $why }
    .keyword = 失敗

serve-carrying-unheard = { $why }
    .keyword = 未聽到

serve-carrying-unbegun = { $why }
    .keyword = 未開始

serve-carrying-refused = { $why }
    .keyword = 拒絕

serve-carrying-holding-the-file = 名冊上 { $roll } 位同伴，檔案在這裡
    .keyword = 持有

serve-carrying-holding-no-file = 名冊上 { $roll } 位同伴，這個節點還沒拿到檔案
    .keyword = 持有

serve-carrying-unread-holding = 這個節點持有的東西：{ $why }
    .keyword = 無法讀

serve-carrying-already-up = 隱藏地址已經開著。`tor off` 關閉它。
    .keyword = 已開啟

serve-carrying-unraised = { $why }
    .keyword = 未開啟

serve-carrying-tor-off = onion 地址現在停止應答。關於它已說過的話仍然有效，直到說出後
    兩個紀元被遺忘。
    .keyword = 隱藏

serve-carrying-none-up = 沒有開著的隱藏地址可關。
    .keyword = 已開啟

serve-carrying-too-late = Tor 已在執行，現在加的網橋改變不了它已建立的連線。請改為帶著
    它重啟節點。
    .keyword = 太晚

serve-carrying-bridged = { $bridges ->
       *[other] { $bridges } 座網橋
    }會在 Tor 下次啟動時使用。
    .keyword = 網橋

serve-carrying-helper = 任何混淆網橋都會執行 { $program }。
    .keyword = 網橋

serve-carrying-not-an-address = { $typed } 不是地址：{ $why }
    .keyword = 無法讀

serve-carrying-stopping = 另一個終端要求的。
    .keyword = 停止中

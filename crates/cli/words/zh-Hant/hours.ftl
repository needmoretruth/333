### `333 run`: keeping the hours, at every epoch boundary.

hours-failed-marking = 正在記錄本紀元：{ $why }
    .keyword = 失敗

hours-failed-sources = 正在記下地址的來源：{ $why }
    .keyword = 失敗

hours-not-leaving = 不在 { $place } 留下這個地址。它只能從這裡到達這個節點，陌生人
    撥它會連到自己那邊的東西。
    .keyword = 匯合

hours-sealing = 正在封存這個節點的地址

hours-failed-keeping-address = 正在儲存這個節點自己的地址：{ $why }
    .keyword = 失敗

hours-failed-saying-where = 正在說明這個節點在哪裡：{ $why }
    .keyword = 失敗

hours-forgot = { $epochs ->
       *[other] { $epochs } 個紀元
    }。現在關於它們說什麼都改變不了判定。
    .keyword = 遺忘

hours-failed-forgetting = 正在遺忘舊宣告：{ $why }
    .keyword = 失敗

hours-minutes = { $minutes ->
       *[other] { $minutes } 分鐘
    }

hours-minutes-and-seconds = { $minutes } 分 { $seconds } 秒
hours-hours-and-minutes = { $hours } 小時 { $minutes } 分

hours-epochs-answered-for = { $epochs ->
       *[other] 已應答 { $epochs } 個紀元
    }

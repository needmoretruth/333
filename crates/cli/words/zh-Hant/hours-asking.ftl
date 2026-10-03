### `333 run`: trading with the other nodes once an epoch, and asking whoever was drawn.

hours-asking-failed-gathering = 正在彙集這個節點可以傳出的內容：{ $why }
    .keyword = 失敗

hours-asking-quiet = { $address }：{ $why }
    .keyword = 沉默

hours-asking-unended = 交換沒有在本紀元的 { $time } 內完成，剩下的時間不會等它
    .keyword = 未完

hours-asking-reading-address = 正在讀取對方的地址
hours-asking-exchanging-heartbeats = 正在交換心跳
hours-asking-trading = 正在交換宣告
hours-asking-putting = 正在提問
hours-asking-sealing-presenting = 正在封存這個節點來說的話
hours-asking-saying-what-for = 正在說明這個節點為何而來
hours-asking-sealing-silence = 正在封存沒有發生的事

hours-asking-no-answer = 無應答
hours-asking-did-not-answer = { $address } 沒有應答
hours-asking-did-not-finish = { $address } 沒有完成心跳
hours-asking-neither = { $address } 既沒提問也沒結束通話
hours-asking-within = 在窗口期允許的 { $seconds } 秒內{ $what }
hours-asking-nothing-within = 在窗口期允許的 { $seconds } 秒內什麼都沒有

hours-asking-going = 外面沒人能連到這個節點，所以它主動去找本紀元被抽中來問它的
    { $drawn } 位同伴。抽籤由紀元和金鑰決定，所以這個節點不用別人
    告訴就知道是誰。
    .keyword = 前往

hours-asking-unknown-drawn-by = 被抽中由一位沒人說過在哪的同伴來問
    .keyword = 不明

hours-asking-unasked = 紀元 { $epoch }：{ $why }
    .keyword = 未問

hours-asking-drawn = 紀元 { $epoch } — 要問 { $asked } 位同伴。沒有人挑選：名字由紀元
    和金鑰決定，在每台機器上都一樣。
    .keyword = 抽籤

hours-asking-unknown-drawn-to-ask = 被抽中去問一位沒人說過在哪的同伴
    .keyword = 不明

hours-asking-unheard = 紀元 { $epoch }：{ $why }
    .keyword = 未聽到

hours-asking-witness = 紀元 { $epoch } 由 { $prover } 應答
    .keyword = 見證

hours-asking-silence = 紀元 { $epoch }：{ $why }
    .keyword = 沉默

### `333 join`: asking a node that holds the file to hand it over.

join-name = { $name }
    .keyword = 名字

join-knocking = { $address }
    .keyword = 敲門

join-silence = 在 { $address } 沒有聯絡到任何人。這不能證明 333 已經結束。本客戶端
    帶的是檔案的雜湊而不是檔案：只能從持有它的人那裡進來。
    .keyword = 沉默

join-knocking-on = 正在敲 { $address } 的門
join-exchanging = 正在交換心跳
join-asking = 正在索取檔案
join-no-answer = { $seconds } 秒後 { $address } 仍無應答

join-given = 來自 { $giver }
    .keyword = 收到

join-joined = 紀元 { $epoch }
    .keyword = 加入

join-holding = 檔案，並且可以傳下去
    .keyword = 持有

join-roll = { $members } 位同伴
    .keyword = 名冊

join-counted = 從紀元 { $epoch } 起，一個紀元也不會提前：隔兩個邊界，視你在本紀元
    何時到達，約 { $least } 到 { $most } 分鐘後。在那之前，有問必答。
    這段時間得到的見證，就是你曾在這裡的全部證明。
    .keyword = 計數

join-vigil = 從現在起用 `333 start` 讓它一直執行。沒人能聯絡到的節點無法被
    見證，而這一段只會被見證一次，或者永遠不會。
    .keyword = 節點

join-already-given = 這個節點已持有檔案，由 { $giver } 在紀元 { $epoch } 交給它。
    沒有要索取的，也沒有索取。
join-same-handover = 這個節點和 { $peer } 已在紀元 { $epoch } 傳過檔案。同一紀元
    再交回來，只是從另一邊讀同一次交接，不會接納任何人，所以沒有索取。

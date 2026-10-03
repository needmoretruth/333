### `333 run`: orders from other terminals on this machine.

serve-told-not-here = 在這個系統上，命令只能通過螢幕到達這個節點。在旁邊啟動第二個
    333 會被拒絕。
    .keyword = 命令

serve-told-cannot = 無法從其他終端接收：{ $why }
    .keyword = 命令

serve-told-not-private = 無法把通訊端設為私有

serve-told-taking = 來自這台機器的任何終端，用螢幕上的說法：
    `333 say 7`、`333 join <invitation>`、`333 tell 'tor on'`、`333 stop`。
    這個節點執行它們，並在那裡回應。
    .keyword = 命令

serve-told-taking-light = 來自這台機器的任何終端：
    `333 say 7`、`333 join <invitation>`、`333 tell 'tor on'`、`333 stop`。
    這個節點執行它們，並在那裡回應。
    .keyword = 命令

serve-told-old-socket = 一個舊通訊端擋著，會留在那裡：{ $why }
    .keyword = 命令

serve-told-not-a-socket = { $path } 存在但不是通訊端，所以不動它；在它被移走之前，無法從
    其他終端把任何東西交給這個節點。
    .keyword = 命令

serve-told-no-longer = 不再從其他終端接收：{ $why }
    .keyword = 命令

serve-told-not-the-owner = 只有這個節點目錄的所有者才能吩咐它
    .keyword = 拒絕

serve-told-too-long = 這比任何命令都長。最長的是 { $bytes } 位元組。
    .keyword = 無法讀

serve-told-other-version = 這個節點講 { $ours }，卻被用 { $theirs } 提問。提問的 333 與正在
    執行的版本不同；請用那一個。
    .keyword = 拒絕

serve-told-unread = { $why }
    .keyword = 無法讀

serve-told-asked = { $order }，來自另一個終端
    .keyword = 請求

serve-told-unheard = 已經沒有東西在執行命令
    .keyword = 未聽到

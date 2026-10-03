### The screen: what it says in the log pane about what was typed into it.

screen-asked = { $typed }
    .keyword = 請求
screen-unheard = 已經沒有東西在執行命令
    .keyword = 未聽到
screen-unread = { $why }
    .keyword = 無法讀
screen-refused = { $why }
    .keyword = 拒絕

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = 有個按鍵無法讀取：{ $why }。之後的按鍵也許可以。
    .keyword = 鍵盤
screen-keyboard-gone = 無法讀取鍵盤（{ $why }），所以螢幕關閉了，節點也隨之停止。
    `333 run --plain` 可以不用鍵盤執行節點。

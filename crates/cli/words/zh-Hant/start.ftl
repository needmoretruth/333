### `333 start` and `333 restart`: running this node in the background.

start-no-node = { $home } 裡還沒有節點

start-no-node-next = `333 join <invitation>` 用執行 333 的人給的邀請加入。
    `333 begin` 獨自開始。

start-in-a-terminal = 已經在終端裡執行。在那裡停止它，或用 `333 stop`，然後
    `333 start` 會在後台執行它。
    .keyword = 執行

start-already = 已經在後台執行。
    .keyword = 執行

start-started = 在後台，現在和每次重啟後。`333 status` 顯示狀況；`333 stop` 停止它。
    .keyword = 已啟動

start-elsewhere = 這台機器的後台服務執行的是 { $other } 裡的節點。
    `333 service uninstall` 移除它，然後再 `333 start`。

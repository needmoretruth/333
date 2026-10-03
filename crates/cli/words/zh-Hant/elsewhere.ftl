### What a command says when another 333 already has this node's directory.

elsewhere-the-vigil = 在這個目錄中執行的節點
elsewhere-the-vigil-by-number = 在這個目錄中執行的節點（行程 { $pid }）
elsewhere-another = 另一個 333
elsewhere-another-by-number = 另一個 333（行程 { $pid }）

elsewhere-done = 由 { $vigil } 執行了。
    .keyword = 完成

elsewhere-failed = { $vigil } 沒有執行。
    .keyword = 失敗

elsewhere-finding-its-name = { $who } 還在尋找這個節點的名字。
    有了名字後請再執行一次。
    .keyword = 佔用

elsewhere-already-keeping = { $who } 已在這裡執行，一個目錄就是一個節點。
    可以從這裡吩咐它：`333 say 7`、`333 join <invitation>`、
    `333 tell 'tor on'`、`333 stop`。第二個節點需要自己的目錄，
    用 --data-dir 指定。
    .keyword = 佔用

elsewhere-keeping = { $who } 正在這裡執行，而且
    { $why }
    .keyword = 佔用

elsewhere-nobody-to-tell = 這個目錄中沒有執行的節點，所以沒有可以吩咐的物件。
    用 `333 start` 或 `333 run` 執行它之後，這條命令就能用了。
    .keyword = 無人

elsewhere-busy = { $who } 佔著這個節點的目錄，
    但它不是可以接手這件事的執行中節點。這裡沒有讀寫任何東西。
    等它結束後請再執行一次。
    .keyword = 佔用

elsewhere-busy-cannot-be-handed = { $who } 佔著這個節點的目錄。
    在這個系統上，還無法從另一個終端把東西交給執行中的 333，
    所以這裡沒有讀寫任何東西。請在它的螢幕裡 `:` 之後輸入，
    或者停掉它再執行一次。
    .keyword = 佔用

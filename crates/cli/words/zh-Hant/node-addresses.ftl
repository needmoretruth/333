### Where the others are, and another copy of this node's name.

node-addresses-reading = 正在讀取地址
node-addresses-keeping = 正在儲存地址
node-addresses-reading-own = 正在讀取這個節點的地址
node-addresses-keeping-own = 正在儲存這個節點的地址

node-addresses-another-copy = 這個名字的副本在外面。一條用這個節點的金鑰簽署、卻不是這個
    節點做出的宣告說，它在紀元 { $said_in } 位於 { $address }。
    它{ $from }到達。要麼這個目錄被複制並啟動了副本，要麼別人拿到了
    金鑰。同名的兩個節點，在任何一個被問到的每個紀元都會互相矛盾。
    停掉其中一個；搬遷節點要用 `333 pack`。在你決定之前，這個節點
    繼續執行。
    .keyword = 副本

node-addresses-unread = 無法讀取各地址來源的記錄，所以重新開始一份。這個節點做的任何
    決定都不讀它。
    .keyword = 無法讀

node-addresses-copies = { $copies ->
       *[other] { $copies } 條用這個節點的金鑰簽署、卻不是它做出的宣告
    }在窗口期內到達了它。這個名字的另一個副本一直在執行。
    `333 status` 會顯示它自稱在哪裡。
    .keyword = 副本

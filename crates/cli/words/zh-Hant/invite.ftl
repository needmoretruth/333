### `333 invite`: the line others use to join through this node.

invite-line = { $invitation }
    .keyword = 邀請

invite-how = 把這一行交給對方。對方在自己的機器上執行 `333 join { $invitation }`。
    .keyword = 方法

invite-none = 還沒有。這個節點還沒找到別人能聯絡到的地址。
    .keyword = 邀請

invite-none-next = `333 start` 執行它；幾分鐘後再問。在沒人開啟過的路由器後面，
    `333 service install --tor` 會用不需要路由器的 onion 地址執行它。
    .keyword = 下一步

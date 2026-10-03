### `333 invite`: the line others use to join through this node.

invite-line = { $invitation }
    .keyword = 邀请

invite-how = 把这一行交给对方。对方在自己的机器上运行 `333 join { $invitation }`。
    .keyword = 方法

invite-none = 还没有。这个节点还没找到别人能联系到的地址。
    .keyword = 邀请

invite-none-next = `333 start` 运行它；几分钟后再问。在没人打开过的路由器后面，
    `333 service install --tor` 会用不需要路由器的 onion 地址运行它。
    .keyword = 下一步

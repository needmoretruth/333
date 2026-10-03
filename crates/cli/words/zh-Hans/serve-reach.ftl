### `333 run`: whether anybody outside can actually get in.

serve-reach-unanswered-at-the-end = 节点停止时路由器还没回应。它答应的东西会在 { $time } 内自行失效。
    .keyword = 关闭

serve-reach-shut-behind-another = 路由器说这个家庭在 { $seen }，这不是开放互联网上的地址：另一台
    路由器或运营商的共享地址挡在它和其他所有人之间，这里没有任何
    东西能请求那一台。`333 run --tor` 完全不需要改路由器。
    .keyword = 关闭

serve-reach-open = 端口 { $port } 从外面能到达这台机器。这个节点敲了 { $outside }
    的门，并由自己应答，所以这个地址可以给任何人。
    .keyword = 已开放

serve-reach-invite = { $invitation }
    .keyword = 邀请

serve-reach-shut-somebody-else = { $outside } 有东西应答了，但不是这个节点。你地址上的那个端口
    属于别的东西，写着它的邀请会把人带到错误的机器。
    .keyword = 关闭

serve-reach-shut-unfinished = { $outside } 有东西接受了连接却没有完成心跳：{ $why }。写着它的
    邀请不能发出去。
    .keyword = 关闭

serve-reach-shut-nothing = { $outside } 没有任何应答，所以在外界看来这个节点没有在监听。
    要么前面的路由器从没被设置把端口 { $port } 转到这里，要么它不让
    内部机器拨打自己的外部地址。`333 run --tor` 不需要改路由器，在
    任何网络都能用。
    .keyword = 关闭

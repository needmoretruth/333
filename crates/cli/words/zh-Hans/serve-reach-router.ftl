### `333 run`: asking the router, keeping what it lent, and giving it back.

serve-reach-router-opened-upnp = 路由器说端口 { $port } { $on } 现在转到这台机器。想撤掉的话，
    它在路由器里显示为 `333`。有没有东西到达，看下一行。
    .keyword = 已开放

serve-reach-router-opened-upnp-for = 路由器说端口 { $port } { $on } 现在转到这台机器，为期 { $time }。
    想撤掉的话，它在路由器里显示为 `333`。有没有东西到达，看下一行。
    .keyword = 已开放

serve-reach-router-opened-lease = 已通过 { $way } 向 { $router } 的路由器申请端口 { $port }，为期
    { $asked_for }。它说端口 { $granted_port } { $on } 现在转到这里，为期
    { $granted }。这个节点会在到期前再申请，停止时归还；如果节点被
    强行终止，到期后路由器会自己放掉。有没有东西到达，看下一行。
    .keyword = 已开放

serve-reach-router-nobody-answered = 这里没有路由器回应开放端口的请求，UPnP-IGD、PCP、NAT-PMP 都
    没有。这很常见：很多路由器三者都关着，有自己地址的机器也无需
    申请。`--no-router` 让这个节点完全不去申请。
    .keyword = 关闭

serve-reach-router-refused = 路由器不肯开放端口 { $port }：{ $why }
    .keyword = 关闭

serve-reach-router-let-go = 路由器放掉了端口 { $port }：没有及时再申请，所以外面无法再通过它
    到达这个节点。重启节点会再申请。
    .keyword = 关闭

serve-reach-router-given-back = 已通过 { $way } 把端口 { $port } 还给路由器；它不再转到这台机器。
    .keyword = 关闭

serve-reach-router-not-taken-back = 路由器没有收回端口 { $port }（{ $why }）。它会在 { $time } 内
    自己放掉。
    .keyword = 关闭

serve-reach-router-moved = 路由器移动了这个节点：现在转到这里的是端口 { $port } { $on }，
    而不是端口 { $before_port } { $before_on }。写着旧端口的邀请已经无法到达。
    .keyword = 已开放

serve-reach-router-not-kept = 申请后路由器没有保留端口 { $port }（{ $why }）。它还会保留
    { $time }，到期前会再申请。
    .keyword = 等待

serve-reach-router-on-its-outside-address = 在它的外部地址上
serve-reach-router-on = 在 { $address } 上

serve-reach-router-one-second = 1 秒
serve-reach-router-two-seconds = 2 秒
serve-reach-router-seconds = { $count } 秒
serve-reach-router-one-minute = 1 分钟
serve-reach-router-two-minutes = 2 分钟
serve-reach-router-minutes = { $count } 分钟
serve-reach-router-one-hour = 1 小时
serve-reach-router-two-hours = 2 小时
serve-reach-router-hours = { $count } 小时

### `333 run`: asking the router, keeping what it lent, and giving it back.

serve-reach-router-opened-upnp = 路由器說埠 { $port } { $on } 現在轉到這台機器。想撤掉的話，
    它在路由器裡顯示為 `333`。有沒有東西到達，看下一行。
    .keyword = 已開放

serve-reach-router-opened-upnp-for = 路由器說埠 { $port } { $on } 現在轉到這台機器，為期 { $time }。
    想撤掉的話，它在路由器裡顯示為 `333`。有沒有東西到達，看下一行。
    .keyword = 已開放

serve-reach-router-opened-lease = 已通過 { $way } 向 { $router } 的路由器申請埠 { $port }，為期
    { $asked_for }。它說埠 { $granted_port } { $on } 現在轉到這裡，為期
    { $granted }。這個節點會在到期前再申請，停止時歸還；如果節點被
    強行終止，到期後路由器會自己放掉。有沒有東西到達，看下一行。
    .keyword = 已開放

serve-reach-router-nobody-answered = 這裡沒有路由器回應開放埠的請求，UPnP-IGD、PCP、NAT-PMP 都
    沒有。這很常見：很多路由器三者都關著，有自己地址的機器也無需
    申請。`--no-router` 讓這個節點完全不去申請。
    .keyword = 關閉

serve-reach-router-refused = 路由器不肯開放埠 { $port }：{ $why }
    .keyword = 關閉

serve-reach-router-let-go = 路由器放掉了埠 { $port }：沒有及時再申請，所以外面無法再通過它
    到達這個節點。重啟節點會再申請。
    .keyword = 關閉

serve-reach-router-given-back = 已通過 { $way } 把埠 { $port } 還給路由器；它不再轉到這台機器。
    .keyword = 關閉

serve-reach-router-not-taken-back = 路由器沒有收回埠 { $port }（{ $why }）。它會在 { $time } 內
    自己放掉。
    .keyword = 關閉

serve-reach-router-moved = 路由器移動了這個節點：現在轉到這裡的是埠 { $port } { $on }，
    而不是埠 { $before_port } { $before_on }。寫著舊埠的邀請已經無法到達。
    .keyword = 已開放

serve-reach-router-not-kept = 申請後路由器沒有保留埠 { $port }（{ $why }）。它還會保留
    { $time }，到期前會再申請。
    .keyword = 等待

serve-reach-router-on-its-outside-address = 在它的外部地址上
serve-reach-router-on = 在 { $address } 上

serve-reach-router-one-second = 1 秒
serve-reach-router-two-seconds = 2 秒
serve-reach-router-seconds = { $count } 秒
serve-reach-router-one-minute = 1 分鐘
serve-reach-router-two-minutes = 2 分鐘
serve-reach-router-minutes = { $count } 分鐘
serve-reach-router-one-hour = 1 小時
serve-reach-router-two-hours = 2 小時
serve-reach-router-hours = { $count } 小時

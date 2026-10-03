### `333 run`: whether anybody outside can actually get in.

serve-reach-unanswered-at-the-end = 節點停止時路由器還沒回應。它答應的東西會在 { $time } 內自行失效。
    .keyword = 關閉

serve-reach-shut-behind-another = 路由器說這個家庭在 { $seen }，這不是開放網際網路上的地址：另一台
    路由器或運營商的共享地址擋在它和其他所有人之間，這裡沒有任何
    東西能請求那一台。`333 run --tor` 完全不需要改路由器。
    .keyword = 關閉

serve-reach-open = 埠 { $port } 從外面能到達這台機器。這個節點敲了 { $outside }
    的門，並由自己應答，所以這個地址可以給任何人。
    .keyword = 已開放

serve-reach-invite = { $invitation }
    .keyword = 邀請

serve-reach-shut-somebody-else = { $outside } 有東西應答了，但不是這個節點。你地址上的那個埠
    屬於別的東西，寫著它的邀請會把人帶到錯誤的機器。
    .keyword = 關閉

serve-reach-shut-unfinished = { $outside } 有東西接受了連線卻沒有完成心跳：{ $why }。寫著它的
    邀請不能發出去。
    .keyword = 關閉

serve-reach-shut-nothing = { $outside } 沒有任何應答，所以在外界看來這個節點沒有在監聽。
    要麼前面的路由器從沒被設定把埠 { $port } 轉到這裡，要麼它不讓
    內部機器撥打自己的外部地址。`333 run --tor` 不需要改路由器，在
    任何網路都能用。
    .keyword = 關閉

### `333 invite`: the line others use to join through this node.

invite-line = { $invitation }
    .keyword = 招待状

invite-how = この行を相手に渡してください。相手は自分のマシンで
    `333 join { $invitation }` を実行します。
    .keyword = 方法

invite-none = まだありません。このノードは、ほかの人が届くアドレスを
    見つけていません。
    .keyword = 招待状

invite-none-next = `333 start` で動かし、数分後にもう一度尋ねてください。誰も
    開けていないルーターの後ろなら、`333 service install --tor` が
    ルーター不要の onion アドレスで動かします。
    .keyword = 次

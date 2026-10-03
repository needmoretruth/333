### Where the others are, and another copy of this node's name.

node-addresses-reading = アドレスを読み込み中
node-addresses-keeping = アドレスを保管中
node-addresses-reading-own = このノードのアドレスを読み込み中
node-addresses-keeping-own = このノードのアドレスを保管中

node-addresses-another-copy = この名前の写しがどこかにあります。このノードの鍵で署名され、
    このノード自身は作っていない言明が、エポック { $said_in } に
    { $address } にいると言っています。
    { $from }届きました。このディレクトリがコピーされて写しが
    起動されたか、誰かほかの人が鍵を持っています。同じ名前の
    二つのノードは、どちらかが質問されるエポックごとに食い違います。
    一方を止めてください。ノードを移すには `333 pack` を使います。
    どちらにするか決めるまで、このノードは動き続けます。
    .keyword = 別の写し

node-addresses-unread = 各アドレスの出所のメモを読めなかったので、新しく始めます。
    このノードの判断はそれを読みません。
    .keyword = 読めず

node-addresses-copies = { $copies ->
       *[other] このノードの鍵で署名され、このノードが作っていない言明
            { $copies } 件
    }が、期間内に届きました。この名前の別の写しが
    動いていました。`333 status` でそれが告げた居場所がわかります。
    .keyword = 別の写し

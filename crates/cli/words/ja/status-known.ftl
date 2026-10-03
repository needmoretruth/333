### `333 status`: where the others are, as far as this node knows.

status-known-another-copy = この名前の別の写し

status-known-sighting = このノードの鍵で署名され、このノード自身は作っていない言明が、
    エポック { $said_in } に { $address } にいると言っています。
    エポック { $epoch } に { $arrived }届きました。

status-known-either = このディレクトリがコピーされて写しが起動されたか、誰かほかの人が
    鍵を持っています。同じ名前の二つのノードは、どちらかが質問される
    エポックごとに食い違います。一方を止めてください。ノードを移すには
    `333 pack` を使います。ここではどちらの写しも止めません。古い言明は
    誰でも再送でき、それを見て止まるノードは、鍵の写しを持つ誰にでも
    止められてしまうからです。

status-known-nowhere = まだ訪ねる先がありません。`333 ping` や `333 join` に渡した
    招待状は保管され、以後ノードはそこを訪ねます。
    .keyword = 既知

status-known-held = { $held ->
       *[other] アドレス { $held } 件
    }。最初に聞いた場所ごとに
    .keyword = 既知

status-known-by-hand = 手入力
status-known-this-network = このネットワーク
status-known-meeting-point = 集合場所
status-known-from-us = 仲間 { $peers } 人から
status-known-not-noted = 記録なし

status-known-where-heard = どこで聞いたかは、そこで誰かが応答するかについて何も言いません。
status-known-sources-lists = `333 status --sources` で一覧できます。

status-known-sources = 出所
status-known-nobody-answered = ここではまだ誰も応答していません
status-known-at = 場所
status-known-first = 最初
status-known-last = 最後
status-known-from-before = 出所を書き留める前から持っていたもの
status-known-when = { $from }、エポック { $epoch }

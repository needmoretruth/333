### `333 run`: whether anybody outside can actually get in.

serve-reach-unanswered-at-the-end = ノードが止まった時、ルーターはまだ応答していませんでした。
    合意したものは { $time } 以内に自然に切れます。
    .keyword = 閉鎖

serve-reach-shut-behind-another = ルーターによると、この家は { $seen } にあり、これは開かれた
    インターネットのアドレスではありません。別のルーターか
    プロバイダの共有アドレスが外との間にあり、ここからそちらには
    頼めません。`333 run --tor` ならルーターの変更は一切要りません。
    .keyword = 閉鎖

serve-reach-open = ポート { $port } は外からこのマシンに届きます。このノードは
    { $outside } を訪ね、自分自身が応答したので、そのアドレスは
    誰に渡しても大丈夫です。
    .keyword = 開放

serve-reach-invite = { $invitation }
    .keyword = 招待状

serve-reach-shut-somebody-else = { $outside } で何かが応答しましたが、このノードではありません。
    あなたのアドレスのそのポートは別のものなので、それを書いた
    招待状は人を違うマシンへ送ってしまいます。
    .keyword = 閉鎖

serve-reach-shut-unfinished = { $outside } の何かが接続を受けたまま鼓動を終えませんでした:
    { $why }。それを書いた招待状は配れません。
    .keyword = 閉鎖

serve-reach-shut-nothing = { $outside } では何も応答しなかったので、外から見る限りこの
    ノードは待ち受けていません。前にあるルーターがポート { $port } を
    ここへ送るよう設定されていないか、内側のマシンが自分の外側の
    アドレスにかけるのを許していません。`333 run --tor` ならルーターの
    変更なしに、どのネットワークでも動きます。
    .keyword = 閉鎖

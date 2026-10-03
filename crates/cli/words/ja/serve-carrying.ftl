### `333 run`: carrying out what the node was told, from its screen or another terminal.

serve-carrying-not-written-who = 応答した相手を書き留めています: { $why }
    .keyword = 失敗

serve-carrying-unheard = { $why }
    .keyword = 未聴取

serve-carrying-unbegun = { $why }
    .keyword = 未開始

serve-carrying-refused = { $why }
    .keyword = 拒否

serve-carrying-holding-the-file = 名簿に { $roll } 人、ファイルはここにあります
    .keyword = 保持

serve-carrying-holding-no-file = 名簿に { $roll } 人、このノードはファイルを受け取っていません
    .keyword = 保持

serve-carrying-unread-holding = このノードが持つもの: { $why }
    .keyword = 読めず

serve-carrying-already-up = 隠れたアドレスはすでに開いています。`tor off` で閉じます。
    .keyword = 開放中

serve-carrying-unraised = { $why }
    .keyword = 未開放

serve-carrying-tor-off = onion アドレスは今から応答しなくなります。それについて言われた
    ことは、言われてから二エポック後に忘れられるまで残ります。
    .keyword = 不可視

serve-carrying-none-up = 閉じる隠れたアドレスはありません。
    .keyword = 開放中

serve-carrying-too-late = Tor はすでに動いていて、今足したブリッジはできた接続を何も
    変えません。代わりにそのブリッジでノードを再起動してください。
    .keyword = 遅すぎ

serve-carrying-bridged = { $bridges ->
       *[other] ブリッジ { $bridges } 本を
    }次の Tor 起動時に使います。
    .keyword = ブリッジ

serve-carrying-helper = 難読化ブリッジには { $program } を使います。
    .keyword = ブリッジ

serve-carrying-not-an-address = { $typed } はアドレスではありません: { $why }
    .keyword = 読めず

serve-carrying-stopping = 別の端末から求められました。
    .keyword = 停止中

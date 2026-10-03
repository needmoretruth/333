### Reaching another node, whichever way its address says to.

dial-would-show = このノードはアドレスを隠しているので、それが見えてしまう
    { $address } への接続は開きません

dial-no-answer = { $seconds } 秒たっても応答がありません

dial-waking = つなぐ価値のある相手が隠れたアドレスにいて、Tor が動いて
    いません。最初の起動には数秒から数分かかり、終わるまで誰にも
    何も尋ねません。
    .keyword = 起動

dial-unwoken = Tor が起動しませんでした: { $why }
    このエポックでは隠れたアドレスを飛ばします。その先のノードは
    応答を怠ったのではありません。尋ねに行く手段がなかったのです。
    .keyword = 未起動

dial-connecting = { $address } に接続中
dial-without-tor = このクライアントは Tor なしでビルドされたので、{ $address } に届きません

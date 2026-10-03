### What a command says when another 333 already has this node's directory.

elsewhere-the-vigil = このディレクトリで動作中のノード
elsewhere-the-vigil-by-number = このディレクトリで動作中のノード (プロセス { $pid })
elsewhere-another = 別の 333
elsewhere-another-by-number = 別の 333 (プロセス { $pid })

elsewhere-done = { $vigil } が実行しました。
    .keyword = 完了

elsewhere-failed = { $vigil } は実行しませんでした。
    .keyword = 失敗

elsewhere-finding-its-name = { $who } がこのノードの名前を
    まだ探しています。名前ができたらもう一度実行してください。
    .keyword = 使用中

elsewhere-already-keeping = { $who } がすでにここで動いています。
    一つのディレクトリは一つのノードです。ここから指示できます:
    `333 say 7`、`333 join <invitation>`、`333 tell 'tor on'`、`333 stop`。
    二つ目のノードには、--data-dir で別のディレクトリを指定します。
    .keyword = 使用中

elsewhere-keeping = { $who } がここで動いています。
    { $why }
    .keyword = 使用中

elsewhere-nobody-to-tell = このディレクトリで動いているノードがないので、伝える相手が
    いません。`333 start` か `333 run` で動かせば使えます。
    .keyword = 不在

elsewhere-busy = { $who } がこのノードの
    ディレクトリを使っていて、これを渡せる動作中のノードではありません。
    何も読み書きしていません。終わったらもう一度実行してください。
    .keyword = 使用中

elsewhere-busy-cannot-be-handed = { $who } がこのノードの
    ディレクトリを使っています。このシステムでは、まだ別の端末から
    動作中の 333 に何も渡せないので、何も読み書きしていません。
    その画面で `:` の後に入力するか、止めてから実行し直してください。
    .keyword = 使用中

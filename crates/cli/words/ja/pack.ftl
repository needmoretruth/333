### `333 pack`: writing this node into one file, to be carried to another machine.

pack-name-the-file = 梱包先のファイル名を指定してください: 333 pack <FILE>

pack-no-node = { $root } に梱包するノードがありません。何も書いていません。

pack-already-exists = { $file } はすでにあります。{ -pack-never-over }
-pack-never-over = 梱包は新しいファイルに書き、古いものに上書きしません。別の名前に。

pack-not-marked = { $file } は書きましたが、梱包済みの印を付けられませんでした。{ -pack-until-it-is }
-pack-until-it-is = 付くまで、このノードは両方に住んでいます。{ -pack-delete-that-file }
-pack-delete-that-file = ここで何か動かす前にそのファイルを削除してください。

pack-creating = { $file } を作成中

pack-name = { $name }
    .keyword = 名前

pack-record-none = まだありません
    .keyword = 記録

pack-record = { $epochs ->
       *[other] { $epochs } エポック、一緒に移ります
    }
    .keyword = 記録

pack-witnessed = { $statements ->
       *[other] 他の鍵がこのノードについて署名した言明 { $statements } 件、
            一緒に移ります
    }
    .keyword = 証言

pack-holding = ファイル、一緒に移ります
    .keyword = 保持

pack-onion-key = onion アドレスの鍵。アドレスも一緒に移ります
    .keyword = 不可視

pack-carrying = このノードを { $file } へ。
    そのファイルこそがこのノードです。持つ者はこの名前で応答できます。
    運び、展開し、それから削除してください。保管用のバックアップでは
    ありません。暗号化していないのは、パスワードも失いうるもう一つの
    ものになり、失えばファイルを失うのと同じく名前を失うからです。
    このディレクトリと同じく、読めるのはあなただけです。
    .keyword = 運搬

pack-packed = { $bytes } バイト。
    { $root } のどれも二度とこのノードとして動きません。
    .keyword = 梱包済

pack-next = 別のマシンで: 333 unpack { $carried }
    引っ越しをやめるなら: { $undo }
    .keyword = 次

pack-not-packed = このノードは梱包されていないので、{ $root } で取り消すものはありません
    .keyword = ここ

pack-restored = このノードはまた { $root } に住んでいます。
    梱包したファイルもまだこの名前です。どこかで展開していれば、
    どちらかを動かす前に一方を消してください。展開していなければ
    { $file } を削除してください
    .keyword = 復元

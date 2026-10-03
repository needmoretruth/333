### `333 unpack`: putting a packed node into this machine's node directory.

-unpack-nothing = 何も展開しませんでした。
-unpack-so-nothing = なので何も展開しませんでした。

unpack-kept = このディレクトリにはすでにノードがいます。その横に展開するには、
    --data-dir で専用のディレクトリを指定してください。

unpack-seed-in = { $file } の中の { $seed }

unpack-not-one-node = そのファイルは { $claimed } を収めたと言いますが、中の鍵は { $name } です。{ -unpack-not-one }
-unpack-not-one = 一つのノードではないので、何も展開しませんでした。

unpack-taken = 展開先のディレクトリを別の 333 が使い始めました。{ -unpack-nothing }

unpack-elsewhere = 333 --data-dir <別のディレクトリ> unpack { $file }

unpack-could-not-open = { $target } にはすでにノードがいて、{ -unpack-could-not-be-opened } 横に展開するには: { $elsewhere }
-unpack-could-not-be-opened = 何を持つか確かめるために開けませんでした。{ -unpack-nothing }

unpack-no-record = まだ記録なし
unpack-epochs-of-record = { $epochs ->
       *[other] { $epochs } エポックの記録
    }
unpack-holding = ファイルあり
unpack-not-holding = ファイルなし

unpack-occupied = { $target } にはすでにノードがいます:
    { $name }、{ $epochs }、{ $holding }。
    上に展開するとそれをすべて失うので、{ -unpack-so-nothing }
    横に展開するには、専用のディレクトリを指定してください:
    { $elsewhere }

unpack-holds-files = { $target } にはファイルがあり、ノードはありません。{ -unpack-its-own } 別の場所へ: { $elsewhere }
-unpack-its-own = ノードは専用のディレクトリに展開するので、{ -unpack-so-nothing }

unpack-opening-the-record = 記録を開いています
unpack-torn = そのファイルの記録は壊れていて、完全なノードではありません。{ -unpack-nothing }
unpack-reading-the-record = 記録を読み込み中
unpack-does-not-verify = そのファイルの記録を検証できません。{ -unpack-nothing }
unpack-another-key = そのファイルの記録は別の鍵が書いたものです。{ -unpack-nothing }

unpack-not-a-place = { $target } はノードを置けるディレクトリではありません
unpack-making-room = { $target } に場所を作っています
unpack-putting = ノードを { $target } に置いています

unpack-name = { $name }
    .keyword = 名前

unpack-record-none = まだありません
    .keyword = 記録

unpack-record = { $epochs ->
       *[other] { $epochs } エポック、検証済み
    }
    .keyword = 記録

unpack-holding-the-file = ファイル
    .keyword = 保持

unpack-onion-key = onion アドレスの鍵。アドレスも一緒に来ました
    .keyword = 不可視

unpack-unpacked = { $target } へ、
    { $packed } に梱包したファイルから。
    今はこれがノードで、そのファイルもそうです: { $file } を削除してください
    .keyword = 展開済

unpack-next = { $serve }
    .keyword = 次

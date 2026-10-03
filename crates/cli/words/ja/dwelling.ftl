### Where this node lives, and whether it still lives here.

-dwelling-would-be-one = 一つの
-dwelling-anywhere-this = どこでも展開していないなら、これで

dwelling-made-at = 作成場所
dwelling-unpacked-at = 展開場所
dwelling-moved-to = 移動先(申告)
dwelling-found-at = このクライアントが最初に開いた場所

dwelling-writing = { $file } を書き込み中
dwelling-putting-in-place = { $file } を配置中
dwelling-resolving = { $dir } を解決中

dwelling-elsewhere = このノードは { $how } { $was } でした。
    今は { $now } にあります。
    ディレクトリを移動または改名しただけなら、それで終わりです。
    コピーして元がまだ動いているなら、一つの名前が二か所にあり、
    互いの記録と食い違います。一つだけ残ったら、ここで伝えてください:
    { $settle }
    .keyword = 住処

dwelling-unread-moment = 読めなかった時刻
dwelling-unread-file = 名前を読めなかったファイル

dwelling-packed = このノードは { $at } に引っ越し用に { $into } へ梱包されました。
    そのファイルを展開した場所に住んでいます。ここでも動かすと
    { -dwelling-would-be-one }名前が二か所にあることになるので、
    { $home } のどれもこのノードとして動きません。

    引っ越しをやめ、そのファイルを{ -dwelling-anywhere-this }
    元に戻せます: { $undo }

dwelling-unmarking = このノードを梱包した印を外しています

### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph. Between two Chinese or Japanese letters a break stays a break.

help-about = 333 のノードの一つ。尋ねられれば応答し、記録を保ち、
    ファイルを渡します。

help-id = このノードの名前を表示します。初回は名前を作ります

help-bootstrap = 渡してくれる人がいないときに、新しい系譜を始めます
help-bootstrap-long = 渡してくれる人がいないときに、新しい系譜を始めます。

    普通の入り方は招待状を使う `333 join` です。これはまず集合場所を
    見て、誰かいれば断ります。誰もいなければファイルを取得し、
    このクライアントが持つハッシュで確かめて書き込みます。
    あなたのノードは自分の系譜の創始者になり、その始まりには誰の
    署名もありません。記録を読めば誰でもそれがわかります。

help-serve = 止めるまで、この端末でこのノードを動かします
help-serve-long = 止めるまで、この端末でこのノードを動かします。

    鼓動と質問に応答し、知っていることを交換し、各エポックで抽選に
    当たった相手に質問します。端末では画面を開きます。`q`、Ctrl-C、
    または別の端末からの `333 stop` で止まります。
    `333 start` は同じことをバックグラウンドで行います。

help-serve-long-light = 止めるまで、この端末でこのノードを動かします。

    鼓動と質問に応答し、知っていることを交換し、各エポックで抽選に
    当たった相手に質問し、一行ずつ表示します。Ctrl-C または別の
    端末からの `333 stop` で止まります。`333 start` は同じことを
    バックグラウンドで行います。

help-say = 333 のうち一つを、エポックごとに一度言います。伝わるのは番号です

help-status = このノードが動いているか、どこで届くか、
    何人が応答しているかを表示します

help-join = 招待状を使い、ファイルを持つノードから受け取ります

help-languages = 言語を一覧するか、このノードの全コマンド用に一つ保存します

help-ping = 別のノードに届き、鼓動を一度交換します

help-pack = このノードを一つのファイルに書き出し、別のマシンへ運びます
help-pack-long = このノードを一つのファイルに書き出し、別のマシンへ運びます。

    名前、記録、ほかの人がこのノードについて署名したもの、ファイル、
    onion アドレスの鍵まで、すべて入ります。その後このディレクトリは
    このノードを動かすのを拒むので、名前が二か所に存在することは
    ありません。ファイルは暗号化されていません。持つ者がこのノード
    です。運び、展開し、削除してください。

help-unpack = 梱包したノードをこのマシンのノードディレクトリに置きます
help-unpack-long = 梱包したノードをこのマシンのノードディレクトリに置きます。

    すでにノードがある場所では拒否します。ファイルを最後まで読み、
    鍵と記録を確かめるまで、何も書き込みません。

help-moved = このノードのディレクトリをコピーではなく移動・改名したと伝えます
help-moved-long = このノードのディレクトリをコピーではなく移動・改名したと伝えます。

    新しい場所にいるノードは、これが入力されるまで毎回そう言います。
    元が動いたままのコピーなら、一つの名前が二か所にあることに
    なるからです。

help-tell = 動作中のノードに、画面の言葉で指示を渡します
help-tell-long = 動作中のノードに、画面の言葉で指示を渡します。

    `tor on`、`tor off`、`bridge <line>`、`helper <program>` など、
    画面で `:` の後に打てる言葉すべてです。動作中のノードが実行し、
    その応答がここに表示されます。`say`、`join`、`ping`、`begin`、
    `status`、`stop` はこれがなくても動作中のノードに届きます。

help-tell-light = 動作中のノードに指示を渡します
help-tell-long-light = 動作中のノードに指示を渡します。

    `tor on`、`tor off`、`bridge <line>`、`helper <program>` です。
    動作中のノードが実行し、その応答がここに表示されます。`say`、
    `join`、`ping`、`begin`、`status`、`stop` はこれがなくても
    動作中のノードに届きます。

help-service = バックグラウンドサービスを直接管理します (`start` と `stop` が使用)
help-service-long = バックグラウンドサービスを直接管理します (`start` と `stop` が使用)。

    頼まれるまで何もインストールせず、書いたファイルと実行した
    コマンドをその都度表示し、`333 service uninstall` ですべて
    取り除きます。

help-service-install = この run オプションでバックグラウンドサービスを入れて起動します
help-service-install-long = この run オプションでバックグラウンドサービスを入れて起動します。

    サービスは指定したオプションそのままで、このノードの
    ディレクトリに対し `333 run` を実行します。横で一時間ごとの確認が
    動き、ノードが止まればこのマシンで知らせます。
    `333 start` はオプションなしで同じことをします。

help-service-uninstall = バックグラウンドサービスを止め、入れたものをすべて取り除きます

help-service-status = サービス管理の状態、ノードが最後に起きていると告げた時刻、最後の数行

help-service-check = ノードが止まっていればこのマシンで知らせます。
    サービスが毎時実行し、問題がなければ何も言いません

help-data-dir = このノードの持ち物すべてを置くディレクトリ。
    名前と、Tor を使うなら Tor の状態

help-timeout = ネットワークと話す各段階で待つ秒数
help-timeout-long = ネットワークと話す各段階で待つ秒数。

    遅らせるのではなく上限です。数分かかりうる唯一の段階である
    Tor の起動に合わせてあります。

help-dangerously-trust-directory-permissions = このマシンの他人が入れるディレクトリも受け入れます
help-dangerously-trust-directory-permissions-long = このマシンの他人が入れるディレクトリも受け入れます。

    ディレクトリにはこのノードの名前の唯一の写しがあるので、
    権限の緩いディレクトリは既定で拒否します。これは試験用の
    ディレクトリや、所有者が変わったコンテナのためのものです。

help-keep-everything = 判定に使う期間だけでなく、すべての言明を永久に保管します
help-keep-everything-long = 判定に使う期間だけでなく、すべての言明を永久に保管します。

    誰の立場も変わりません。言明はどこに保管しても同じように
    検証できます。

help-bridges = 普通の Tor への入り口を塞ぐネットワーク用のブリッジ行
help-bridges-long = 普通の Tor への入り口を塞ぐネットワーク用のブリッジ行。

    受け取ったブリッジごとに一度、受け取ったとおりに指定します。
    ここではブリッジを取得しません。ブリッジは人が意図して配る
    ものです。一覧を集めてまとめて塞ぐことができないようにです。

help-bridge-helper = 難読化ブリッジを話すプログラム。名前かパスで指定
help-bridge-helper-long = 難読化ブリッジを話すプログラム。名前かパスで指定。

    ブリッジ行が求め、パスにあるのが `lyrebird` でない場合にだけ
    必要です。同梱していないのは、固定した写しはすぐに古くなる
    からです。

help-language = 表示する言語のタグ: `ko`、`es`、`zh-Hant`
help-language-long = 表示する言語のタグ: `ko`、`es`、`zh-Hant`。

    指定がなければ `THE333_LANGUAGE`、次に `333 language <TAG>` で
    保存した言語、最後に英語です。システムの言語は使いません。
    `333 language` は言葉のある言語を一覧します。
    `<data-dir>/words/<tag>/` にカタログのフォルダを置けば、
    ビルドせずに言語を足せます。333 の言葉そのものは翻訳しません。

help-count-in = 十進、十二進、twelve-ascii のどれで数えるか
help-count-in-long = 十進、十二進、twelve-ascii のどれで数えるか。

    表示する数はすべてこの進法で書き、入力した数もこの進法で
    読みます。十二進の `say 238` は十進の `say 332` です。名前、
    アドレス、ポート、バージョンは数え直さず、通信の中身も
    変わりません。指定がなければ `THE333_COUNT_IN`、次に十進です。

help-bootstrap-meet = 一人で始める前に人を探す場所

help-bootstrap-anyway = 誰かがいても始めます

help-serve-bind = 待ち受けるアドレスとポート

help-serve-tor = onion アドレスも開き、居場所を知られずに届くようにします。
    Tor の起動には数秒から数分かかります

help-serve-no-direct = ソケットを一切開きません。--tor と一緒にだけ使い、
    アドレスを通信に一切出しません

help-serve-announce = ほかのノードにこのノードの宛先として伝えるアドレス
help-serve-announce-long = ほかのノードにこのノードの宛先として伝えるアドレス。

    ソケットがそれを言えないときに必要です。全インターフェースで
    待ち受けるときや、ポートを転送する機器の後ろにいるときです。

help-serve-no-mdns = このノードがここにいることをローカルネットワークで告げません
help-serve-no-mdns-long = このノードがここにいることをローカルネットワークで告げません。

    告げる場合に出ていくのは、このマシンで何かが 333 を話している
    ことと、そのポートだけで、このノードの名前は出ません。同じ家の
    二つのノードが招待状なしに見つけ合う方法です。

help-serve-no-router = ポートをこのマシンへ送るようルーターに頼みません
help-serve-no-router-long = ポートをこのマシンへ送るようルーターに頼みません。

    家庭のルーターは、内側から頼まれていない通信を捨てます。
    内側のプログラムが UPnP-IGD、PCP、NAT-PMP でポート転送を頼む
    まではそうです。ネットワークを変えるので、行うときは表示します。
    `--no-upnp` はこれの古い名前です。

help-serve-meet = 誰にも紹介されていないノードを探す場所
help-serve-meet-long = 誰にも紹介されていないノードを探す場所。

    ノードの居場所について署名した言明を置く固定のアドレスです。
    そこで読んだものはすべてここで検証します。

help-serve-no-meet = 集合場所を一切使いません
help-serve-no-meet-long = 集合場所を一切使いません。

    このノードに届くのは、招待状を受け取った人と、このネットワーク
    上のノードだけになります。

help-serve-plain = 画面を描かず、行を表示します
help-serve-plain-long = 画面を描かず、行を表示します。

    端末以外ではいつも行を表示します。これは端末でもそうさせます。

help-serve-plain-light = 行を表示します。この版はいつもそうします
help-serve-plain-long-light = 行を表示します。この版はいつもそうします。

    この版には画面がありません。同じコマンドがどちらの版でも
    動くように、このオプションを受け付けます。

help-say-index = 0 から { $last } のどれか。数える進法 (--count-in) で入力します。
    言葉はまだ書かれていません

help-status-sources = このノードが持つアドレスをすべて一覧します。
    誰のものか、どこでいつ初めて聞き、最後はどこか

help-status-json = このノードが観測したものを、プログラム用の JSON で。
    アドレスもポートも含みません

help-join-address = すでに持っている人からの招待状 (`333:host:port`)

help-ping-address = 招待状 (`333:host:port`) またはアドレス:
    `host`、`host:port`、`[::1]:port`、`何か.onion` (Tor 経由)

help-pack-file = 書き出すファイル。まだ存在してはいけません

help-pack-undo = 引っ越しをやめたとき、ここでの梱包を取り消します
help-pack-undo-long = 引っ越しをやめたとき、ここでの梱包を取り消します。

    そのファイルをどこでも展開していない場合だけです。展開して
    いれば、二つになってしまいます。

help-unpack-file = `333 pack` が書き出したファイル

help-tell-order = 画面に打つのと同じ形の指示

help-tell-order-light = `tor on` や `bridge <line>` と同じ形の指示

help-service-install-flags = `run` のオプション。run の後に打つとおりに

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = ヘルプを表示します
help-print-help-more = ヘルプを表示します (詳しくは '--help')
help-print-help-summary = ヘルプを表示します (要約は '-h')
help-print-version = バージョンを表示します
help-print-this = このメッセージか、指定したサブコマンドのヘルプを表示します
help-print-for = サブコマンドのヘルプを表示します

help-start = このノードをバックグラウンドで動かします。今も、再起動後も

help-stop = このノードを止め、再起動後も止めたままにします

help-restart = このノードを止め、バックグラウンドで再び動かします

help-logs = このノードがバックグラウンドで書いた最後の数行を表示します

help-logs-follow = 新しい行が来るたびに表示し続けます (systemd が保つ場合)

help-invite = 他の人がこのノード経由で参加するための招待状を表示します

help-status-all = このノードが知っていることを、各部分の意味とともにすべて表示します

help-languages-tag = 保存する言語のタグ: `ko`、`en`。`en` で英語に戻ります

help-start-example = 例: 333 start

help-stop-example = 例: 333 stop

help-restart-example = 例: 333 restart

help-status-example = 例: 333 status --all

help-logs-example = 例: 333 logs -f

help-id-example = 例: 333 name

help-invite-example = 例: 333 invite

help-bootstrap-example = 例: 333 begin

help-serve-example = 例: 333 run --tor

help-say-example = 例: 333 say 7

help-join-example = 例: 333 join 333:192.0.2.7:3333

help-languages-example = 例: 333 language ja

help-ping-example = 例: 333 ping 333:192.0.2.7:3333

help-pack-example = 例: 333 pack node.333

help-unpack-example = 例: 333 unpack node.333

help-moved-example = 例: 333 moved

help-tell-example = 例: 333 tell tor on

help-service-example = 例: 333 service status

help-service-install-example = 例: 333 service install --tor

help-service-uninstall-example = 例: 333 service uninstall

help-service-status-example = 例: 333 service status

help-service-check-example = 例: 333 service check

help-start-flags = `run` のオプション。run の後に打つとおりに。
    別のオプションを渡すまで以後の起動でも使います

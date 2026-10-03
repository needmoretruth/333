# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = コピー
js-copied = コピーしました
js-selected = 選択しました
js-state-awake = このサイトのノードは起きています
js-state-not-running = このサイトのノードは動いていません
js-in-hours = { $h } 時間 { $m } 分後
js-in-minutes = { $m } 分後
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = この系譜の { $n } 番目のエポック

## The network

js-network-state-founder = どの名簿にもない
js-network-state-ok = このエポックに応答
js-network-state-quiet = このエポックは沈黙
js-network-state-later = 後のエポックから計数
js-network-state-seen = 見えたが名簿にない
js-network-awake = 起きている
js-network-not-running = 動いていない
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · あなたの
js-network-find-bad = ノードの名前は 16 進数です。最初の 6 文字以上を入力してください。
js-network-find-none = このサイトのノードはその名前のノードを見ていません。
js-network-find-many = そう始まるノードが { $count } 個あります。名前をもっと入力してください。
js-network-find-marked = この端末で、あなたのノードとして印を付けました。
js-network-select = ノードを選ぶと、このサイトのノードがそれについて知っていることが表示されます。
js-network-role-founder = この系譜の創始者
js-network-role-site = このサイトのノード
js-network-role-yours = あなたのノード（この端末）
js-network-role-none = ネットワーク上のノード
js-network-col-name = 名前
js-network-col-state = このエポック
js-network-col-given = ファイル受領
js-network-col-counted = 計数開始
js-network-col-answered = 最後の応答
js-network-col-said = 発言
js-network-col-reached = 到達経路
js-network-row-said = このエポックの発言
js-network-row-handed = ファイルを渡した相手
js-network-row-testimony = 証言
js-network-given-by = エポック { $epoch }、{ $sponsor } から
js-network-given-founder = 誰からでもありません。この系譜を始めました。
js-network-given-none = 名簿にない
js-network-epoch = エポック { $epoch }
js-network-epoch-now = { $epoch }（このエポック）
js-network-epoch-ago = { $epoch }（{ $ago } 前）
js-network-more = ほか { $count } 個は下の表に
js-network-nothing = なし
js-network-reach-direct = 直接
js-network-reach-tor = Tor 経由
js-network-reach-tor-short = Tor
js-network-reach-unknown = 不明
js-network-testimony = { $asked } 回尋ねられ、{ $asking } 回尋ねた（直近 3 エポック）
js-network-copy-name = 名前をコピー
js-network-select-name = 上の名前を選択してください
js-network-mine = これは私のノードです
js-network-tag-founder = 創始者
js-network-tag-site = このサイト
js-network-tag-yours = あなたの
js-network-empty = このサイトのノードはまだ他のノードを見ていません。
js-network-this-node = このノード
js-network-yes = はい
js-network-no = いいえ
js-network-none = なし

## Where we are

js-map-watch = ライブで見る
js-map-stop = 見るのをやめる
js-map-read-at = { $read_at } UTC に読み込み。
js-map-unreadable = 今は掲示板を読み込めませんでした。
js-map-tor = Tor
js-map-nowhere = 場所を特定できないところ
js-map-nobody = 居場所を告げているノードはありません。
js-map-all = 居場所を告げているノードの合計
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = ネットワーク上にいるが、場所を告げていない
js-map-dot = ノード { $count } 個

## The board

js-board-said = エポック { $epoch } に告げた · { $node }
js-board-site = このサイトのノード
js-board-tor = Tor 経由

## Take the program

js-start-machine-linux-x86_64 = x86-64 の Linux
js-start-machine-linux-aarch64 = 64 ビット ARM の Linux
js-start-machine-linux-armv6 = 32 ビット ARM の Linux
js-start-machine-macos-aarch64 = Apple シリコンの Mac
js-start-machine-macos-x86_64 = Intel チップの Mac
js-start-machine-windows-x86_64 = Windows
js-start-phone = スマートフォンかタブレットのようです。このプログラムは電源を入れたままにするコンピュータ用です。そのコンピュータをここで選んでください。
js-start-unknown = このブラウザは何の上で動いているかを伝えていません。ここでマシンを選んでください。
js-start-sure = このブラウザは { $machine } で動いていると伝えているので、それを選んであります。
js-start-mac = このブラウザは Mac だとは伝えていますが、チップは伝えていないので、Apple シリコンを選んであります。インストーラーはマシン自体に尋ねます。
js-start-linux = このブラウザは Linux だとは伝えていますが、プロセッサは伝えていないので、x86-64 を選んであります。インストーラーはマシン自体に尋ねます。
js-start-chosen = 上で選択

## The story on the home page, drawn

js-story-file = 333.txt · 3 バイト
js-story-gave = あなたに渡しました
js-story-received = あなたから受け取りました
js-story-signed = 署名済み
js-story-minutes = 333 分
js-story-epochs = 333 エポック
js-story-now = 現在
js-story-answering = 応答中
js-story-roll = 名簿
js-story-years = { $years } 年

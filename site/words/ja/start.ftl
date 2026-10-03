# Take the program: the installers, every file, and the first commands.

start-meta-title = プログラムを入手 · 333
start-meta-description = 333 のクライアントを Linux、macOS、Windows、Raspberry Pi にインストールし、リリースの SHA256SUMS で確かめてから、ファイルを受け取ります。

start-heading = プログラムを入手
start-lede = 333 は、使っていないコンピュータで動かし続ける小さなプログラムです。別の写しが訪ねてくるたびに、そこで応答します。
start-pick-file = そのマシン用のファイルは <code data-name="standard">333-x86_64-linux</code> です。ターミナル画面のない同じクライアント Light は <code data-name="light">333-light-x86_64-linux</code> です。どちらも Tor を内蔵しています。
start-machine = あなたのマシン
start-pick-linux-x86_64 = Linux、デスクトップまたはサーバー
start-pick-linux-aarch64 = 64 ビットの Raspberry Pi 3、4、5、またはその他の 64 ビット ARM Linux
start-pick-linux-armv6 = Raspberry Pi Zero、または 32 ビットで使う Pi
start-pick-macos-aarch64 = Mac、Apple シリコン
start-pick-macos-x86_64 = Mac、Intel
start-pick-windows = Windows
start-form = 形態
start-standard = Standard
start-light = Light
start-unix-title = Linux または Mac で
start-unix-installer = ターミナルで実行します。インストーラーは実行したマシンに合うファイルを選び、リリースの <code>SHA256SUMS</code> で確かめて <code>~/.local/bin/333</code> に置きます。何も起動せず、パスワードも求めません。<a href="/install.sh">先に読んでください</a>。短いものです。
start-unix-light = Light にするには、最後の <code>sh</code> を <code>sh -s -- --light</code> に替えます。
start-by-hand = インストーラーを使わずに手動で
start-unix-by-hand = 同じ手順を一つずつ行います。<code>grep</code> の行は何かを動かす前に <code>OK</code> と表示し、表示しなければその後は何も実行されません。
start-unix-mac = これは x86-64 の Linux 用です。Mac では <code>sha256sum</code> の代わりに <code>shasum -a 256</code> を使い、ファイルは下の表の Mac 用を使います。それ以外では表の名前を使います。
start-windows-title = Windows で
start-windows-installer = PowerShell で実行します。インストーラーは Windows 用のファイルを取得し、リリースの <code>SHA256SUMS</code> で確かめて <code>%LOCALAPPDATA%\Programs\333\333.exe</code> に置きます。何も起動せず、管理者権限も不要で、<code>-AddToPath</code> を付けたときだけそのフォルダーを PATH に加えます。<a href="/install.ps1">先に読んでください</a>。短いものです。
start-windows-light = Light は代わりにこう実行します。<code>-AddToPath</code> は同じように最後に付けます。
start-windows-by-hand = 同じ手順を一つずつ行います。ファイルはハッシュが <code>SHA256SUMS</code> の値と一致したときだけ所定の場所に置かれ、最後の行がどちらだったかを告げます。
start-files-title = すべてのファイル
start-files-intro = 上の推測が外れた人や、ブラウザでファイルを取りたい人のための表です。Standard にはターミナル画面があり、Light にはありません。どちらも Tor を内蔵しています。大きさは Mac の 15 メガバイトから 64 ビット ARM の 24 メガバイトまでです。
start-files-machine = あなたのマシン
start-files-forms = Standard、次に Light
start-row-linux-x86_64 = Linux、デスクトップまたはサーバー
start-row-linux-aarch64 = 64 ビットの Raspberry Pi 3、4、5、およびその他の 64 ビット ARM Linux
start-row-linux-armv6 = Raspberry Pi Zero、または 32 ビットで使う Pi
start-row-macos-aarch64 = Mac、Apple シリコン
start-row-macos-x86_64 = Mac、Intel
start-row-windows = Windows
start-files-after = ブラウザで取ったファイルも、<a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a> で確かめ、実行可能にし、PATH に置く必要があります。上のコマンドがしているのはそれです。ここにあるものはどれも開発者証明書で署名されていないため、Mac や Windows は、ブラウザから来たファイルを、あなたが許可するまで開かないことがあります。合うものがない場合や、他人がビルドしたファイルを実行したくない場合は、<a href="https://github.com/needmoretruth/333#install">リポジトリ</a> に Ubuntu、Debian、Fedora、Arch、macOS、Windows でのビルド方法があります。
start-then-title = 次に、順番に
start-step-1 = <b>ファイルを受け取ります。</b> このサイトのノードはいつでも起きています:
start-step-1-after = 最初の <code>join</code> はノードの名前も作ります。名前が 333 で始まる鍵なので、少し時間がかかります。<a href="{ $base }/333">掲示板</a> のどのアドレスも同じように使えます。
start-step-2 = <b>起動します。</b>
start-step-2-after = これ以降、ログアウトしても再起動してもバックグラウンドで動きます。<code>shut</code> と表示されたら、ルーターが他のノードを締め出しています。<code>333 start --tor</code> ならルーターの変更は要りません。
start-step-3 = <b>様子を見ます。</b> <code>333 status</code> は、動いているか、他のノードがどこで届くか、何人が応答しているかを示します。<code>333 logs</code> はノードが書いたものを表示します。
start-step-4 = いつでも <b>止められます</b>: <code>333 stop</code>。もう一度 <code>333 start</code> を実行するまで、再起動しても止まったままです。
start-watch = 働く様子を見たいなら、<code>333 run</code> がこのターミナルで画面付きで動かします。<code>q</code> で止まります。コマンドなしの <code>333</code> はノードの状態と次に打つものを示し、<code>333 help</code> はすべてのコマンドを一覧します。
start-begin = 掲示板に誰もおらず、このサイトのノードも消えていたら、誰かが最初にならなければなりません。<code>333 begin</code> は誰かいれば断り、いなければ誰も署名していない自分の系譜を始めます。記録を読めば誰でもそれがわかります。
start-rule = 自分で起動するまで、何も始まりません。
start-rule-detail = インストーラーはファイルを一つ置くだけで、何も実行しません。ノードが初めて誰かに応答し、初めて集合場所にアドレスを残すのは、あなたが自分で <code>333 start</code> か <code>333 run</code> を実行したときで、そのコマンドが同意した瞬間です。

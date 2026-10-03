# Take the program: the installers, every file, and the first commands.

start-meta-title = 取得程式 · 333
start-meta-description = 在 Linux、macOS、Windows 或樹莓派上安裝 333 客戶端，用該版本的 SHA256SUMS 校驗，然後接收檔案。

start-heading = 取得程式
start-lede = 333 是一個小程式，你讓它在一臺用不上的電腦上一直執行，每當它的另一個副本來敲門，它就在那裡應答。
start-pick-file = 這臺機器用的檔案是 <code data-name="standard">333-x86_64-linux</code>。Light 是沒有終端螢幕的同一個客戶端，檔案是 <code data-name="light">333-light-x86_64-linux</code>。兩者都自帶 Tor。
start-machine = 你的機器
start-pick-linux-x86_64 = Linux，桌上型電腦或伺服器
start-pick-linux-aarch64 = 64 位的樹莓派 3、4、5，或其他 64 位 ARM Linux
start-pick-linux-armv6 = 樹莓派 Zero，或任何 32 位的樹莓派
start-pick-macos-aarch64 = Mac，Apple 晶片
start-pick-macos-x86_64 = Mac，Intel
start-pick-windows = Windows
start-form = 形態
start-standard = Standard
start-light = Light
start-unix-title = 在 Linux 或 Mac 上
start-unix-installer = 在終端裡執行。安裝程式為它所在的機器挑選檔案，用該版本的 <code>SHA256SUMS</code> 校驗，放到 <code>~/.local/bin/333</code>。它不啟動任何東西，也不要密碼。<a href="/install.sh">先讀一讀</a>，它很短。
start-unix-light = 要裝 Light，把結尾的 <code>sh</code> 換成 <code>sh -s -- --light</code>。
start-by-hand = 或者不用安裝程式，手動來
start-unix-by-hand = 同樣的步驟，一步一步。<code>grep</code> 那一行會在移動任何東西之前列印 <code>OK</code>，沒列印的話後面什麼都不會執行。
start-unix-mac = 以上是 x86-64 上的 Linux。在 Mac 上用 <code>shasum -a 256</code> 代替 <code>sha256sum</code>，檔案用下表中 Mac 的那個；其他地方用表中的名字。
start-windows-title = 在 Windows 上
start-windows-installer = 在 PowerShell 裡執行。安裝程式下載 Windows 檔案，用該版本的 <code>SHA256SUMS</code> 校驗，放到 <code>%LOCALAPPDATA%\Programs\333\333.exe</code>。它不啟動任何東西，不需要管理員，只有加上 <code>-AddToPath</code> 才會把那個資料夾加入 PATH。<a href="/install.ps1">先讀一讀</a>，它很短。
start-windows-light = 要裝 Light，改為這樣執行。<code>-AddToPath</code> 同樣加在末尾。
start-windows-by-hand = 同樣的步驟，一步一步。只有檔案的雜湊與 <code>SHA256SUMS</code> 給出的一致，才會把它放到位，最後一行會說明結果。
start-files-title = 所有檔案
start-files-intro = 給上面猜錯了的人，或者更想用瀏覽器下載檔案的人。Standard 有終端螢幕，Light 沒有，兩者都自帶 Tor。大小從 Mac 上的 15 MB 到 64 位 ARM 上的 24 MB。
start-files-machine = 你的機器
start-files-forms = Standard，然後 Light
start-row-linux-x86_64 = Linux，桌上型電腦或伺服器
start-row-linux-aarch64 = 64 位的樹莓派 3、4、5，以及其他 64 位 ARM Linux
start-row-linux-armv6 = 樹莓派 Zero，或任何 32 位的樹莓派
start-row-macos-aarch64 = Mac，Apple 晶片
start-row-macos-x86_64 = Mac，Intel
start-row-windows = Windows
start-files-after = 用瀏覽器下載的檔案，同樣要用 <a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a> 校驗、設為可執行、放進 PATH，這正是上面的命令所做的。這裡沒有任何檔案用開發者證書籤名，所以 Mac 或 Windows 可能拒絕開啟從瀏覽器來的檔案，直到你另行允許。如果都不合適，或者你不想執行別人構建的檔案，<a href="https://github.com/needmoretruth/333#install">程式碼儲存庫</a>裡寫了如何在 Ubuntu、Debian、Fedora、Arch、macOS 和 Windows 上構建。
start-then-title = 然後，按順序
start-step-1 = <b>接收檔案。</b>本站節點全天醒著：
start-step-1-after = 第一次 <code>join</code> 還會生成你節點的名字，一把名字以 333 開頭的金鑰，需要一點時間。<a href="{ $base }/333">公告板</a>上的任何地址都一樣能用。
start-step-2 = <b>啟動它。</b>
start-step-2-after = 從現在起它在後臺執行，登出後、重啟後都是。如果它顯示 <code>shut</code>，說明你的路由器把別人擋在外面：<code>333 start --tor</code> 不需要改路由器。
start-step-3 = <b>檢視它。</b><code>333 status</code> 說明它是否在執行、別人在哪裡能聯絡到它，以及我們中有多少在應答。<code>333 logs</code> 顯示它寫下的內容。
start-step-4 = 隨時可以<b>停止它</b>：<code>333 stop</code>。重啟後它保持停止，直到你再次執行 <code>333 start</code>。
start-watch = 想看它工作，<code>333 run</code> 會在這個終端裡帶著螢幕執行它；<code>q</code> 停止它。只輸入 <code>333</code> 會說明你的節點處於什麼狀態、接下來輸入什麼，<code>333 help</code> 列出所有命令。
start-begin = 如果公告板上沒有人、本站節點也不在了，總得有人做第一個：<code>333 begin</code> 在有人時會拒絕，否則開始一條你自己的譜系，沒有人為它簽名，任何讀你記錄的人都看得到。
start-rule = 在你啟動之前，什麼都不會開始。
start-rule-detail = 安裝程式只放好一個檔案，什麼都不執行。你的節點第一次應答別人、第一次在匯合點留下地址，是在你親自執行 <code>333 start</code> 或 <code>333 run</code> 的時候，那條命令就是你同意的時刻。

### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph. Between two Chinese or Japanese letters a break stays a break.

help-about = 333 的一個節點。被問時應答，儲存自己的記錄，並把檔案傳下去。

help-id = 顯示這個節點的名字，首次執行時建立

help-bootstrap = 沒有人能把檔案交給你時，開始一條新的譜系
help-bootstrap-long = 沒有人能把檔案交給你時，開始一條新的譜系。

    通常的加入方式是用邀請執行 `333 join`。這條命令先檢視匯合點，
    那裡有人就拒絕。沒有人時，它獲取檔案，用本客戶端帶的雜湊
    校驗，然後寫下。你的節點就成為自己譜系的創始者，開端沒有任何
    人的簽名，讀它記錄的任何人都能看到。

help-serve = 在這個終端執行這個節點，直到你停止它
help-serve-long = 在這個終端執行這個節點，直到你停止它。

    它應答心跳和提問，交換所知，並在每個紀元去問抽中由它提問的
    節點。在終端中它會開啟螢幕；`q`、Ctrl-C 或在另一個終端執行
    `333 stop` 都能停止它。`333 start` 在後台做同樣的事。

help-serve-long-light = 在這個終端執行這個節點，直到你停止它。

    它應答心跳和提問，交換所知，並在每個紀元去問抽中由它提問的
    節點，一行一行地輸出。Ctrl-C 或在另一個終端執行 `333 stop` 都能
    停止它。`333 start` 在後台做同樣的事。

help-say = 每個紀元說一次 333 中的一個。傳出去的是編號

help-status = 顯示這個節點是否在執行、別人在哪裡能找到它，以及有多少同伴在應答

help-join = 用邀請從持有檔案的節點那裡接收檔案

help-languages = 列出語言，或為這個節點的所有命令儲存一種語言

help-ping = 聯絡另一個節點，與它交換一次心跳

help-pack = 把這個節點寫進一個檔案，帶到另一台機器
help-pack-long = 把這個節點寫進一個檔案，帶到另一台機器。

    全部帶走：名字、記錄、別人為它簽署的內容、檔案，以及它 onion
    地址的金鑰。之後這個目錄拒絕執行它，名字就不會出現在兩個
    地方。檔案沒有加密：誰持有它，誰就是這個節點。帶走、解包、
    刪除。

help-unpack = 把打包的節點放進這台機器的節點目錄
help-unpack-long = 把打包的節點放進這台機器的節點目錄。

    已有節點的地方會拒絕。檔案讀完、金鑰和記錄校驗通過之前，
    什麼都不寫。

help-moved = 說明這個節點的目錄是移動或改名的，不是複製的
help-moved-long = 說明這個節點的目錄是移動或改名的，不是複製的。

    節點發現自己在新位置時，每次執行都會提醒，直到輸入這條命令，
    因為原件仍在執行的副本會讓一個名字出現在兩個地方。

help-tell = 用螢幕上的說法給執行中的節點下命令
help-tell-long = 用螢幕上的說法給執行中的節點下命令。

    `tor on`、`tor off`、`bridge <line>`、`helper <program>`，以及
    螢幕在 `:` 之後接受的所有詞。執行中的節點執行它，回應顯示在
    這裡。`say`、`join`、`ping`、`begin`、`status` 和 `stop` 不用
    這條命令也能傳給執行中的節點。

help-tell-light = 給執行中的節點下命令
help-tell-long-light = 給執行中的節點下命令。

    `tor on`、`tor off`、`bridge <line>` 和 `helper <program>`。
    執行中的節點執行它，回應顯示在這裡。`say`、`join`、`ping`、
    `begin`、`status` 和 `stop` 不用這條命令也能傳給執行中的節點。

help-service = 直接管理後台服務（`start` 和 `stop` 用的就是它）
help-service-long = 直接管理後台服務（`start` 和 `stop` 用的就是它）。

    你不要求就什麼都不裝，寫的每個檔案、跑的每條命令都會隨時
    顯示，`333 service uninstall` 會全部移除。

help-service-install = 用這些 run 選項安裝後台服務並啟動
help-service-install-long = 用這些 run 選項安裝後台服務並啟動。

    服務按給定的選項原樣為這個節點的目錄執行 `333 run`，旁邊每小時
    檢查一次，節點停了就在這台機器上提示。`333 start` 不帶選項做
    同樣的事。

help-service-uninstall = 停止後台服務並移除它裝的所有東西

help-service-status = 服務管理器的說法、節點最後一次報告醒著的時間，以及最後幾行

help-service-check = 節點停了就在這台機器上提示。服務每小時執行一次；一切正常時
    什麼都不說

help-data-dir = 存放這個節點全部所有物的目錄：名字，以及使用 Tor 時 Tor 的狀態

help-timeout = 每個聯網步驟等待的秒數
help-timeout-long = 每個聯網步驟等待的秒數。

    這是上限，不是延遲。按啟動 Tor 來定，這是唯一可能要幾分鐘的
    步驟。

help-dangerously-trust-directory-permissions = 接受這台機器上其他人能進入的目錄
help-dangerously-trust-directory-permissions-long = 接受這台機器上其他人能進入的目錄。

    目錄裡有這個節點名字的唯一副本，所以權限寬鬆的目錄預設會被
    拒絕。這是給測試目錄和屬主奇怪的容器用的。

help-keep-everything = 永久儲存所有宣告，而不只是判定所用的窗口期
help-keep-everything-long = 永久儲存所有宣告，而不只是判定所用的窗口期。

    這不會改變任何人的狀態：每條宣告無論存在哪裡，驗證結果都一樣。

help-bridges = 一行網橋，用於封鎖了 Tor 常規入口的網路
help-bridges-long = 一行網橋，用於封鎖了 Tor 常規入口的網路。

    每座拿到的網橋給一次，照拿到時的樣子原樣給出。這裡不會獲取
    網橋：網橋是人有意分發的，為的是沒有哪份清單能被直接收集並
    封鎖。

help-bridge-helper = 講混淆網橋協定的程式，按名字或路徑
help-bridge-helper-long = 講混淆網橋協定的程式，按名字或路徑。

    只有網橋行需要、且路徑上的不是 `lyrebird` 時才需要。不隨附，
    因為凍結的副本很快就會過時。

help-language = 使用的語言標籤：`ko`、`es`、`zh-Hant`
help-language-long = 使用的語言標籤：`ko`、`es`、`zh-Hant`。

    不指定時，先看 `THE333_LANGUAGE`，再看 `333 language <TAG>`
    儲存的語言，最後是英語。不使用系統語言。`333 language` 列出有
    詞語的語言，在 `<data-dir>/words/<tag>/` 放一個目錄的詞庫，
    不用編譯就能加一種語言。333 的詞語本身永不翻譯。

help-count-in = 用十進位制、十二進位制或 twelve-ascii 計數
help-count-in-long = 用十進位制、十二進位制或 twelve-ascii 計數。

    顯示的每個數都用它寫，輸入的每個數都按它讀：十二進位制的
    `say 238` 就是十進位制的 `say 332`。名字、地址、埠和版本永不
    換算，網上傳的內容也不變。不指定時，先看 `THE333_COUNT_IN`，
    再用十進位制。

help-bootstrap-meet = 獨自開始之前去哪裡找人

help-bootstrap-anyway = 即使已有人也開始

help-serve-bind = 監聽的地址和埠

help-serve-tor = 同時開一個 onion 地址，讓別人不知道位置也能聯絡到這個節點。
    喚醒 Tor 需要幾秒到幾分鐘

help-serve-no-direct = 完全不開通訊端。只能與 --tor 一起用；你的地址完全不上網

help-serve-announce = 告訴其他節點用來聯絡這個節點的地址
help-serve-announce-long = 告訴其他節點用來聯絡這個節點的地址。

    通訊端自己說不出時需要：監聽所有網絡卡時，或在轉發埠的裝置
    後面時。

help-serve-no-mdns = 不在本地網路中宣告這個節點在這裡
help-serve-no-mdns-long = 不在本地網路中宣告這個節點在這裡。

    否則宣告出去的是這台機器上有東西講 333 以及埠，不是這個
    節點的名字。同一個家裡的兩個節點就是這樣不用邀請互相找到的。

help-serve-no-router = 不請路由器把埠轉給這台機器
help-serve-no-router-long = 不請路由器把埠轉給這台機器。

    家用路由器會丟棄裡面沒人請求過的東西，直到裡面的程式通過
    UPnP-IGD、PCP 或 NAT-PMP 請它轉發埠。這會改變網路，所以發生
    時會顯示。`--no-upnp` 是它的舊名字。

help-serve-meet = 去哪裡找沒人介紹給它的節點
help-serve-meet-long = 去哪裡找沒人介紹給它的節點。

    一個固定地址，存放關於節點在哪裡的簽名宣告。在那裡讀到的
    一切都在這裡驗證。

help-serve-no-meet = 完全不用匯合點
help-serve-no-meet-long = 完全不用匯合點。

    這樣只有拿到邀請的人和這個網路上的節點能聯絡到這個節點，
    其他人都不行。

help-serve-plain = 逐行輸出，而不畫螢幕
help-serve-plain-long = 逐行輸出，而不畫螢幕。

    不在終端時總是逐行輸出；這個選項讓終端裡也這樣。

help-serve-plain-light = 逐行輸出，這個版本總是如此
help-serve-plain-long-light = 逐行輸出，這個版本總是如此。

    這個版本沒有螢幕。接受這個選項，是為了同一行命令在兩個版本
    裡都能用。

help-say-index = 哪一個，0 到 { $last }，按計數的進位制（--count-in）輸入。詞語還沒寫

help-status-sources = 列出這個節點持有的所有地址：屬於誰、最早在哪裡何時聽說、
    最後在哪裡

help-status-json = 這個節點觀察到的內容，以 JSON 供程式讀取。不含任何地址或埠

help-join-address = 已持有檔案的人給的邀請（`333:host:port`）

help-ping-address = 邀請（`333:host:port`）或地址：`host`、`host:port`、
    `[::1]:port` 或 `某個.onion`（經由 Tor）

help-pack-file = 要寫的檔案。它必須還不存在

help-pack-undo = 搬家放棄時，撤銷這裡的打包
help-pack-undo-long = 搬家放棄時，撤銷這裡的打包。

    僅當檔案從未在任何地方解包時：若解包過，這會變成兩個。

help-unpack-file = `333 pack` 寫出的檔案

help-tell-order = 命令，按在螢幕裡輸入的樣子

help-tell-order-light = 命令，寫法如 `tor on` 或 `bridge <line>`

help-service-install-flags = `run` 的選項，按你在它後面輸入的樣子

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = 顯示幫助
help-print-help-more = 顯示幫助（用 '--help' 看更多）
help-print-help-summary = 顯示幫助（用 '-h' 看摘要）
help-print-version = 顯示版本
help-print-this = 顯示這條訊息或給定子命令的幫助
help-print-for = 顯示子命令的幫助

help-start = 在後台執行這個節點，現在和每次重啟後

help-stop = 停止這個節點，重啟後也保持停止

help-restart = 停止這個節點，再在後台執行它

help-logs = 顯示這個節點在後台執行時寫的最後幾行

help-logs-follow = 新行一來就繼續顯示（由 systemd 儲存時）

help-invite = 顯示別人通過這個節點加入所用的邀請

help-status-all = 顯示這個節點知道的一切，並說明每部分的含義

help-languages-tag = 要儲存的語言標籤：`ko`、`en`。`en` 回到英語

help-start-example = 範例：333 start

help-stop-example = 範例：333 stop

help-restart-example = 範例：333 restart

help-status-example = 範例：333 status --all

help-logs-example = 範例：333 logs -f

help-id-example = 範例：333 name

help-invite-example = 範例：333 invite

help-bootstrap-example = 範例：333 begin

help-serve-example = 範例：333 run --tor

help-say-example = 範例：333 say 7

help-join-example = 範例：333 join 333:192.0.2.7:3333

help-languages-example = 範例：333 language zh-Hant

help-ping-example = 範例：333 ping 333:192.0.2.7:3333

help-pack-example = 範例：333 pack node.333

help-unpack-example = 範例：333 unpack node.333

help-moved-example = 範例：333 moved

help-tell-example = 範例：333 tell tor on

help-service-example = 範例：333 service status

help-service-install-example = 範例：333 service install --tor

help-service-uninstall-example = 範例：333 service uninstall

help-service-status-example = 範例：333 service status

help-service-check-example = 範例：333 service check

help-start-flags = `run` 的選項，按你在它後面輸入的樣子。之後每次啟動都沿用，
    直到給出別的選項

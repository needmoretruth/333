# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = 複製
js-copied = 已複製
js-selected = 已選中
js-state-awake = 本站節點醒著
js-state-not-running = 本站節點沒有執行
js-in-hours = { $h } 小時 { $m } 分鐘後
js-in-minutes = { $m } 分鐘後
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = 本譜系第 { $n } 個紀元

## The network

js-network-state-founder = 不在任何名冊上
js-network-state-ok = 本紀元應答
js-network-state-quiet = 本紀元沉默
js-network-state-later = 從之後的紀元起計數
js-network-state-seen = 見過，不在名冊上
js-network-awake = 醒著
js-network-not-running = 沒有執行
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · 你的
js-network-find-bad = 節點的名字是十六進位制，至少輸入前 6 個字元。
js-network-find-none = 本站節點沒見過叫這個名字的節點。
js-network-find-many = 有 { $count } 個節點以此開頭。請多輸入一些名字。
js-network-find-marked = 已在此裝置上標記為你的。
js-network-select = 選擇一個節點，檢視本站節點對它的瞭解。
js-network-role-founder = 本譜系的創始者
js-network-role-site = 本站節點
js-network-role-yours = 你的，在此裝置上
js-network-role-none = 網路上的一個節點
js-network-col-name = 名字
js-network-col-state = 本紀元
js-network-col-given = 收到檔案
js-network-col-counted = 計數起點
js-network-col-answered = 上次應答
js-network-col-said = 已說
js-network-col-reached = 到達方式
js-network-row-said = 本紀元已說
js-network-row-handed = 把檔案交給了
js-network-row-testimony = 見證
js-network-given-by = 紀元 { $epoch }，來自 { $sponsor }
js-network-given-founder = 無人交給。它開始了這條譜系。
js-network-given-none = 不在名冊上
js-network-epoch = 紀元 { $epoch }
js-network-epoch-now = { $epoch }（本紀元）
js-network-epoch-ago = { $epoch }（{ $ago }前）
js-network-more = 另有 { $count } 個在下表中
js-network-nothing = 無
js-network-reach-direct = 直接
js-network-reach-tor = 經由 Tor
js-network-reach-tor-short = Tor
js-network-reach-unknown = 未知
js-network-testimony = 被問 { $asked } 次，問了 { $asking } 次（最近 3 個紀元）
js-network-copy-name = 複製名字
js-network-select-name = 選中上面的名字
js-network-mine = 這是我的節點
js-network-tag-founder = 創始者
js-network-tag-site = 本站
js-network-tag-yours = 你的
js-network-empty = 本站節點還沒見過其他節點。
js-network-this-node = 這個節點
js-network-yes = 是
js-network-no = 否
js-network-none = 無

## Where we are

js-map-watch = 即時檢視
js-map-stop = 停止檢視
js-map-read-at = 讀取於 { $read_at } UTC。
js-map-unreadable = 剛才無法讀取公告板。
js-map-tor = Tor
js-map-nowhere = 無法定位的地方
js-map-nobody = 沒有人說自己在哪裡。
js-map-all = 所有說明位置的
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = 在網路上，但沒說在哪裡
js-map-dot = { $count } 個節點

## The board

js-board-said = 紀元 { $epoch } 由 { $node } 說出
js-board-site = 本站節點
js-board-tor = 經由 Tor

## Take the program

js-start-machine-linux-x86_64 = x86-64 上的 Linux
js-start-machine-linux-aarch64 = 64 位 ARM 上的 Linux
js-start-machine-linux-armv6 = 32 位 ARM 上的 Linux
js-start-machine-macos-aarch64 = Apple 晶片的 Mac
js-start-machine-macos-x86_64 = Intel 晶片的 Mac
js-start-machine-windows-x86_64 = Windows
js-start-phone = 這看起來是手機或平板，而這個程式是給一直開著的電腦用的。請在這裡選擇那臺電腦。
js-start-unknown = 這個瀏覽器沒有說明它執行在什麼上。請在這裡選擇你的機器。
js-start-sure = 這個瀏覽器說它執行在{ $machine }上，所以這裡選了它。
js-start-mac = 這個瀏覽器說它在 Mac 上，但沒說是哪種晶片，所以這裡選了 Apple 晶片。安裝程式會直接詢問機器本身。
js-start-linux = 這個瀏覽器說它在 Linux 上，但沒說是哪種處理器，所以這裡選了 x86-64。安裝程式會直接詢問機器本身。
js-start-chosen = 已在上方選擇

## The story on the home page, drawn

js-story-file = 333.txt · 3 位元組
js-story-gave = 我把它交給了你
js-story-received = 我從你那裡收到了它
js-story-signed = 已簽名
js-story-minutes = 333 分鐘
js-story-epochs = 333 個紀元
js-story-now = 現在
js-story-answering = 應答中
js-story-roll = 名冊上
js-story-years = { $years } 年

### `333 unpack`: putting a packed node into this machine's node directory.

-unpack-nothing = 沒有解包任何東西。
-unpack-so-nothing = 所以沒有解包任何東西。

unpack-kept = 這個目錄裡已經住著一個節點。要在它旁邊解包，用 --data-dir 給它
    一個自己的目錄。

unpack-seed-in = { $file } 中的{ $seed }

unpack-not-one-node = 該檔案自稱包含 { $claimed }，裡面的金鑰卻是 { $name }。{ -unpack-not-one }
-unpack-not-one = 它不是同一個節點，沒有解包任何東西。

unpack-taken = 另一個 333 佔用瞭解包的目標目錄。{ -unpack-nothing }

unpack-elsewhere = 333 --data-dir <另一個目錄> unpack { $file }

unpack-could-not-open = { $target } 裡已經住著一個節點，而且{ -unpack-could-not-be-opened } 要在它旁邊解包：{ $elsewhere }
-unpack-could-not-be-opened = 無法開啟它看裡面有什麼。{ -unpack-nothing }

unpack-no-record = 還沒有記錄
unpack-epochs-of-record = { $epochs ->
       *[other] { $epochs } 個紀元的記錄
    }
unpack-holding = 持有檔案
unpack-not-holding = 未持有檔案

unpack-occupied = { $target } 裡已經住著一個節點：
    { $name }，{ $epochs }，{ $holding }。
    在上面解包會永遠丟掉這一切，{ -unpack-so-nothing }
    要在它旁邊解包，給它一個自己的目錄：
    { $elsewhere }

unpack-holds-files = { $target } 裡有檔案但沒有節點。{ -unpack-its-own } 解包到別處：{ $elsewhere }
-unpack-its-own = 節點要解包到它自己的目錄，{ -unpack-so-nothing }

unpack-opening-the-record = 正在開啟記錄
unpack-torn = 該檔案中的記錄殘缺，所以不是完整的節點。{ -unpack-nothing }
unpack-reading-the-record = 正在讀取記錄
unpack-does-not-verify = 該檔案中的記錄無法通過校驗。{ -unpack-nothing }
unpack-another-key = 該檔案中的記錄由另一把金鑰寫成。{ -unpack-nothing }

unpack-not-a-place = { $target } 不是可以放節點的目錄
unpack-making-room = 正在 { $target } 騰出位置
unpack-putting = 正在把節點放進 { $target }

unpack-name = { $name }
    .keyword = 名字

unpack-record-none = 還沒有
    .keyword = 記錄

unpack-record = { $epochs ->
       *[other] { $epochs } 個紀元，已校驗
    }
    .keyword = 記錄

unpack-holding-the-file = 檔案
    .keyword = 持有

unpack-onion-key = 它 onion 地址的金鑰，所以地址也一起來了
    .keyword = 隱藏

unpack-unpacked = 放進 { $target }，
    來自 { $packed } 打包的檔案。
    現在它就是這個節點，那個檔案也是：刪掉 { $file }
    .keyword = 已解包

unpack-next = { $serve }
    .keyword = 下一步

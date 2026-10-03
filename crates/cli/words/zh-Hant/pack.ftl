### `333 pack`: writing this node into one file, to be carried to another machine.

pack-name-the-file = 請指定打包這個節點的檔案：333 pack <FILE>

pack-no-node = { $root } 裡沒有可打包的節點。什麼都沒寫。

pack-already-exists = { $file } 已存在。{ -pack-never-over }
-pack-never-over = 打包只寫新檔案，從不覆蓋舊檔案；換個名字。

pack-not-marked = 已寫入 { $file }，但這個目錄無法標記為已打包。{ -pack-until-it-is }
-pack-until-it-is = 在標記之前，這個節點同時住在兩處：{ -pack-delete-that-file }
-pack-delete-that-file = 在這裡執行任何東西之前，刪掉那個檔案。

pack-creating = 正在建立 { $file }

pack-name = { $name }
    .keyword = 名字

pack-record-none = 還沒有
    .keyword = 記錄

pack-record = { $epochs ->
       *[other] { $epochs } 個紀元，一起帶走
    }
    .keyword = 記錄

pack-witnessed = { $statements ->
       *[other] 其他金鑰為它簽署的宣告 { $statements } 條，一起帶走
    }
    .keyword = 見證

pack-holding = 檔案，一起帶走
    .keyword = 持有

pack-onion-key = 它 onion 地址的金鑰，所以地址也一起帶走
    .keyword = 隱藏

pack-carrying = 這個節點，寫進 { $file }。
    那個檔案就是這個節點：誰持有它，誰就能以這個名字應答。帶走、
    解包，然後刪掉；它不是要留著的備份。它沒有加密，因為密碼會是
    又一樣可能丟的東西，而丟了密碼就和丟了檔案一樣會丟掉名字。
    和這個目錄一樣，只有你能讀它。
    .keyword = 攜帶

pack-packed = { $bytes } 位元組。
    { $root } 中的任何東西都不會再作為這個節點行動。
    .keyword = 已打包

pack-next = 在另一台機器上：333 unpack { $carried }
    如果放棄搬家：{ $undo }
    .keyword = 下一步

pack-not-packed = 這個節點沒有打包，所以 { $root } 裡沒有可撤銷的
    .keyword = 這裡

pack-restored = 這個節點又住回了 { $root }。
    它被打包進的檔案仍是這個名字。如果它在別處解包過，兩者中的一個
    必須先刪掉，才能執行任何一個；如果沒有，就刪掉 { $file }
    .keyword = 已恢復

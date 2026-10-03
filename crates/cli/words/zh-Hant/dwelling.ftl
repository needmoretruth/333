### Where this node lives, and whether it still lives here.

-dwelling-would-be-one = 會讓同一個
-dwelling-anywhere-this = 在任何地方解包，這條命令

dwelling-made-at = 創建於
dwelling-unpacked-at = 解包於
dwelling-moved-to = 據稱移到了
dwelling-found-at = 本客戶端首次開啟於

dwelling-writing = 正在寫入 { $file }
dwelling-putting-in-place = 正在放置 { $file }
dwelling-resolving = 正在解析 { $dir }

dwelling-elsewhere = 這個節點曾{ $how } { $was }，
    現在位於 { $now }。
    如果目錄只是移動或改名，那就沒事。如果是複製的，而原來的仍在
    執行，那就是同一個名字在兩個地方，彼此的記錄會互相矛盾。
    等只剩一個時，在這裡說明：{ $settle }
    .keyword = 住處

dwelling-unread-moment = 無法讀取的時刻
dwelling-unread-file = 名字無法讀取的檔案

dwelling-packed = 這個節點在 { $at } 為搬家打包進了 { $into }。
    那個檔案在哪裡解包，它就住在哪裡。在這裡也執行它，{ -dwelling-would-be-one }
    名字出現在兩個地方，所以 { $home } 中沒有任何東西會作為它行動。

    如果放棄了搬家，那個檔案也沒有{ -dwelling-anywhere-this }
    會把它放回原處：{ $undo }

dwelling-unmarking = 正在去掉這個節點已打包的標記

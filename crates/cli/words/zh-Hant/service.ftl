### `333 service`: running the node through logouts and reboots, when asked to.

service-mind = { $node } 位於這個系統會清空的地方，而這個節點的名字別處都沒有。
    服務會在那裡執行它，直到被清空。
    .keyword = 注意

service-runs = { $command }
    .keyword = 節點

service-undo-partial = `333 service uninstall` 會移除這些步驟裡已經做了的部分。
    .keyword = 撤銷

service-no-receipt-directory = 這個系統沒有給出存放憑據的配置目錄

service-wrote-receipt = { $path }，`333 service uninstall` 靠它知道要撤銷什麼。
    .keyword = 已寫入

service-undo = `333 service uninstall` 會停止節點並撤銷以上全部。
    節點自己的目錄兩者都不會碰。
    .keyword = 撤銷

service-uninstalled = 不再由服務執行。{ $node } 保持節點離開時的樣子：`333 run` 手動
    執行它，`333 start` 重新設定服務。
    .keyword = 節點

service-none-installed = 沒有為這個使用者通過 `333 service install` 安裝過。
    .keyword = 服務

service-state = { $state }
    .keyword = 服務

service-node = { $node }
    .keyword = 節點

service-last-awake = 最後一次是 { $at }，{ $ago }前
    .keyword = 醒著

service-never-awake = 在這個目錄裡從沒報告過
    .keyword = 醒著

service-said-nothing = 沒有保留下來的
    .keyword = 輸出

service-said-last = { $lines ->
       *[other] 最後 { $lines } 行：
    }
    .keyword = 輸出

service-no-manager = 這個系統沒有 `333 service` 會詢問的服務管理器。`333 run --plain`
    可在這裡讓程式持續執行的東西下執行節點。

service-not-installed-here = 未安裝：這裡沒有它認識的服務管理器

# Said by every service manager's own file.

service-creating = 正在建立 { $path }
service-writing = 正在寫入 { $path }
service-removing = 正在移除 { $path }

service-wrote = { $path }
    .keyword = 已寫入

service-removed = { $path }
    .keyword = 已移除

service-left = { $path }。不是 `333 service install` 寫的。
    .keyword = 保留

service-failed = { $why }
    .keyword = 失敗

service-not-installed = 未安裝
service-running = 執行中
service-starting = 啟動中

### `333 service` on macOS: launchd.

service-launchd-no-home = 這個系統沒有給出這個使用者的主目錄
service-launchd-asking-who = 正在問 `id -u` 這是哪個使用者

service-launchd-login = launchd 從你登入到登出一直執行節點，重啟後你登入時再重新開始。
    它的輸出在 { $log }。
    .keyword = 登入

service-launchd-ended-with = { $state }，上次以 { $code } 結束
service-launchd-loaded = 已載入，launchd 沒有說它在做什麼

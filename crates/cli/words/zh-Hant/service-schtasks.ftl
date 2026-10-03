### `333 service` on Windows: Task Scheduler.

service-schtasks-cannot-pass = `{ $word }` 含有 " 或 %，cmd.exe 無法原樣傳遞
service-schtasks-no-user = Windows 沒有說明這是哪個使用者（%USERDOMAIN% 和 %USERNAME%）

service-schtasks-logon = 你登入期間，Windows 從登入起執行節點。開機就執行的服務需要專用
    賬戶，而節點住在你自己的目錄裡。
    它的輸出在 { $log }。
    .keyword = 登入

service-schtasks-ready = 已停止，333 秒內會再次啟動
service-schtasks-disabled = 已停用：不會自己再啟動

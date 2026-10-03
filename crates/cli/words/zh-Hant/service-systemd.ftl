### `333 service` on Linux: systemd.

service-systemd-no-configuration = 這個系統沒有給出這個使用者的配置目錄

service-systemd-no-session = systemd 在這裡沒有為這個使用者維持會話（{ $why }）。這個使用者在
    控制台或通過 ssh 登入時才會開始，su 或 sudo 不行。以這個使用者
    登入後再執行一次。

service-systemd-wrote-over = { $path }，替換了原來的
    .keyword = 已寫入

service-systemd-linger-already = 已為 { $user } 開啟。登出後節點繼續執行，開機時無人登入也會啟動。
    .keyword = linger

service-systemd-linger-on = 已為 { $user } 開啟。登出後節點繼續執行，開機時無人登入也會啟動。
    .keyword = linger

service-systemd-linger-not-on = 未開啟：{ $why }。沒有它，節點會在你登出時停止，重啟後等你再
    登入。`sudo loginctl enable-linger { $user }` 可開啟。
    .keyword = linger

service-systemd-linger-off = 已重新關閉，與安裝前一樣。
    .keyword = linger

service-systemd-linger-left = 保持原樣。安裝時並沒有開啟它。
    .keyword = linger

service-systemd-not-answering = 未知：systemd 沒有為這個使用者應答
service-systemd-restarting = 已停止，停止 333 秒後會再次啟動
service-systemd-stopping = 正在停止
service-systemd-failed = 失敗（{ $result }）
service-systemd-stopped = 已停止
service-systemd-at-every-boot = { $state }，每次開機時啟動
service-systemd-not-again = { $state }，{ $file_state }：不會自己再啟動

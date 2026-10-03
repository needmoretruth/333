### Reaching another node, whichever way its address says to.

dial-would-show = 這個節點隱藏自己的地址，所以不會開啟到 { $address } 的連線，
    那會暴露地址

dial-no-answer = { $seconds } 秒後仍無應答

dial-waking = 值得聯絡的人在隱藏地址上，而 Tor 沒有執行。首次啟動需要幾秒到
    幾分鐘，結束前不會向任何人提問。
    .keyword = 喚醒

dial-unwoken = Tor 未能啟動：{ $why }
    本紀元跳過隱藏地址。它們後面的節點並非沒有應答，而是沒有
    任何東西到達那裡去提問。
    .keyword = 未喚醒

dial-connecting = 正在連線 { $address }
dial-without-tor = 本客戶端編譯時沒有 Tor，無法到達 { $address }

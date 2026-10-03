### Reaching another node, whichever way its address says to.

dial-would-show = 这个节点隐藏自己的地址，所以不会打开到 { $address } 的连接，
    那会暴露地址

dial-no-answer = { $seconds } 秒后仍无应答

dial-waking = 值得联系的人在隐藏地址上，而 Tor 没有运行。首次启动需要几秒到
    几分钟，结束前不会向任何人提问。
    .keyword = 唤醒

dial-unwoken = Tor 未能启动：{ $why }
    本纪元跳过隐藏地址。它们后面的节点并非没有应答，而是没有
    任何东西到达那里去提问。
    .keyword = 未唤醒

dial-connecting = 正在连接 { $address }
dial-without-tor = 本客户端编译时没有 Tor，无法到达 { $address }

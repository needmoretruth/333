### `333 run`: trading with the other nodes once an epoch, and asking whoever was drawn.

hours-asking-failed-gathering = 正在汇集这个节点可以传出的内容：{ $why }
    .keyword = 失败

hours-asking-quiet = { $address }：{ $why }
    .keyword = 沉默

hours-asking-unended = 交换没有在本纪元的 { $time } 内完成，剩下的时间不会等它
    .keyword = 未完

hours-asking-reading-address = 正在读取对方的地址
hours-asking-exchanging-heartbeats = 正在交换心跳
hours-asking-trading = 正在交换声明
hours-asking-putting = 正在提问
hours-asking-sealing-presenting = 正在封存这个节点来说的话
hours-asking-saying-what-for = 正在说明这个节点为何而来
hours-asking-sealing-silence = 正在封存没有发生的事

hours-asking-no-answer = 无应答
hours-asking-did-not-answer = { $address } 没有应答
hours-asking-did-not-finish = { $address } 没有完成心跳
hours-asking-neither = { $address } 既没提问也没挂断
hours-asking-within = 在窗口允许的 { $seconds } 秒内{ $what }
hours-asking-nothing-within = 在窗口允许的 { $seconds } 秒内什么都没有

hours-asking-going = 外面没人能连到这个节点，所以它主动去找本纪元被抽中来问它的
    { $drawn } 位同伴。抽签由纪元和密钥决定，所以这个节点不用别人
    告诉就知道是谁。
    .keyword = 前往

hours-asking-unknown-drawn-by = 被抽中由一位没人说过在哪的同伴来问
    .keyword = 不明

hours-asking-unasked = 纪元 { $epoch }：{ $why }
    .keyword = 未问

hours-asking-drawn = 纪元 { $epoch } — 要问 { $asked } 位同伴。没有人挑选：名字由纪元
    和密钥决定，在每台机器上都一样。
    .keyword = 抽签

hours-asking-unknown-drawn-to-ask = 被抽中去问一位没人说过在哪的同伴
    .keyword = 不明

hours-asking-unheard = 纪元 { $epoch }：{ $why }
    .keyword = 未听到

hours-asking-witness = 纪元 { $epoch } 由 { $prover } 应答
    .keyword = 见证

hours-asking-silence = 纪元 { $epoch }：{ $why }
    .keyword = 沉默

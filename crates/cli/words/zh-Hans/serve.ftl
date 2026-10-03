### `333 run`.

serve-nothing-listening = 不会有任何东西在监听：--no-direct 需要 --tor

serve-name = { $name }
    .keyword = 名字

serve-waiting-for-the-file = 这个节点还没拿到文件，所以还没有为它计数，也还没有可供任何人
    见证的东西。它自己造不出文件。文件只能来自已经持有它的人，
    你们双方都为交接签名。请要一张邀请，然后运行
    `333 join 333:对方地址:3333`。在此期间应答不花任何代价，
    这也是别人找到你的方式。
    .keyword = 等待

serve-hand = 邀请指向的是地点，不是人。在那里应答的，靠持有自己的密钥证明
    自己是谁。
    .keyword = 信任

serve-invite = { $invitation }
    .keyword = 邀请

serve-answer = { $bound }
    .keyword = 应答

serve-nearby = 正在本网络宣告这里有东西讲 333，并收听其他节点。不是这个节点
    的名字：发出去的，就是同一网络的端口扫描能发现的东西。
    --no-mdns 让这个节点不参与。
    .keyword = 附近

serve-nearby-failed = 无法开始在本网络宣告这个节点在这里：{ $why }
    .keyword = 附近

serve-meet = { $place } 是这个节点寻找没人介绍过的同伴的地方。那里读到的一切都
    由说话者本人签名，那里的东西一概不信。--no-meet 让这个节点远离它。
    .keyword = 汇合

serve-listener-stopped = 一个监听器意外停止了

serve-farewell = 在纪元 { $epoch } 结束。这期间被抽中来问你的人，会签名说他们问了
    却什么也没听到，你的窗口期读的就是这个。窗口期长 { $window } 个
    纪元，并且在移动。
    .keyword = 节点

serve-farewell-on-no-roll = 在纪元 { $epoch } 结束。你不在任何名册上，所以没有人出去问你，
    这期间也不会有任何关于你的签名。
    .keyword = 节点

### `333 run`.

serve-nothing-listening = 不會有任何東西在監聽：--no-direct 需要 --tor

serve-name = { $name }
    .keyword = 名字

serve-waiting-for-the-file = 這個節點還沒拿到檔案，所以還沒有為它計數，也還沒有可供任何人
    見證的東西。它自己造不出檔案。檔案只能來自已經持有它的人，
    你們雙方都為交接簽名。請要一張邀請，然後執行
    `333 join 333:對方地址:3333`。在此期間應答不花任何代價，
    這也是別人找到你的方式。
    .keyword = 等待

serve-hand = 邀請指向的是地點，不是人。在那裡應答的，靠持有自己的金鑰證明
    自己是誰。
    .keyword = 信任

serve-invite = { $invitation }
    .keyword = 邀請

serve-answer = { $bound }
    .keyword = 應答

serve-nearby = 正在本網路宣告這裡有東西講 333，並收聽其他節點。不是這個節點
    的名字：發出去的，就是同一網路的埠掃描能發現的東西。
    --no-mdns 讓這個節點不參與。
    .keyword = 附近

serve-nearby-failed = 無法開始在本網路宣告這個節點在這裡：{ $why }
    .keyword = 附近

serve-meet = { $place } 是這個節點尋找沒人介紹過的同伴的地方。那裡讀到的一切都
    由說話者本人簽名，那裡的東西一概不信。--no-meet 讓這個節點遠離它。
    .keyword = 匯合

serve-listener-stopped = 一個監聽器意外停止了

serve-farewell = 在紀元 { $epoch } 結束。這期間被抽中來問你的人，會簽名說他們問了
    卻什麼也沒聽到，你的窗口期讀的就是這個。窗口期長 { $window } 個
    紀元，並且在移動。
    .keyword = 節點

serve-farewell-on-no-roll = 在紀元 { $epoch } 結束。你不在任何名冊上，所以沒有人出去問你，
    這期間也不會有任何關於你的簽名。
    .keyword = 節點

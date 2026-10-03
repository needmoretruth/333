### `333 join`: asking a node that holds the file to hand it over.

join-name = { $name }
    .keyword = 名前

join-knocking = { $address }
    .keyword = 訪問

join-silence = { $address } では誰にも届きませんでした。333 が終わった証拠では
    ありません。このクライアントが持つのはファイルのハッシュで、
    ファイルではありません。入るには、持っている人から受け取るしか
    ありません。
    .keyword = 沈黙

join-knocking-on = { $address } を訪ねています
join-exchanging = 鼓動を交換中
join-asking = ファイルを求めています
join-no-answer = { $seconds } 秒たっても { $address } から応答がありません

join-given = { $giver } から
    .keyword = 受領

join-joined = エポック { $epoch }
    .keyword = 参加

join-holding = ファイルを持ち、渡すことができます
    .keyword = 保持

join-roll = { $members } 人
    .keyword = 名簿

join-counted = エポック { $epoch } から。一つも早くはなりません。境界二つ先、
    このエポックのどこで着いたかにより { $least }〜{ $most } 分後です。
    それまでは、尋ねられることにすべて答えてください。その間に
    証言されたことが、あなたがここにいた証拠のすべてです。
    .keyword = 計数

join-vigil = これからは `333 start` で動かし続けます。誰も届かないノードは
    何も証言されず、この期間の証言は一度きりか、ゼロです。
    .keyword = ノード

join-already-given = このノードはエポック { $epoch } に { $giver } からファイルを受け取っています。
    求めるものはなく、何も求めませんでした。
join-same-handover = このノードと { $peer } はエポック { $epoch } にすでにファイルを
    受け渡しています。同じエポックに返すのは、その受け渡しを反対側
    から読んだものにすぎず、誰も加入させないので、何も求めませんでした。

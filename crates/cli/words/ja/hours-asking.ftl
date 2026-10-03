### `333 run`: trading with the other nodes once an epoch, and asking whoever was drawn.

hours-asking-failed-gathering = このノードが渡せるものを集めています: { $why }
    .keyword = 失敗

hours-asking-quiet = { $address }: { $why }
    .keyword = 沈黙

hours-asking-unended = このエポックの { $time } 以内に交換が終わらず、残りの時間は
    それを待ちません
    .keyword = 未完

hours-asking-reading-address = 相手のアドレスを読み込み中
hours-asking-exchanging-heartbeats = 鼓動を交換中
hours-asking-trading = 言明を交換中
hours-asking-putting = 質問中
hours-asking-sealing-presenting = このノードが言いに来たことを封印中
hours-asking-saying-what-for = このノードが何をしに来たかを伝えています
hours-asking-sealing-silence = 起きなかったことを封印中

hours-asking-no-answer = 応答なし
hours-asking-did-not-answer = { $address } は応答しませんでした
hours-asking-did-not-finish = { $address } は鼓動を終えませんでした
hours-asking-neither = { $address } は質問も切断もしませんでした
hours-asking-within = 期間が許す { $seconds } 秒以内に { $what }
hours-asking-nothing-within = 期間が許す { $seconds } 秒以内に何もなし

hours-asking-going = 外からこのノードへの接続を開けないので、このエポックで
    このノードに質問するよう抽選された { $drawn } 人のところへ
    自分から出向きます。抽選はエポックと鍵から決まるので、言われ
    なくても相手がわかります。
    .keyword = 出向く

hours-asking-unknown-drawn-by = 居場所を誰も告げていない仲間から質問される抽選に当たりました
    .keyword = 不明

hours-asking-unasked = エポック { $epoch }: { $why }
    .keyword = 未質問

hours-asking-drawn = エポック { $epoch } — 質問する相手は { $asked } 人。誰も選んで
    いません。名前はエポックと鍵から、どのマシンでも同じに決まります。
    .keyword = 抽選

hours-asking-unknown-drawn-to-ask = 居場所を誰も告げていない仲間に質問する抽選に当たりました
    .keyword = 不明

hours-asking-unheard = エポック { $epoch }: { $why }
    .keyword = 未聴取

hours-asking-witness = エポック { $epoch } に { $prover } が応答
    .keyword = 証言

hours-asking-silence = エポック { $epoch }: { $why }
    .keyword = 沈黙

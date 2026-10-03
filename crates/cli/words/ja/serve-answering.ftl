### `333 run`: what a peer can ask for at the door, and what this node does about it.

serve-answering-asked = エポック { $epoch }、{ $verifier } から
    .keyword = 質問

serve-answering-empty = 誰かがファイルを求めました。このノードには渡すものがありません。
    .keyword = 空

serve-answering-gave = エポック { $epoch } に { $receiver } へファイルを
    .keyword = 渡した

serve-answering-roll = { $members } 人
    .keyword = 名簿

serve-answering-cursed = { $name } が尋ねました。333 は扉ごとにそうするとおり、
    その命から { $milliseconds } ミリ秒を奪いました。
    .keyword = 呪い

serve-answering-early = 誰かがエポック { $asked_about } について質問されに来ましたが、
    このノードは { $now } にいます
    .keyword = 早い

serve-answering-witness = エポック { $epoch } に { $prover } が応答。質問されにここへ来ました
    .keyword = 証言

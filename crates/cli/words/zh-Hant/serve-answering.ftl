### `333 run`: what a peer can ask for at the door, and what this node does about it.

serve-answering-asked = 紀元 { $epoch }，由 { $verifier }
    .keyword = 被問

serve-answering-empty = 有人索取檔案。這個節點沒有可給的。
    .keyword = 空

serve-answering-gave = 在紀元 { $epoch } 把檔案交給 { $receiver }
    .keyword = 已交出

serve-answering-roll = { $members } 位同伴
    .keyword = 名冊

serve-answering-cursed = { $name } 問了。333 像在每扇門前那樣，從對方的生命裡拿走了
    { $milliseconds } 毫秒。
    .keyword = 詛咒

serve-answering-early = 有人來接受關於紀元 { $asked_about } 的提問，而這個節點在紀元 { $now }
    .keyword = 過早

serve-answering-witness = 紀元 { $epoch } 由 { $prover } 應答，對方是來這裡接受提問的
    .keyword = 見證

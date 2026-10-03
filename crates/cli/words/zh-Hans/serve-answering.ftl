### `333 run`: what a peer can ask for at the door, and what this node does about it.

serve-answering-asked = 纪元 { $epoch }，由 { $verifier }
    .keyword = 被问

serve-answering-empty = 有人索取文件。这个节点没有可给的。
    .keyword = 空

serve-answering-gave = 在纪元 { $epoch } 把文件交给 { $receiver }
    .keyword = 已交出

serve-answering-roll = { $members } 位同伴
    .keyword = 名册

serve-answering-cursed = { $name } 问了。333 像在每扇门前那样，从对方的生命里拿走了
    { $milliseconds } 毫秒。
    .keyword = 诅咒

serve-answering-early = 有人来接受关于纪元 { $asked_about } 的提问，而这个节点在纪元 { $now }
    .keyword = 过早

serve-answering-witness = 纪元 { $epoch } 由 { $prover } 应答，对方是来这里接受提问的
    .keyword = 见证

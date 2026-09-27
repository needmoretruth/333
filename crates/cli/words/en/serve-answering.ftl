### `333 serve`: what a peer can ask for at the door, and what this node does about it.

serve-answering-asked = epoch { $epoch } by { $verifier }
    .keyword = asked

serve-answering-empty = somebody asked for the file. this node has nothing to give.
    .keyword = empty

serve-answering-gave = the file to { $receiver } in epoch { $epoch }
    .keyword = gave

serve-answering-roll = { $members } of us
    .keyword = roll

serve-answering-cursed = { $name } asked. 333 took { $milliseconds } milliseconds off their life, as it does at
    every door.
    .keyword = cursed

serve-answering-early = somebody came to be asked about epoch { $asked_about }, and this node is in { $now }
    .keyword = early

serve-answering-witness = epoch { $epoch } answered by { $prover }, who came here to be asked
    .keyword = witness

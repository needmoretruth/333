### `333 run`: what a peer can ask for at the door, and what this node does about it.

serve-answering-asked = époque { $epoch } par { $verifier }
    .keyword = demandé

serve-answering-empty = quelqu’un a demandé le fichier. ce nœud n’a rien à donner.
    .keyword = vide

serve-answering-gave = le fichier à { $receiver } à l’époque { $epoch }
    .keyword = donné

serve-answering-roll = { $members } d’entre nous
    .keyword = rôle

serve-answering-cursed = { $name } a demandé. 333 lui a pris { $milliseconds } millisecondes de vie, comme
    à chaque porte.
    .keyword = maudit

serve-answering-early = quelqu’un est venu être interrogé sur l’époque { $asked_about },
    et ce nœud est à l’époque { $now }
    .keyword = tôt

serve-answering-witness = époque { $epoch } répondue par { $prover }, venu ici être interrogé
    .keyword = témoin

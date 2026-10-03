### `333 run`: what a peer can ask for at the door, and what this node does about it.

serve-answering-asked = época { $epoch }, por { $verifier }
    .keyword = pregunta

serve-answering-empty = alguien pidió el archivo. este nodo no tiene nada que dar.
    .keyword = vacío

serve-answering-gave = el archivo a { $receiver } en la época { $epoch }
    .keyword = dio

serve-answering-roll = { $members } de nosotros
    .keyword = padrón

serve-answering-cursed = preguntó { $name }. 333 le quitó { $milliseconds } milisegundos de vida, como hace
    en cada puerta.
    .keyword = maldito

serve-answering-early = alguien vino a que le preguntaran por la época { $asked_about },
    y este nodo está en la { $now }
    .keyword = pronto

serve-answering-witness = época { $epoch }, respondida por { $prover }, que vino aquí a que le preguntaran
    .keyword = testigo

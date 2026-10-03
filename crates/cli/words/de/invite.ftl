### `333 invite`: the line others use to join through this node.

invite-line = { $invitation }
    .keyword = Einlad.

invite-how = gib diese Zeile der anderen Person. Sie führt auf ihrem Rechner
    `333 join { $invitation }` aus.
    .keyword = wie

invite-none = noch keine. Dieser Knoten hat keine Adresse gefunden, die andere
    erreichen können.
    .keyword = Einlad.

invite-none-next = `333 start` lässt ihn laufen; frag in ein paar Minuten noch einmal.
    Hinter einem Router, den niemand geöffnet hat, lässt
    `333 service install --tor` ihn mit einer Onion-Adresse laufen, die
    keinen Router braucht.
    .keyword = weiter

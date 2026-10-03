### `333 invite`: the line others use to join through this node.

invite-line = { $invitation }
    .keyword = invite

invite-how = donnez cette ligne à l’autre personne. Sur sa machine, elle lance
    `333 join { $invitation }`.
    .keyword = comment

invite-none = pas encore. Ce nœud n’a pas trouvé d’adresse que d’autres peuvent
    atteindre.
    .keyword = invite

invite-none-next = `333 start` le lance ; redemandez dans quelques minutes. Derrière un
    routeur que personne n’a ouvert, `333 service install --tor` le lance
    avec une adresse onion, qui n’a pas besoin de routeur.
    .keyword = ensuite

### `333 invite`: the line others use to join through this node.

invite-line = { $invitation }
    .keyword = invita

invite-how = da esta línea a la otra persona. En su máquina ejecuta
    `333 join { $invitation }`.
    .keyword = cómo

invite-none = aún ninguna. Este nodo no ha encontrado una dirección que otros
    puedan alcanzar.
    .keyword = invita

invite-none-next = `333 start` lo pone en marcha; vuelve a preguntar en unos minutos.
    Detrás de un router que nadie abrió, `333 service install --tor` lo
    ejecuta con una dirección onion, que no necesita router.
    .keyword = después

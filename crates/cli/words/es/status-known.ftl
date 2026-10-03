### `333 status`: where the others are, as far as this node knows.

status-known-another-copy = OTRA COPIA DE ESTE NOMBRE

status-known-sighting = Una declaración firmada con la clave de este nodo, que este nodo
    nunca hizo, dice que está en { $address }, en la época { $said_in }.
    Llegó { $arrived }, en la época { $epoch }.

status-known-either = O se copió este directorio y se puso en marcha la copia, o alguien
    más tiene la clave. Dos nodos con un mismo nombre se contradicen en
    cada época por la que se pregunte a cualquiera de ellos. Detén uno;
    `333 pack` es la forma de mudar un nodo. Nada aquí detiene ninguna
    copia por ti: cualquiera puede reenviar una declaración antigua, y
    un nodo que se detuviera al verla podría apagarlo quien tenga una
    copia de su clave.

status-known-nowhere = aún no hay dónde llamar. Una invitación dada a `333 ping` o a
    `333 join` se guarda, y el nodo llama ahí desde entonces.
    .keyword = SABIDO

status-known-held = { $held ->
        [one] { $held } dirección
       *[other] { $held } direcciones
    }, según dónde se supo de cada una
    .keyword = SABIDO

status-known-by-hand = a mano
status-known-this-network = esta red
status-known-meeting-point = un punto de encuentro
status-known-from-us = de { $peers } de nosotros
status-known-not-noted = sin anotar

status-known-where-heard = Dónde se supo de cada una no dice nada de si alguien responde ahí.
status-known-sources-lists = `333 status --sources` las lista.

status-known-sources = FUENTES
status-known-nobody-answered = aquí aún no ha respondido nadie
status-known-at = en
status-known-first = primera
status-known-last = última
status-known-from-before = guardada desde antes de que este nodo anotara de dónde
    venían las direcciones
status-known-when = { $from }, en la época { $epoch }

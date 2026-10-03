### `333 join`: asking a node that holds the file to hand it over.

join-name = { $name }
    .keyword = nombre

join-knocking = { $address }
    .keyword = llama

join-silence = no se alcanzó a nadie en { $address }. Eso no prueba que 333 haya
    terminado. Este cliente lleva el hash del archivo, no el archivo: no
    hay más entrada que alguien que lo tenga.
    .keyword = silencio

join-knocking-on = llamando a { $address }
join-exchanging = intercambiando latidos
join-asking = pidiendo el archivo
join-no-answer = sin respuesta de { $address } tras { $seconds } s

join-given = por { $giver }
    .keyword = recibido

join-joined = en la época { $epoch }
    .keyword = unido

join-holding = el archivo, y puede pasarlo
    .keyword = tiene

join-roll = { $members } de nosotros
    .keyword = padrón

join-counted = desde la época { $epoch }, y ni una época antes: a dos fronteras,
    entre { $least } y { $most } minutos, según en qué punto de esta época
    llegaste. Hasta entonces, responde a todo lo que se te pregunte. Lo
    que se atestigüe en ese tiempo es toda la prueba de que estuviste aquí.
    .keyword = cuenta

join-vigil = `333 start` lo mantiene en marcha desde ahora. No se puede
    atestiguar nada de un nodo al que nadie alcanza, y este tramo se
    atestigua una vez o nunca.
    .keyword = nodo

join-already-given = este nodo ya tiene el archivo, dado por { $giver } en la época { $epoch }.
    No hay nada que pedir, y no se pidió nada.
join-same-handover = este nodo y { $peer } ya se pasaron el archivo en la época { $epoch }.
    Devolverlo en la misma época es esa misma entrega vista desde el otro
    lado, y no admite a nadie, así que no se pidió nada.

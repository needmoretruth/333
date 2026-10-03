### `333 bootstrap`: beginning a line of your own, when there is nobody to join.

bootstrap-name = { $name }
    .keyword = nombre

bootstrap-vigil = `333 start` lo pone en marcha para que responda.
    .keyword = nodo

bootstrap-already-has-it = este nodo ya tiene el archivo. No hay nada que empezar.

bootstrap-stop = { $already ->
        [one] { $already } de nosotros está diciendo
       *[other] { $already } de nosotros están diciendo
    } en { $meet } dónde se le puede encontrar.
    Empezar ahora por tu cuenta abriría una segunda línea junto a la
    suya sin motivo. Abre { $board } en un navegador, toma una de las
    invitaciones y ejecuta `333 join` con ella.
    Si has leído esto y aun así quieres empezar, `--anyway` lo dice.
    .keyword = alto

bootstrap-not-the-file = lo que llegó no es el archivo

bootstrap-begun = el archivo está en el directorio de este nodo, y este nodo es el
    comienzo de su propia línea. Nadie firmó la entrega, porque nadie la
    hizo, y cualquiera que lea el registro de este nodo puede verlo.

    Esa es la posición del fundador, y no es una cualquiera. Un padrón
    admite a quien recibió el archivo, así que un nodo que no lo recibió
    de nadie no está en ningún padrón: nadie vendrá a preguntarle nada y
    nunca saldrá sorteado para preguntar a nadie. Aun así puede ir a los
    que salgan sorteados para preguntarle y recibir testimonio de ese modo.

    A quien le entregues el archivo después lo admites de la forma
    habitual, firmando los dos, y cuenta desde ese momento.
    .keyword = iniciado

bootstrap-reading-the-board = leyendo el tablón en { $place }

bootstrap-asking = a { $meet } por el archivo
    .keyword = pide

bootstrap-asking-for-the-file = pidiendo el archivo a { $meet }

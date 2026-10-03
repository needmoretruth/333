### `333 run`: trading with the other nodes once an epoch, and asking whoever was drawn.

hours-asking-failed-gathering = reuniendo lo que este nodo podría pasar: { $why }
    .keyword = falló

hours-asking-quiet = { $address }: { $why }
    .keyword = callado

hours-asking-unended = el intercambio no terminó dentro de { $time } de esta época, y el
    resto de las horas no lo esperará
    .keyword = sin fin

hours-asking-reading-address = leyendo la dirección de un par
hours-asking-exchanging-heartbeats = intercambiando latidos
hours-asking-trading = intercambiando declaraciones
hours-asking-putting = planteando la pregunta
hours-asking-sealing-presenting = sellando lo que este nodo vino a decir
hours-asking-saying-what-for = diciendo a qué vino este nodo
hours-asking-sealing-silence = sellando lo que no ocurrió

hours-asking-no-answer = sin respuesta
hours-asking-did-not-answer = { $address } no respondió
hours-asking-did-not-finish = { $address } no terminó el latido
hours-asking-neither = { $address } ni preguntó ni colgó
hours-asking-within = { $what } dentro de los { $seconds } s que permite la ventana
hours-asking-nothing-within = nada dentro de los { $seconds } s que permite la ventana

hours-asking-going = nadie de fuera puede abrir una conexión con este nodo, así que va él
    a los { $drawn } de nosotros sorteados para preguntarle en esta época.
    El sorteo sale de la época y de las claves, así que este nodo sabe
    quiénes son sin que se lo digan.
    .keyword = va

hours-asking-unknown-drawn-by = sorteado para que le pregunte uno de nosotros
    cuyo paradero nadie ha dicho
    .keyword = sin dato

hours-asking-unasked = época { $epoch }: { $why }
    .keyword = sin preg

hours-asking-drawn = época { $epoch }: preguntar a { $asked } de nosotros. Nadie lo eligió:
    los nombres salen de la época y de las claves, igual en cada máquina.
    .keyword = sorteo

hours-asking-unknown-drawn-to-ask = sorteado para preguntar a uno de nosotros
    cuyo paradero nadie ha dicho
    .keyword = sin dato

hours-asking-unheard = época { $epoch }: { $why }
    .keyword = sin oír

hours-asking-witness = época { $epoch }, respondida por { $prover }
    .keyword = testigo

hours-asking-silence = época { $epoch }: { $why }
    .keyword = silencio

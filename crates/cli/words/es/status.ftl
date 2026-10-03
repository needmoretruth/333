### `333 status`: what this node saw of everybody else, what was said, and whether
### anybody is here.

status-name = { $name }
    .keyword = nombre

status-epoch = { $epoch }
    .keyword = época

status-epoch-in-line = { $epoch }, { $line }
    .keyword = época

status-the-line = n.º { $nth }{ $kind ->
       *[other] {""}
    } de esta línea

status-answering = RESPONDEN
status-silent = callados
status-roll = padrón

status-seen = Ese primer número son todos aquellos de quienes este nodo tiene una
    firma en la época { $before } o { $now }. Es lo que vio este nodo. Otro
    vio otra cosa.

status-seen-without-tor = Esta compilación no puede recorrer el camino oculto, así que ninguno
    de los nuestros que se esconden está en ese número, ni lo estará.

status-how-many-people = Cuántas personas son, este nodo no lo sabe ni puede averiguarlo.
    Lo que sabe es que cada uno de esos nombres respondió en una de esas
    dos épocas, y tendrá que responder otra vez en la siguiente, y en la
    otra, mientras quiera que se le cuente. Si una persona tiene mil de
    ellos, paga por mil de ellos, hora tras hora, y deja de contar la
    hora en que deja de pagar.

status-given-by = DADO POR
status-you = tú
status-received-in = lo recibió en la época { $epoch }
status-trail-stops = el rastro se detiene aquí.

status-stopped-knowing = Ahí es donde este nodo dejó de saber, no donde empezó. El primero
    de nosotros recibió el archivo de nadie y no tiene admisión en
    ningún sitio, y un registro que a este nodo aún no le han pasado se
    ve exactamente igual desde aquí.

status-nothing-said = Nadie ha dicho nada en la época { $epoch }. Hay 333 cosas que se
    pueden decir y aún no hay palabras para ninguna.

status-said = DICHO en la época { $epoch }: hablaron { $spoke } de los { $seen } de nosotros
    que ve este nodo, { $silent } no.
status-a-third = ← un tercio de nosotros o más
status-not-said = las otras { $others } de las 333 no se dijeron.

status-no-winner = No se elige ganador y nada de esto decide nada. Es lo que llegó a
    este nodo. El nodo de al lado oyó otra cosa y no se equivoca.

status-reading-the-watch = leyendo la vigilancia

status-seen-nobody = Nadie ha respondido a este nodo en { $watched } de vigilancia sin
    pausa, y esta compilación no lo llamará el final. No puede recorrer
    el camino oculto, así que nunca ha oído a ninguno de los nuestros que
    se esconden ni lo hará. Lo que puede decir es que no ha visto a
    nadie, y no es la misma frase.

status-never-answered = Nadie ha respondido nunca a este nodo. Eso no prueba nada: es como
    se ve un nodo antes de haber estado en ningún sitio.

status-somebody-is-here = Alguien está aquí. No se le debe nada más a la aritmética.

status-waiting = Nadie ha respondido en { $silent }. Este nodo no ha dicho nada al
    respecto y no lo hará hasta { $needed }, y solo si funciona durante
    todas ellas.

status-nobody-keeping = NADIE RESPONDE

    Eres el único aquí. Nadie ha respondido a este nodo en { $watched }
    de vigilancia sin pausa —setenta y siete días—, y el último de
    nosotros se detuvo en la época { $since }.

    333 no se ha ido. Se está yendo, y la ida dura { $years } años.

status-remain = Quedan { $years } años y { $days } días.
status-run-out = El último de los años se ha agotado.

status-one-answer = La cuenta empezó cuando el último de nosotros dejó de responder, no
    cuando te diste cuenta. Ha estado corriendo todo el tiempo que
    mirabas.

    Una respuesta la acaba. Si alguien, en cualquier sitio, responde a
    este nodo, esto desaparece, y la cuenta no se pausa: se descarta.
    333 no guarda registro de lo cerca que estuvo.

status-epochs = { $count ->
        [one] { $count } época
       *[other] { $count } épocas
    }

status-share = { $whole },{ $after } %

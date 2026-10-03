### `333 run`: carrying out what the node was told, from its screen or another terminal.

serve-carrying-not-written-who = anotando quién respondió: { $why }
    .keyword = falló

serve-carrying-unheard = { $why }
    .keyword = sin oír

serve-carrying-unbegun = { $why }
    .keyword = no hecho

serve-carrying-refused = { $why }
    .keyword = negado

serve-carrying-holding-the-file = { $roll } de nosotros en el padrón, y el archivo está aquí
    .keyword = tiene

serve-carrying-holding-no-file = { $roll } de nosotros en el padrón, y a este nodo no le han dado el archivo
    .keyword = tiene

serve-carrying-unread-holding = lo que tiene este nodo: { $why }
    .keyword = ilegible

serve-carrying-already-up = la dirección oculta ya está activa. `tor off` la quita.
    .keyword = activa

serve-carrying-unraised = { $why }
    .keyword = cerrada

serve-carrying-tor-off = la dirección onion deja de responder ahora. Lo que ya se dijo de
    ella sigue en pie hasta que se olvide, dos épocas después de decirse.
    .keyword = oculto

serve-carrying-none-up = no hay ninguna dirección oculta activa que quitar.
    .keyword = activa

serve-carrying-too-late = Tor ya está en marcha, y un puente añadido ahora no cambia nada de
    la conexión que ya hizo. Reinicia el nodo con él.
    .keyword = tarde

serve-carrying-bridged = { $bridges ->
        [one] { $bridges } puente se usará
       *[other] { $bridges } puentes se usarán
    } la próxima vez que arranque Tor.
    .keyword = puente

serve-carrying-helper = { $program } se ejecutará para cualquier puente ofuscado.
    .keyword = puente

serve-carrying-not-an-address = { $typed } no es una dirección: { $why }
    .keyword = ilegible

serve-carrying-stopping = se pidió desde otra terminal.
    .keyword = detiene

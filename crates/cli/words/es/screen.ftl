### The screen: what it says in the log pane about what was typed into it.

screen-asked = { $typed }
    .keyword = pedido
screen-unheard = ya nada cumple las órdenes
    .keyword = sin oír
screen-unread = { $why }
    .keyword = ilegible
screen-refused = { $why }
    .keyword = negado

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = no se pudo leer una tecla: { $why }. Las siguientes quizá sí.
    .keyword = teclado
screen-keyboard-gone = no se pudo leer el teclado ({ $why }), así que la pantalla se cerró y
    el nodo con ella. `333 run --plain` ejecuta el nodo sin teclado.

### `333 run`: orders from other terminals on this machine.

serve-told-not-here = en este sistema llegan a este nodo por su pantalla y por ningún
    otro sitio. Un segundo 333 iniciado a su lado se rechaza.
    .keyword = órdenes

serve-told-cannot = no se pueden recibir desde otras terminales: { $why }
    .keyword = órdenes

serve-told-not-private = no se pudo hacer privado el socket

serve-told-taking = desde cualquier terminal de esta máquina, con las palabras de la
    pantalla: `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`,
    `333 stop`. Este nodo las cumple y responde allí.
    .keyword = órdenes

serve-told-taking-light = desde cualquier terminal de esta máquina:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    Este nodo las cumple y responde allí.
    .keyword = órdenes

serve-told-old-socket = un socket viejo estorba y se queda ahí: { $why }
    .keyword = órdenes

serve-told-not-a-socket = { $path } existe y no es un socket, así que se deja en paz, y no se
    puede pasar nada a este nodo desde otra terminal hasta que se mueva.
    .keyword = órdenes

serve-told-no-longer = ya no se reciben desde otras terminales: { $why }
    .keyword = órdenes

serve-told-not-the-owner = solo el dueño del directorio de este nodo puede decirle algo
    .keyword = negado

serve-told-too-long = eso es más largo que cualquier orden. La más larga tiene { $bytes } bytes.
    .keyword = ilegible

serve-told-other-version = este nodo habla { $ours } y se le preguntó en { $theirs }. El 333 que
    preguntó es una versión distinta de la que funciona; usa esa.
    .keyword = negado

serve-told-unread = { $why }
    .keyword = ilegible

serve-told-asked = { $order }, desde otra terminal
    .keyword = pedido

serve-told-unheard = ya nada cumple las órdenes
    .keyword = sin oír

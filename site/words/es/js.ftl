# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = Copiar
js-copied = Copiado
js-selected = Seleccionado
js-state-awake = El nodo de este sitio está despierto
js-state-not-running = El nodo de este sitio no está en marcha
js-in-hours = en { $h } h { $m } min
js-in-minutes = en { $m } min
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = época n.º { $n } de esta línea

## The network

js-network-state-founder = En ningún padrón
js-network-state-ok = Responde en esta época
js-network-state-quiet = En silencio esta época
js-network-state-later = Cuenta desde una época posterior
js-network-state-seen = Visto, no está en el padrón
js-network-awake = Despierto
js-network-not-running = No está en marcha
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · tuyo
js-network-find-bad = El nombre de un nodo es hexadecimal; escribe al menos sus 6 primeros caracteres.
js-network-find-none = El nodo de este sitio no ha visto ningún nodo con ese nombre.
js-network-find-many = { $count } nodos empiezan así. Escribe más del nombre.
js-network-find-marked = Marcado como tuyo en este dispositivo.
js-network-select = Elige un nodo para ver lo que el nodo de este sitio sabe de él.
js-network-role-founder = Fundador de esta línea
js-network-role-site = El nodo de este sitio
js-network-role-yours = Tuyo, en este dispositivo
js-network-role-none = Un nodo de la red
js-network-col-name = Nombre
js-network-col-state = Esta época
js-network-col-given = Recibió el archivo
js-network-col-counted = Cuenta desde
js-network-col-answered = Última respuesta
js-network-col-said = Dijo
js-network-col-reached = Alcanzado
js-network-row-said = Dijo en esta época
js-network-row-handed = Entregó el archivo a
js-network-row-testimony = Testimonio
js-network-given-by = época { $epoch }, de { $sponsor }
js-network-given-founder = De nadie. Empezó esta línea.
js-network-given-none = No está en el padrón
js-network-epoch = época { $epoch }
js-network-epoch-now = { $epoch } (esta época)
js-network-epoch-ago = { $epoch } (hace { $ago })
js-network-more = y { $count } más en la tabla de abajo
js-network-nothing = Nada
js-network-reach-direct = Directamente
js-network-reach-tor = Por Tor
js-network-reach-tor-short = Tor
js-network-reach-unknown = No se sabe
js-network-testimony = preguntado por { $asked }, preguntó a { $asking } (3 últimas épocas)
js-network-copy-name = Copiar nombre
js-network-select-name = Selecciona el nombre de arriba
js-network-mine = Este es mi nodo
js-network-tag-founder = fundador
js-network-tag-site = este sitio
js-network-tag-yours = tuyo
js-network-empty = El nodo de este sitio aún no ha visto ningún otro nodo.
js-network-this-node = Este nodo
js-network-yes = Sí
js-network-no = No
js-network-none = Ninguno

## Where we are

js-map-watch = Verlo en directo
js-map-stop = Dejar de verlo
js-map-read-at = Leído a las { $read_at } UTC.
js-map-unreadable = No se ha podido leer el tablón ahora mismo.
js-map-tor = Tor
js-map-nowhere = En ningún sitio que el borde pudiera situar
js-map-nobody = Nadie dice dónde está.
js-map-all = Todos los que lo dicen
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = En la red, sin decir dónde
js-map-dot = { $count ->
    [one] { $count } nodo
   *[other] { $count } nodos
  }

## The board

js-board-said = Dicho en la época { $epoch } por { $node }
js-board-site = el nodo de este sitio
js-board-tor = por Tor

## Take the program

js-start-machine-linux-x86_64 = Linux en x86-64
js-start-machine-linux-aarch64 = Linux en ARM de 64 bits
js-start-machine-linux-armv6 = Linux en ARM de 32 bits
js-start-machine-macos-aarch64 = un Mac con Apple silicon
js-start-machine-macos-x86_64 = un Mac con chip Intel
js-start-machine-windows-x86_64 = Windows
js-start-phone = Parece un teléfono o una tableta, y el programa es para un ordenador que se queda encendido. Elige aquí ese ordenador.
js-start-unknown = Este navegador no dice en qué funciona. Elige tu máquina aquí.
js-start-sure = Este navegador dice que funciona en { $machine }, así que eso está elegido aquí.
js-start-mac = Este navegador dice que está en un Mac pero no con qué chip, así que aquí está elegido Apple silicon. El instalador se lo pregunta a la propia máquina.
js-start-linux = Este navegador dice que está en Linux pero no con qué procesador, así que aquí está elegido x86-64. El instalador se lo pregunta a la propia máquina.
js-start-chosen = Elegido arriba

## The story on the home page, drawn

js-story-file = 333.txt · 3 bytes
js-story-gave = Te lo entregué
js-story-received = Lo recibí de ti
js-story-signed = firmado
js-story-minutes = 333 min
js-story-epochs = 333 épocas
js-story-now = ahora
js-story-answering = responden
js-story-roll = en el padrón
js-story-years = { $years } años

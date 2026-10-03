### What the shared parts of the commands say.

commands-clock-at-zero = { $epoch }. El reloj de esta máquina dice que es 1970, así que este
    nodo cree estar al principio de los tiempos. Nadie le entregará nada
    ni dará testimonio de él hasta que se ponga en hora.
    .keyword = época

commands-called-first = se eligió la primera clave que se creó.
    .keyword = elegida

commands-called = { $not_called ->
        [one] se creó { $not_called } clave que no se eligió. esta sí.
       *[other] se crearon { $not_called } claves que no se eligieron. esta sí.
    }
    .keyword = elegida

commands-torn = se descartaron del registro { $bytes } bytes de una entrada sin terminar
    .keyword = rota

commands-record = { $epochs ->
        [one] { $epochs } época ya respondida, ninguna abierta a revisión
       *[other] { $epochs } épocas ya respondidas, ninguna abierta a revisión
    }
    .keyword = registro

commands-witnessed = { $statements ->
        [one] { $statements } declaración que otra clave firmó sobre este nodo. Se guarda
            aunque su época ya haya pasado, porque nada más de ella
            sobrevive a la ventana.
       *[other] { $statements } declaraciones que otras claves firmaron sobre este nodo. Se
            guardan aunque sus épocas ya hayan pasado, porque nada más de
            ellas sobrevive a la ventana.
    }
    .keyword = testigo

commands-unseen = nadie ha firmado nada sobre este nodo, en ninguna época. Salir hacia
    fuera funciona, pero que te alcancen no, y solo cuenta lo segundo:
    quien sale sorteado para preguntar tiene que llegar. Lo causan dos
    cosas: un router que no envía el puerto 3333 a esta máquina, o una
    dirección que nadie recibió. `run --tor` no necesita ninguna de las
    dos: una dirección onion se alcanza detrás de cualquier router, y
    este cliente ya lleva Tor.
    .keyword = oculto

commands-roll-alone = 1 de nosotros, que es este nodo
    .keyword = padrón

commands-roll = { $members } de nosotros
    .keyword = padrón

commands-known = donde { $addresses } de nosotros dijeron que buscáramos
    .keyword = conocido

commands-holding = el archivo, y puede pasarlo
    .keyword = tiene

commands-keeping = todo, para siempre. No le da nada a este nodo: cada declaración
    lleva su propia firma y se verifica igual dondequiera que se guarde.
    No hay archivo oficial ni archivero.
    .keyword = guarda

commands-ignored = { $admissions } admisiones que no se pudieron leer
    .keyword = ignorado

commands-learned-where = dónde están { $addresses } más de nosotros
    .keyword = aprende

commands-rejoined = { $members } más de nosotros por nombre, de un nodo que conocía { $were }.
    Éramos dos cuentas y ahora la cuenta es una.
    .keyword = reunido

commands-learned-names = { $members } más de nosotros por nombre
    .keyword = aprende

commands-heard = { $speakers } de nosotros hablan
    .keyword = oído

commands-carried = { $statements ->
        [one] { $statements } declaración sobre una época aún abierta
       *[other] { $statements } declaraciones sobre épocas aún abiertas
    }
    .keyword = llevado

commands-exchange = { $node }  época { $epoch }  { $clocks }  ({ $liveness })
    .keyword = testigo

commands-answered-the-challenge = respondió al desafío que elegimos
commands-spoke-first = habló primero, lo que solo prueba que habló

commands-clocks-together = relojes a la par
commands-clocks-ahead = su reloj va { $apart } por delante del nuestro
commands-clocks-behind = su reloj va { $apart } por detrás del nuestro
commands-hours-and-minutes = { $hours } h { $minutes } min
commands-minutes-and-seconds = { $minutes } min { $seconds } s
commands-seconds = { $seconds } s

commands-waking = Tor. el camino oculto tarda en abrirse.
    .keyword = arranca

commands-waking-through = Tor, a través de { $bridges ->
        [one] { $bridges } puente
       *[other] { $bridges } puentes
    }. el camino oculto tarda en abrirse.
    .keyword = arranca

commands-no-tor = sin conexión Tor tras { $seconds } s
commands-starting-tor = iniciando el cliente Tor

# What a handover puts a signature under, read back.
commands-signed-giving = tú dijiste: te entregué el archivo en la época { $epoch }.
    ellos dijeron: recibí de ti el archivo en la época { $epoch }.
    está escrito a dos manos, y ninguna mano puede retirarlo.
    .keyword = firmado

commands-signed-taking = ellos dijeron: te entregué el archivo en la época { $epoch }.
    tú dijiste: recibí de ti el archivo en la época { $epoch }.
    está escrito a dos manos, y ninguna mano puede retirarlo.
    .keyword = firmado

commands-brimming = { $statements ->
        [one] { $statements } declaración no cabía en una ronda y espera a la siguiente
       *[other] { $statements } declaraciones no cabían en una ronda y esperan a la siguiente
    }
    .keyword = rebosa

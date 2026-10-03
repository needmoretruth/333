### `333 run`: asking the router, keeping what it lent, and giving it back.

serve-reach-router-opened-upnp = el router dice que el puerto { $port } { $on } llega ahora a esta
    máquina. Aparece allí como `333` si quieres quitarlo. Si llega algo
    lo dice la línea siguiente.
    .keyword = abierto

serve-reach-router-opened-upnp-for = el router dice que el puerto { $port } { $on } llega ahora a esta
    máquina, durante { $time }. Aparece allí como `333` si quieres
    quitarlo. Si llega algo lo dice la línea siguiente.
    .keyword = abierto

serve-reach-router-opened-lease = se pidió al router en { $router } por { $way } el puerto { $port }
    durante { $asked_for }. Dice que el puerto { $granted_port } { $on } llega aquí
    ahora, durante { $granted }. Este nodo lo vuelve a pedir antes de que
    venza y lo devuelve al detenerse; si en cambio se le mata, el router
    lo suelta cuando se acaba el tiempo. Si llega algo lo dice la línea
    siguiente.
    .keyword = abierto

serve-reach-router-nobody-answered = ningún router de aquí respondió a una petición de abrir un puerto,
    por UPnP-IGD, PCP ni NAT-PMP. Es normal: muchos tienen los tres
    apagados, y una máquina con dirección propia no tiene nada que pedir.
    `--no-router` hace que este nodo no lo pida nunca.
    .keyword = cerrado

serve-reach-router-refused = el router no quiso abrir el puerto { $port }: { $why }
    .keyword = cerrado

serve-reach-router-let-go = el router soltó el puerto { $port }: no se le volvió a pedir a tiempo,
    así que nadie de fuera puede alcanzar este nodo por él. Reiniciar el
    nodo lo vuelve a pedir.
    .keyword = cerrado

serve-reach-router-given-back = el puerto { $port } se devolvió al router por { $way }; ya no llega a
    esta máquina.
    .keyword = cerrado

serve-reach-router-not-taken-back = el router no aceptó la devolución del puerto { $port } ({ $why }).
    Lo soltará por sí solo dentro de { $time }.
    .keyword = cerrado

serve-reach-router-moved = el router movió este nodo: ahora llega aquí el puerto { $port } { $on }
    en lugar del { $before_port } { $before_on }. Una invitación con el
    antiguo ya no llega.
    .keyword = abierto

serve-reach-router-not-kept = el router no conservó el puerto { $port } al pedírselo ({ $why }).
    Aún lo tiene durante { $time }, y se le vuelve a pedir antes.
    .keyword = espera

serve-reach-router-on-its-outside-address = en su dirección exterior
serve-reach-router-on = en { $address }

serve-reach-router-one-second = un segundo
serve-reach-router-two-seconds = dos segundos
serve-reach-router-seconds = { $count } segundos
serve-reach-router-one-minute = un minuto
serve-reach-router-two-minutes = dos minutos
serve-reach-router-minutes = { $count } minutos
serve-reach-router-one-hour = una hora
serve-reach-router-two-hours = dos horas
serve-reach-router-hours = { $count } horas

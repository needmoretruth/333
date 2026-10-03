### `333 run`: leaving this node's address at a meeting point, and reading everyone else's.

hours-meeting-unreadable = no se pudo leer { $place }: { $why }
    .keyword = cita

hours-meeting-read-failed-inside = leer { $place } falló dentro de este nodo: { $why }
    .keyword = cita

hours-meeting-left = dejada la dirección de este nodo en { $place }
    .keyword = cita

hours-meeting-stopped = este nodo se detuvo antes de que { $place } respondiera
    .keyword = cita

hours-meeting-leaving-failed-inside = dejar la dirección de este nodo en { $place } falló dentro de este nodo: { $why }
    .keyword = cita

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place } acepta una declaración por minuto de cada dirección de
    internet, y recibió una de esta hace menos de un minuto.{ $holding }
    Este nodo vuelve a dejar su dirección { $when }.
    .keyword = cita

hours-meeting-full = { $place } ya aceptó todas las declaraciones de un día y acepta más
    tras la medianoche UTC. Aún se puede leer.{ $holding }
    Este nodo vuelve a dejar su dirección { $next_epoch }.
    .keyword = cita

hours-meeting-full-until = { $place } ya aceptó todas las declaraciones de un día y acepta más
    tras la medianoche UTC, dentro de { $midnight }. Aún se puede leer.{ $holding }
    Este nodo vuelve a dejar su dirección { $next_epoch }.
    .keyword = cita

hours-meeting-holds-from = Aún guarda la dirección de este nodo de la época { $epoch }.
hours-meeting-holds-nothing = No guarda nada de este nodo.

hours-meeting-at-the-next-epoch = en la próxima época, dentro de { $wait }
hours-meeting-in = dentro de { $wait }

hours-meeting-did-not-reach = la dirección de este nodo no llegó a { $place }: { $why }
    .keyword = cita

hours-meeting-not-taken = { $place } no aceptó la dirección de este nodo: { $why }
    .keyword = cita

hours-meeting-seconds = { $seconds ->
        [one] { $seconds } segundo
       *[other] { $seconds } segundos
    }

hours-meeting-nobody = nadie dice dónde está en { $place }
    .keyword = cita

hours-meeting-newer = { $fresh ->
        [one] { $fresh } dirección más reciente
       *[other] { $fresh } direcciones más recientes
    } en { $place }
    .keyword = cita

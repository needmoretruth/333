### `333 run`: keeping the hours, at every epoch boundary.

hours-failed-marking = registrando esta época: { $why }
    .keyword = falló

hours-failed-sources = anotando de dónde vinieron las direcciones: { $why }
    .keyword = falló

hours-not-leaving = no se deja esta dirección en { $place }. Llega a este nodo desde
    aquí y desde ningún otro sitio, y un desconocido que la marcara
    llegaría a algo suyo.
    .keyword = cita

hours-sealing = sellando la dirección de este nodo

hours-failed-keeping-address = guardando la dirección propia de este nodo: { $why }
    .keyword = falló

hours-failed-saying-where = diciendo dónde está este nodo: { $why }
    .keyword = falló

hours-forgot = { $epochs ->
        [one] { $epochs } época
       *[other] { $epochs } épocas
    }. nada que se diga ahora sobre ellas podría cambiar un veredicto.
    .keyword = olvida

hours-failed-forgetting = olvidando declaraciones antiguas: { $why }
    .keyword = falló

hours-minutes = { $minutes ->
        [one] { $minutes } minuto
       *[other] { $minutes } minutos
    }

hours-minutes-and-seconds = { $minutes } min { $seconds } s
hours-hours-and-minutes = { $hours } h { $minutes } min

hours-epochs-answered-for = { $epochs ->
        [one] { $epochs } época respondida
       *[other] { $epochs } épocas respondidas
    }

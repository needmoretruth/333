### `333 run`.

serve-nothing-listening = nada estaría escuchando: --no-direct necesita --tor

serve-name = { $name }
    .keyword = nombre

serve-waiting-for-the-file = a este nodo no le han dado el archivo, así que aún no cuenta nada
    para él y aún no hay nada que nadie pueda atestiguar. No puede
    crearlo. Solo llega de alguien que ya lo tiene, y los dos firmáis la
    entrega. Pide una invitación, y luego
    `333 join 333:su.dirección:3333`. Responder mientras tanto no cuesta
    nada y es como la gente te encuentra.
    .keyword = espera

serve-hand = una invitación nombra un lugar, no una persona. Quien responda ahí
    demuestra quién es teniendo su clave.
    .keyword = confía

serve-invite = { $invitation }
    .keyword = invita

serve-answer = { $bound }
    .keyword = responde

serve-nearby = anunciando en esta red que algo aquí habla 333, y escuchando a los
    demás. No el nombre de este nodo: lo que sale es lo que encontraría
    un escaneo de puertos de la misma red. --no-mdns lo mantiene fuera.
    .keyword = cerca

serve-nearby-failed = no se pudo empezar a anunciar en esta red que este nodo está aquí: { $why }
    .keyword = cerca

serve-meet = { $place } es donde este nodo busca a gente que nadie le presentó.
    Todo lo que se lee ahí lo firma quien lo dijo, y no se cree nada de
    lo que hay ahí. --no-meet lo mantiene lejos.
    .keyword = cita

serve-listener-stopped = un oyente se detuvo de forma inesperada

serve-farewell = terminó en la época { $epoch }. Quien salga sorteado para preguntar por
    ti mientras esto no funciona firma que preguntó y no oyó nada, y eso
    es lo que lee tu ventana. Mide { $window } épocas, y se mueve.
    .keyword = nodo

serve-farewell-on-no-roll = terminó en la época { $epoch }. No estás en el padrón de nadie, así
    que nadie sale a preguntar por ti, y no se firma nada sobre ti
    mientras esto no funciona.
    .keyword = nodo

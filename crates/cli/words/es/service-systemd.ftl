### `333 service` on Linux: systemd.

service-systemd-no-configuration = este sistema no indica ningún directorio de configuración
    para este usuario

service-systemd-no-session = systemd no mantiene una sesión para este usuario aquí ({ $why }).
    La inicia cuando este usuario entra, en la consola o por ssh, y no
    con su ni sudo. Entra como este usuario y vuelve a ejecutar esto.

service-systemd-wrote-over = { $path }, en lugar del que había
    .keyword = escrito

service-systemd-linger-already = ya activo para { $user }. Mantiene el nodo en marcha tras cerrar
    sesión, y lo inicia al arrancar sin nadie conectado.
    .keyword = linger

service-systemd-linger-on = activado para { $user }. Mantiene el nodo en marcha tras cerrar
    sesión, y lo inicia al arrancar sin nadie conectado.
    .keyword = linger

service-systemd-linger-not-on = no activo: { $why }. Sin él, el nodo se detiene al cerrar sesión y
    espera a que vuelvas a entrar tras un reinicio.
    `sudo loginctl enable-linger { $user }` lo activa.
    .keyword = linger

service-systemd-linger-off = desactivado otra vez, como antes de instalar.
    .keyword = linger

service-systemd-linger-left = se dejó como estaba. La instalación no lo activó.
    .keyword = linger

service-systemd-not-answering = desconocido: systemd no responde por este usuario
service-systemd-restarting = detenido, y vuelve a iniciarse 333 segundos después de detenerse
service-systemd-stopping = deteniéndose
service-systemd-failed = falló ({ $result })
service-systemd-stopped = detenido
service-systemd-at-every-boot = { $state }, e iniciado en cada arranque
service-systemd-not-again = { $state }, y { $file_state }: no vuelve a iniciarse solo

### `333 service`: running the node through logouts and reboots, when asked to.

service-mind = { $node } está en un sitio que este sistema vacía, y el nombre de este
    nodo no se guarda en ningún otro. El servicio lo ejecuta ahí hasta que
    se vacíe.
    .keyword = ojo

service-runs = { $command }
    .keyword = nodo

service-undo-partial = `333 service uninstall` quita lo que se haya hecho de esto.
    .keyword = deshacer

service-no-receipt-directory = este sistema no indica ningún directorio de configuración donde
    guardar el recibo

service-wrote-receipt = { $path }, que es como `333 service uninstall` sabe qué deshacer.
    .keyword = escrito

service-undo = `333 service uninstall` detiene el nodo y deshace todo lo anterior.
    El directorio propio del nodo no se toca en ningún caso.
    .keyword = deshacer

service-uninstalled = ya no lo ejecuta un servicio. { $node } queda como lo dejó el nodo:
    `333 run` lo ejecuta a mano, y `333 start` vuelve a montar el servicio.
    .keyword = nodo

service-none-installed = `333 service install` no instaló ninguno para este usuario.
    .keyword = servicio

service-state = { $state }
    .keyword = servicio

service-node = { $node }
    .keyword = nodo

service-last-awake = lo dijo por última vez el { $at }, hace { $ago }
    .keyword = activo

service-never-awake = nunca lo dijo, en este directorio
    .keyword = activo

service-said-nothing = nada que se haya guardado
    .keyword = dijo

service-said-last = { $lines ->
        [one] la última línea:
       *[other] las últimas { $lines } líneas:
    }
    .keyword = dijo

service-no-manager = este sistema no tiene un gestor de servicios al que `333 service`
    sepa preguntar. `333 run --plain` ejecuta el nodo bajo lo que sea que
    mantenga los programas en marcha aquí.

service-not-installed-here = no instalado: aquí no hay ningún gestor de servicios que esto conozca

# Said by every service manager's own file.

service-creating = creando { $path }
service-writing = escribiendo { $path }
service-removing = quitando { $path }

service-wrote = { $path }
    .keyword = escrito

service-removed = { $path }
    .keyword = quitado

service-left = { $path }. No lo escribió `333 service install`.
    .keyword = dejado

service-failed = { $why }
    .keyword = falló

service-not-installed = no instalado
service-running = en marcha
service-starting = iniciándose

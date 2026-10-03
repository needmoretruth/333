### `333 service`: the line a running node writes to say it is still awake.

service-awake-failed = escribiendo que el nodo está despierto, en { $root }: { $why }. Nada en
    esta máquina puede saber que funciona hasta que esto vuelva a ir.
    .keyword = falló

service-awake-never-kept = no funciona. El servicio está instalado y el nodo nunca dijo que
    estuviera despierto. `333 service status` dice por qué.
    .keyword = nodo

service-awake-not-kept-since = no funciona desde { $at }, hace { $ago }. `333 service status` dice
    por qué.
    .keyword = nodo

service-awake-under-a-minute = menos de un minuto

service-awake-minutes = { $minutes ->
        [one] { $minutes } minuto
       *[other] { $minutes } minutos
    }

service-awake-epochs = { $epochs ->
        [one] { $epochs } época
       *[other] { $epochs } épocas
    }

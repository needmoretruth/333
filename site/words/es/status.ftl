# Status: the network now, how it has been one epoch at a time, and this site's machine.

status-meta-title = Estado · 333
status-meta-description = Cómo está la red de 333 ahora y cómo ha estado, época a época: nodos en el padrón, nodos que responden, declaraciones en el tablón, y si el nodo y la máquina del propio sitio siguieron en marcha.
status-heading = Estado
status-lede = El nodo de este sitio mira la red cada 15 segundos y anota las cifras una vez por época.
status-now-title = Ahora
status-roll = En el padrón, fundador incluido
status-saying = Dicen dónde están
status-tor = De ellos, por Tor
status-site-node = El nodo de este sitio
status-time-title = A lo largo del tiempo
status-time-lede = Una muestra por época: las últimas cifras que dio el nodo de este sitio antes de que terminara la época. Un hueco en una línea es una época que nadie anotó.
status-chart-recent-title = Las últimas 333 épocas
status-chart-all-title = Todo lo anotado
# Under each chart, the same numbers as text. Every $variable is a number or a date
# (YYYY-MM-DD, UTC), or a dash when there is none.
status-chart-summary = Épocas { $first } a { $last }, del { $from } al { $to }. En el padrón: mínimo { $roll_low }, máximo { $roll_high }, último { $roll_latest }. Responden: mínimo { $answering_low }, máximo { $answering_high }, último { $answering_latest }.
status-chart-too-few = Hay menos de dos épocas anotadas en este tramo, así que aún no hay línea que dibujar.
status-machine-title = La máquina de este sitio
status-release = Versión
status-deployed = Desplegada
status-observed = Última mirada al nodo
status-age = { $seconds ->
    [one] hace 1 segundo.
   *[other] hace { $seconds } segundos.
}
status-observed-running = Estaba en marcha.
status-observed-not-running = No estaba en marcha.
status-uptime = Máquina encendida desde hace
status-uptime-value = { $days ->
    [one] 1 día
   *[other] { $days } días
}, { $hours ->
    [one] 1 hora
   *[other] { $hours } horas
}
status-elsewhere = Todos los nodos están en <a href="{ $base }/network">la página de la red</a>, y dónde están, en <a href="{ $base }/map">el mapa</a>.
status-json = Las mismas cifras para un programa: <a href="/api/status">/api/status</a>.

### `333 run`: the socket listener.

serve-socket-port-taken = Io sur ĉi tiu maŝino jam aŭskultas ĉe la retpordo { $port }: alia
    nodo, aŭ alia programo. Haltigu ĝin, aŭ donu --bind kun alia retpordo.

serve-socket-not-here = { $ip } ne estas adreso de ĉi tiu maŝino. --bind 0.0.0.0:{ $port }
    aŭskultas ĉe ĉiuj.

serve-socket-privileged = La retpordo { $port } estas sub 1024, kie nur la administranto de ĉi
    tiu maŝino rajtas aŭskulti. Donu --bind kun retpordo super ĝi:
    --bind { $suggested }.

serve-socket-listening-on = aŭskultante ĉe { $bind }

serve-socket-accepting = akceptante samulon

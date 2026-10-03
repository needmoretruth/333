### `333 run`: the socket listener.

serve-socket-port-taken = Etwas auf diesem Rechner lauscht schon auf Port { $port }: ein anderer
    Knoten oder ein anderes Programm. Stoppe es, oder gib --bind mit
    einem anderen Port an.

serve-socket-not-here = { $ip } ist keine Adresse dieses Rechners. --bind 0.0.0.0:{ $port }
    lauscht auf allen.

serve-socket-privileged = Port { $port } liegt unter 1024, wo nur der Administrator dieses
    Rechners lauschen darf. Gib --bind mit einem höheren Port an:
    --bind { $suggested }.

serve-socket-listening-on = lausche auf { $bind }

serve-socket-accepting = nehme ein Gegenüber an

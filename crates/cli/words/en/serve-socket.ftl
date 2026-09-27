### `333 serve`: the socket listener.

serve-socket-port-taken = Something on this machine already listens on port { $port }: another
    node, or another program. Stop it, or pass --bind with another port.

serve-socket-not-here = { $ip } is not an address of this machine. --bind 0.0.0.0:{ $port }
    listens on all of them.

serve-socket-listening-on = listening on { $bind }

serve-socket-accepting = accepting a peer

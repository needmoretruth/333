### `333 run`: the socket listener.

serve-socket-port-taken = Algo en esta máquina ya escucha en el puerto { $port }: otro nodo u
    otro programa. Detenlo, o usa --bind con otro puerto.

serve-socket-not-here = { $ip } no es una dirección de esta máquina. --bind 0.0.0.0:{ $port }
    escucha en todas.

serve-socket-privileged = El puerto { $port } está por debajo de 1024, donde solo puede escuchar
    el administrador de esta máquina. Usa --bind con un puerto mayor:
    --bind { $suggested }.

serve-socket-listening-on = escuchando en { $bind }

serve-socket-accepting = aceptando un par

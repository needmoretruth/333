### `333 run`: the socket listener.

serve-socket-port-taken = Quelque chose sur cette machine écoute déjà sur le port { $port } : un
    autre nœud, ou un autre programme. Arrêtez-le, ou passez --bind avec
    un autre port.

serve-socket-not-here = { $ip } n’est pas une adresse de cette machine. --bind 0.0.0.0:{ $port }
    écoute sur toutes.

serve-socket-privileged = Le port { $port } est sous 1024, où seul l’administrateur de cette
    machine peut écouter. Passez --bind avec un port au-dessus :
    --bind { $suggested }.

serve-socket-listening-on = écoute sur { $bind }

serve-socket-accepting = accueil d’un pair

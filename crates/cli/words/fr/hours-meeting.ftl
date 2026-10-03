### `333 run`: leaving this node's address at a meeting point, and reading everyone else's.

hours-meeting-unreadable = { $place } est illisible : { $why }
    .keyword = agora

hours-meeting-read-failed-inside = la lecture de { $place } a échoué dans ce nœud : { $why }
    .keyword = agora

hours-meeting-left = adresse de ce nœud laissée sur { $place }
    .keyword = agora

hours-meeting-stopped = ce nœud s’est arrêté avant que { $place } réponde
    .keyword = agora

hours-meeting-leaving-failed-inside = laisser l’adresse de ce nœud sur { $place } a échoué dans ce nœud : { $why }
    .keyword = agora

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place } prend une déclaration par minute de chaque adresse
    internet, et en a reçu une de celle-ci il y a moins d’une minute.{ $holding }
    Ce nœud relaisse son adresse { $when }.
    .keyword = agora

hours-meeting-full = { $place } a pris toutes les déclarations qu’il prend en un jour et en
    reprend après minuit UTC. On peut encore le lire.{ $holding }
    Ce nœud relaisse son adresse { $next_epoch }.
    .keyword = agora

hours-meeting-full-until = { $place } a pris toutes les déclarations qu’il prend en un jour et en
    reprend après minuit UTC, dans { $midnight }. On peut encore le lire.{ $holding }
    Ce nœud relaisse son adresse { $next_epoch }.
    .keyword = agora

hours-meeting-holds-from = Il garde encore l’adresse de ce nœud de l’époque { $epoch }.
hours-meeting-holds-nothing = Il ne garde rien de ce nœud.

hours-meeting-at-the-next-epoch = à la prochaine époque, dans { $wait }
hours-meeting-in = dans { $wait }

hours-meeting-did-not-reach = l’adresse de ce nœud n’a pas atteint { $place } : { $why }
    .keyword = agora

hours-meeting-not-taken = { $place } n’a pas pris l’adresse de ce nœud : { $why }
    .keyword = agora

hours-meeting-seconds = { $seconds ->
        [one] { $seconds } seconde
       *[other] { $seconds } secondes
    }

hours-meeting-nobody = personne ne dit où il est sur { $place }
    .keyword = agora

hours-meeting-newer = { $fresh ->
        [one] { $fresh } adresse plus récente
       *[other] { $fresh } adresses plus récentes
    } sur { $place }
    .keyword = agora

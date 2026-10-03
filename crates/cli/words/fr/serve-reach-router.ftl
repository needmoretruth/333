### `333 run`: asking the router, keeping what it lent, and giving it back.

serve-reach-router-opened-upnp = le routeur dit que le port { $port } { $on } arrive maintenant sur
    cette machine. Il y figure sous `333` si vous voulez le retirer. La
    ligne suivante dit si quelque chose arrive.
    .keyword = ouvert

serve-reach-router-opened-upnp-for = le routeur dit que le port { $port } { $on } arrive maintenant sur
    cette machine, pendant { $time }. Il y figure sous `333` si vous voulez
    le retirer. La ligne suivante dit si quelque chose arrive.
    .keyword = ouvert

serve-reach-router-opened-lease = demandé au routeur { $router } par { $way } le port { $port }
    pendant { $asked_for }. Il dit que le port { $granted_port } { $on } arrive
    ici, pendant { $granted }. Ce nœud redemande avant l’échéance et le
    rend à l’arrêt ; si le nœud est tué, le routeur le lâche à la fin du
    délai. La ligne suivante dit si quelque chose arrive.
    .keyword = ouvert

serve-reach-router-nobody-answered = aucun routeur ici n’a répondu à une demande d’ouverture de port,
    par UPnP-IGD, PCP ou NAT-PMP. C’est courant : beaucoup les ont tous
    trois désactivés, et une machine avec sa propre adresse n’a rien à
    demander. `--no-router` empêche ce nœud de demander.
    .keyword = fermé

serve-reach-router-refused = le routeur n’a pas voulu ouvrir le port { $port } : { $why }
    .keyword = fermé

serve-reach-router-let-go = le routeur a lâché le port { $port } : il n’a pas été redemandé à
    temps, donc personne dehors ne peut atteindre ce nœud par lui.
    Relancer le nœud le redemande.
    .keyword = fermé

serve-reach-router-given-back = le port { $port } est rendu au routeur par { $way } ; il n’arrive
    plus sur cette machine.
    .keyword = fermé

serve-reach-router-not-taken-back = le routeur n’a pas repris le port { $port } ({ $why }). Il le
    lâchera de lui-même d’ici { $time }.
    .keyword = fermé

serve-reach-router-moved = le routeur a déplacé ce nœud : le port { $port } { $on } arrive ici
    au lieu du port { $before_port } { $before_on }. Une invitation qui
    nomme l’ancien n’arrive plus.
    .keyword = ouvert

serve-reach-router-not-kept = le routeur n’a pas gardé le port { $port } quand on le lui a
    demandé ({ $why }). Il l’a encore pendant { $time }, et on le lui
    redemande avant.
    .keyword = attente

serve-reach-router-on-its-outside-address = sur son adresse extérieure
serve-reach-router-on = sur { $address }

serve-reach-router-one-second = une seconde
serve-reach-router-two-seconds = deux secondes
serve-reach-router-seconds = { $count } secondes
serve-reach-router-one-minute = une minute
serve-reach-router-two-minutes = deux minutes
serve-reach-router-minutes = { $count } minutes
serve-reach-router-one-hour = une heure
serve-reach-router-two-hours = deux heures
serve-reach-router-hours = { $count } heures

### `333 run`: carrying out what the node was told, from its screen or another terminal.

serve-carrying-not-written-who = note de qui a répondu : { $why }
    .keyword = échec

serve-carrying-unheard = { $why }
    .keyword = inouï

serve-carrying-unbegun = { $why }
    .keyword = pas fait

serve-carrying-refused = { $why }
    .keyword = refusé

serve-carrying-holding-the-file = { $roll } d’entre nous sur le rôle, et le fichier est ici
    .keyword = détient

serve-carrying-holding-no-file = { $roll } d’entre nous sur le rôle, et ce nœud n’a pas reçu le fichier
    .keyword = détient

serve-carrying-unread-holding = ce que détient ce nœud : { $why }
    .keyword = non lu

serve-carrying-already-up = l’adresse cachée est déjà ouverte. `tor off` la ferme.
    .keyword = ouverte

serve-carrying-unraised = { $why }
    .keyword = fermée

serve-carrying-tor-off = l’adresse onion cesse de répondre maintenant. Ce qui a déjà été dit
    d’elle tient jusqu’à son oubli, deux époques après avoir été dit.
    .keyword = caché

serve-carrying-none-up = il n’y a pas d’adresse cachée ouverte à fermer.
    .keyword = ouverte

serve-carrying-too-late = Tor tourne déjà, et un pont ajouté maintenant ne change rien à la
    connexion déjà faite. Relancez plutôt le nœud avec lui.
    .keyword = tard

serve-carrying-bridged = { $bridges ->
        [one] { $bridges } pont servira
       *[other] { $bridges } ponts serviront
    } au prochain démarrage de Tor.
    .keyword = pont

serve-carrying-helper = { $program } sera lancé pour tout pont obscurci.
    .keyword = pont

serve-carrying-not-an-address = { $typed } n’est pas une adresse : { $why }
    .keyword = non lu

serve-carrying-stopping = demandé depuis un autre terminal.
    .keyword = arrêt

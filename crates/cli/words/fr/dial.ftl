### Reaching another node, whichever way its address says to.

dial-would-show = ce nœud garde son adresse cachée, il n’ouvrira donc pas de
    connexion vers { $address }, qui la montrerait

dial-no-answer = pas de réponse après { $seconds } s

dial-waking = quelqu’un qui vaut d’être atteint est à une adresse cachée et Tor
    n’est pas actif. Le premier démarrage prend de quelques secondes à
    quelques minutes, et rien n’est demandé à personne avant la fin.
    .keyword = réveil

dial-unwoken = Tor n’a pas démarré : { $why }
    Les adresses cachées sont sautées cette époque. Les nœuds derrière
    elles n’ont pas manqué de répondre : rien ne les a atteints pour
    demander.
    .keyword = endormi

dial-connecting = connexion à { $address }
dial-without-tor = ce client a été compilé sans Tor, il ne peut donc pas atteindre { $address }

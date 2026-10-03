### `333 run`: whether anybody outside can actually get in.

serve-reach-unanswered-at-the-end = le routeur n’avait pas répondu à l’arrêt du nœud. Ce qu’il aurait
    accepté expire de lui-même d’ici { $time }.
    .keyword = fermé

serve-reach-shut-behind-another = le routeur dit que ce foyer est à { $seen }, qui n’est pas une
    adresse de l’internet ouvert : un autre routeur, ou l’adresse
    partagée du fournisseur, se tient entre lui et tous les autres, et
    rien ici ne peut le lui demander. `333 run --tor` ne demande aucun
    changement de routeur.
    .keyword = fermé

serve-reach-open = le port { $port } atteint cette machine depuis l’extérieur. Ce nœud a
    frappé à { $outside } et s’est répondu à lui-même : cette adresse
    peut être donnée à n’importe qui.
    .keyword = ouvert

serve-reach-invite = { $invitation }
    .keyword = invite

serve-reach-shut-somebody-else = quelque chose a répondu à { $outside } et ce n’était pas ce nœud. Ce
    port de votre adresse appartient à autre chose : une invitation qui
    le nomme enverrait les gens vers la mauvaise machine.
    .keyword = fermé

serve-reach-shut-unfinished = quelque chose à { $outside } a pris la connexion sans finir un
    battement : { $why }. Une invitation qui la nomme n’est pas à donner.
    .keyword = fermé

serve-reach-shut-nothing = rien n’a répondu à { $outside } : pour le monde extérieur, ce nœud
    n’écoute pas. Soit le routeur devant lui n’a jamais reçu l’ordre
    d’envoyer le port { $port } ici, soit il ne laisse pas une machine
    intérieure composer sa propre adresse extérieure. `333 run --tor` ne
    demande aucun changement de routeur et marche sur tout réseau.
    .keyword = fermé

### `333 bootstrap`: beginning a line of your own, when there is nobody to join.

bootstrap-name = { $name }
    .keyword = nom

bootstrap-vigil = `333 start` le lance, pour qu’il réponde.
    .keyword = nœud

bootstrap-already-has-it = ce nœud a déjà le fichier. Il n’y a rien à commencer.

bootstrap-stop = { $already ->
        [one] { $already } d’entre nous dit
       *[other] { $already } d’entre nous disent
    } sur { $meet } où les trouver. Commencer seul
    maintenant ouvrirait une seconde lignée à côté de la leur sans
    raison. Ouvrez { $board } dans un navigateur, prenez une des
    invitations et lancez plutôt `333 join` avec elle.
    Si vous avez lu ceci et voulez quand même commencer, `--anyway` le dit.
    .keyword = halte

bootstrap-not-the-file = ce qui est revenu n’est pas le fichier

bootstrap-begun = le fichier est dans le répertoire de ce nœud, et ce nœud est le début
    de sa propre lignée. Personne n’a signé la remise, puisque personne ne
    l’a faite, et quiconque lit le registre de ce nœud peut le voir.

    C’est la place du fondateur, et elle n’est pas ordinaire. Un rôle
    admet quiconque a reçu le fichier ; un nœud qui ne l’a reçu de
    personne n’est donc sur aucun rôle : personne ne viendra lui demander
    quoi que ce soit, et il n’est jamais tiré au sort pour interroger qui
    que ce soit. Il peut encore aller vers ceux tirés au sort pour
    l’interroger, et recevoir ainsi leur témoignage.

    Celui à qui vous remettez ensuite le fichier est admis de la façon
    ordinaire, vous signez tous les deux, et il compte dès cet instant.
    .keyword = commencé

bootstrap-reading-the-board = lecture du tableau sur { $place }

bootstrap-asking = { $meet } pour le fichier
    .keyword = demande

bootstrap-asking-for-the-file = demande du fichier à { $meet }

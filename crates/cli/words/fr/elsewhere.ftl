### What a command says when another 333 already has this node's directory.

elsewhere-the-vigil = le nœud qui tourne dans ce répertoire
elsewhere-the-vigil-by-number = le nœud qui tourne dans ce répertoire (processus { $pid })
elsewhere-another = un autre 333
elsewhere-another-by-number = un autre 333 (processus { $pid })

elsewhere-done = fait par { $vigil }.
    .keyword = fait

elsewhere-failed = { $vigil } ne l’a pas fait.
    .keyword = échec

elsewhere-finding-its-name = { $who } cherche encore le nom
    de ce nœud. Relancez ceci quand il en aura un.
    .keyword = occupé

elsewhere-already-keeping = { $who } tourne déjà ici, et un répertoire est un nœud.
    On peut lui dire des choses d’ici : `333 say 7`,
    `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`. Un second
    nœud a besoin de son propre répertoire, donné avec --data-dir.
    .keyword = occupé

elsewhere-keeping = { $who } tourne ici, et
    { $why }
    .keyword = occupé

elsewhere-nobody-to-tell = aucun nœud ne tourne dans ce répertoire, il n’y a donc personne
    à qui le dire. `333 start` ou `333 run` le lance, et alors ceci marche.
    .keyword = personne

elsewhere-busy = { $who } tient le répertoire de ce
    nœud et n’est pas un nœud en marche à qui l’on peut confier ceci. Rien
    n’a été lu ni écrit ici. Relancez ceci quand il aura fini.
    .keyword = occupé

elsewhere-busy-cannot-be-handed = { $who } tient le répertoire de ce
    nœud. Sur ce système, on ne peut encore rien confier à un 333 en
    marche depuis un autre terminal, donc rien n’a été lu ni écrit ici.
    Tapez-le dans son écran après `:`, ou arrêtez-le et relancez ceci.
    .keyword = occupé

### `333 service`: running the node through logouts and reboots, when asked to.

service-mind = { $node } est dans un endroit que ce système vide, et le nom de ce
    nœud n’est gardé nulle part ailleurs. Le service l’y fait tourner
    jusqu’à ce qu’il soit vidé.
    .keyword = gare

service-runs = { $command }
    .keyword = nœud

service-undo-partial = `333 service uninstall` retire ce qui en a été fait.
    .keyword = annuler

service-no-receipt-directory = ce système n’indique aucun répertoire de configuration où garder
    le reçu

service-wrote-receipt = { $path }, grâce auquel `333 service uninstall` sait quoi annuler.
    .keyword = écrit

service-undo = `333 service uninstall` arrête le nœud et annule tout ce qui précède.
    Le répertoire propre du nœud n’est touché par aucun des deux.
    .keyword = annuler

service-uninstalled = n’est plus lancé par un service. { $node } reste tel que le nœud l’a
    laissé : `333 run` le lance à la main, et `333 start` remet le
    service en place.
    .keyword = nœud

service-none-installed = aucun n’a été installé par `333 service install` pour cet utilisateur.
    .keyword = service

service-state = { $state }
    .keyword = service

service-node = { $node }
    .keyword = nœud

service-last-awake = l’a dit pour la dernière fois à { $at }, il y a { $ago }
    .keyword = éveillé

service-never-awake = ne l’a jamais dit, dans ce répertoire
    .keyword = éveillé

service-said-nothing = rien qui ait été gardé
    .keyword = dit

service-said-last = { $lines ->
        [one] la dernière ligne :
       *[other] les { $lines } dernières lignes :
    }
    .keyword = dit

service-no-manager = ce système n’a pas de gestionnaire de services que `333 service`
    sache interroger. `333 run --plain` lance le nœud sous ce qui garde
    les programmes en marche ici.

service-not-installed-here = non installé : il n’y a ici aucun gestionnaire de services connu

# Said by every service manager's own file.

service-creating = création de { $path }
service-writing = écriture de { $path }
service-removing = retrait de { $path }

service-wrote = { $path }
    .keyword = écrit

service-removed = { $path }
    .keyword = retiré

service-left = { $path }. `333 service install` ne l’a pas écrit.
    .keyword = laissé

service-failed = { $why }
    .keyword = échec

service-not-installed = non installé
service-running = en marche
service-starting = démarrage

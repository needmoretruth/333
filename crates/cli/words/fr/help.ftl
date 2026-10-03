### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph.

help-about = Un nœud de 333. Il répond quand on l’interroge, garde son registre
    et transmet le fichier.

help-id = Affiche le nom de ce nœud, et en crée un au premier lancement

help-bootstrap = Commence une lignée quand personne ne peut vous remettre le fichier
help-bootstrap-long = Commence une lignée quand personne ne peut vous remettre le fichier.

    L’entrée ordinaire est `333 join` avec une invitation. Ceci regarde
    d’abord le point de rencontre et refuse s’il y a quelqu’un. S’il n’y
    a personne, il récupère le fichier, le vérifie avec l’empreinte que
    porte ce client et l’inscrit. Votre nœud est alors le fondateur de sa
    propre lignée, sans la signature de personne à son commencement, et
    quiconque lit son registre peut le voir.

help-serve = Fait tourner ce nœud dans ce terminal jusqu’à ce que vous l’arrêtiez
help-serve-long = Fait tourner ce nœud dans ce terminal jusqu’à ce que vous l’arrêtiez.

    Il répond aux battements et aux questions, échange ce qu’il sait et
    interroge à chaque époque les nœuds qu’il est tiré au sort
    d’interroger. Dans un terminal il ouvre l’écran ; `q`, Ctrl-C ou
    `333 stop` depuis un autre terminal l’arrête. `333 start` fait la même
    chose en arrière-plan.

help-serve-long-light = Fait tourner ce nœud dans ce terminal jusqu’à ce que vous l’arrêtiez.

    Il répond aux battements et aux questions, échange ce qu’il sait et
    interroge à chaque époque les nœuds qu’il est tiré au sort
    d’interroger, ligne par ligne. Ctrl-C ou `333 stop` depuis un autre
    terminal l’arrête. `333 start` fait la même chose en arrière-plan.

help-say = Dit l’une des 333, une fois par époque. Ce qui voyage, c’est le numéro

help-status = Montre si ce nœud tourne, où les autres peuvent l’atteindre, et combien
    d’entre nous répondent

help-join = Reçoit le fichier d’un nœud qui l’a, avec une invitation

help-languages = Liste les langues, ou en garde une pour toutes les commandes de ce nœud

help-ping = Atteint un autre nœud et échange un battement avec lui

help-pack = Écrit ce nœud dans un fichier, à emporter sur une autre machine
help-pack-long = Écrit ce nœud dans un fichier, à emporter sur une autre machine.

    Tout part : son nom, son registre, ce que d’autres ont signé sur lui,
    le fichier, et la clé de son adresse onion. Ensuite ce répertoire
    refuse de le faire tourner, pour que le nom ne soit jamais en deux
    lieux. Le fichier n’est pas chiffré : qui le détient est ce nœud.
    Emportez-le, dépaquetez-le, supprimez-le.

help-unpack = Place un nœud empaqueté dans le répertoire de nœud de cette machine
help-unpack-long = Place un nœud empaqueté dans le répertoire de nœud de cette machine.

    Refusé là où un nœud vit déjà. Rien n’est écrit avant que le fichier
    ait été lu en entier et que sa clé et son registre soient vérifiés.

help-moved = Dit que le répertoire de ce nœud a été déplacé ou renommé, pas copié
help-moved-long = Dit que le répertoire de ce nœud a été déplacé ou renommé, pas copié.

    Un nœud qui se trouve dans un nouvel endroit le dit à chaque lancement
    jusqu’à ce que ceci soit tapé, car une copie dont l’original tourne
    encore serait un même nom en deux lieux.

help-tell = Donne un ordre à un nœud en marche, avec les mots de son écran
help-tell-long = Donne un ordre à un nœud en marche, avec les mots de son écran.

    `tor on`, `tor off`, `bridge <line>`, `helper <program>`, et tout autre
    mot que l’écran accepte après `:`. Le nœud en marche l’exécute et sa
    réponse s’affiche ici. `say`, `join`, `ping`, `begin`, `status` et
    `stop` atteignent un nœud en marche de la même façon sans ceci.

help-tell-light = Donne un ordre à un nœud en marche
help-tell-long-light = Donne un ordre à un nœud en marche.

    `tor on`, `tor off`, `bridge <line>` et `helper <program>`. Le nœud en
    marche l’exécute et sa réponse s’affiche ici. `say`, `join`, `ping`,
    `begin`, `status` et `stop` atteignent un nœud en marche de la même
    façon sans ceci.

help-service = Gère le service d’arrière-plan (`start` et `stop` s’en servent)
help-service-long = Gère le service d’arrière-plan (`start` et `stop` s’en servent).

    Rien n’est installé avant que vous le demandiez, chaque fichier écrit
    et chaque commande lancée s’affichent au fur et à mesure, et
    `333 service uninstall` retire tout.

help-service-install = Installe le service d’arrière-plan avec ces options et le démarre
help-service-install-long = Installe le service d’arrière-plan avec ces options et le démarre.

    Le service lance `333 run` avec exactement les options données, pour
    le répertoire de ce nœud, et une vérification horaire le signale sur
    cette machine si le nœud s’arrête. `333 start` fait la même chose sans
    options.

help-service-uninstall = Arrête le service d’arrière-plan et retire tout ce qu’il a installé

help-service-status = Ce que dit le gestionnaire de services, quand le nœud a dit pour la
    dernière fois qu’il était éveillé, et ses dernières lignes

help-service-check = Le signale sur cette machine si le nœud s’est arrêté. Le service le
    lance toutes les heures ; il ne dit rien quand tout va bien

help-data-dir = Répertoire de tout ce que possède ce nœud : son nom, et l’état de Tor
    s’il utilise Tor

help-timeout = Secondes d’attente pour chaque étape qui parle au réseau
help-timeout-long = Secondes d’attente pour chaque étape qui parle au réseau.

    Un plafond, pas un délai. Il est prévu pour démarrer Tor, la seule
    étape qui peut prendre des minutes.

help-dangerously-trust-directory-permissions = Accepte un répertoire où d’autres utilisateurs peuvent entrer
help-dangerously-trust-directory-permissions-long = Accepte un répertoire où d’autres utilisateurs peuvent entrer.

    Le répertoire garde la seule copie du nom de ce nœud ; un répertoire
    aux droits trop larges est donc refusé par défaut. Ceci sert aux
    répertoires de test et aux conteneurs aux propriétaires étranges.

help-keep-everything = Garde toutes les déclarations pour toujours, plutôt que la seule
    fenêtre sur laquelle on juge
help-keep-everything-long = Garde toutes les déclarations pour toujours, plutôt que la seule
    fenêtre sur laquelle on juge.

    Cela ne change rien à la position de personne : chaque déclaration se
    vérifie pareil où qu’elle soit gardée.

help-bridges = Une ligne de pont, pour un réseau qui bloque l’accès normal à Tor
help-bridges-long = Une ligne de pont, pour un réseau qui bloque l’accès normal à Tor.

    Donnez-la une fois pour chaque pont reçu, exactement comme on vous l’a
    donné. Rien ici ne va chercher de ponts : ce sont des personnes qui
    les donnent, exprès, pour qu’aucune liste ne puisse être simplement
    récoltée et bloquée.

help-bridge-helper = Le programme qui parle un pont obscurci, par nom ou par chemin
help-bridge-helper-long = Le programme qui parle un pont obscurci, par nom ou par chemin.

    Nécessaire seulement quand une ligne de pont en demande un et que ce
    n’est pas `lyrebird` dans le chemin. Il n’est pas fourni, car une
    copie figée serait vite la mauvaise.

help-language = La langue à parler, en étiquette : `ko`, `es`, `zh-Hant`
help-language-long = La langue à parler, en étiquette : `ko`, `es`, `zh-Hant`.

    Sans elle, `THE333_LANGUAGE`, puis la langue gardée par
    `333 language <TAG>`, puis l’anglais. La langue du système n’est pas
    utilisée. `333 language` liste les langues qui ont des mots, et un
    dossier de catalogues dans `<data-dir>/words/<tag>/` en ajoute une
    sans rien compiler. Les 333 mots eux-mêmes ne sont jamais traduits.

help-count-in = Compte en base dix, douze ou twelve-ascii
help-count-in-long = Compte en base dix, douze ou twelve-ascii.

    Chaque nombre affiché est écrit dans cette base et chaque nombre tapé
    y est lu : `say 238` en douze est `say 332` en dix. Les noms, adresses,
    ports et versions ne sont jamais recomptés, et rien ne change sur le
    réseau. Sans elle, `THE333_COUNT_IN`, puis dix.

help-bootstrap-meet = Où chercher des gens avant de commencer seul

help-bootstrap-anyway = Commence même si quelqu’un est déjà là

help-serve-bind = Adresse et port d’écoute

help-serve-tor = Ouvre aussi une adresse onion, pour qu’on atteigne ce nœud sans
    savoir où il est. Réveiller Tor prend de quelques secondes à minutes

help-serve-no-direct = N’ouvre aucun socket. Seulement avec --tor ; votre adresse reste
    entièrement hors du réseau

help-serve-announce = L’adresse à laquelle les autres nœuds doivent atteindre celui-ci
help-serve-announce-long = L’adresse à laquelle les autres nœuds doivent atteindre celui-ci.

    Nécessaire quand le socket ne peut pas la dire : écoute sur toutes les
    interfaces, ou derrière quelque chose qui redirige un port.

help-serve-no-mdns = Ne dit pas sur le réseau local que ce nœud est ici
help-serve-no-mdns-long = Ne dit pas sur le réseau local que ce nœud est ici.

    Sinon, ce qui sort, c’est que quelque chose sur cette machine parle
    333 et sur quel port, pas le nom de ce nœud. C’est ainsi que deux
    nœuds d’une même maison se trouvent sans invitation.

help-serve-no-router = Ne demande pas au routeur d’envoyer le port à cette machine
help-serve-no-router-long = Ne demande pas au routeur d’envoyer le port à cette machine.

    Un routeur domestique jette ce que personne à l’intérieur n’a demandé,
    jusqu’à ce qu’un programme à l’intérieur lui demande de rediriger un
    port, par UPnP-IGD, PCP ou NAT-PMP. Cela change le réseau, c’est donc
    affiché quand cela arrive. `--no-upnp` est l’ancien nom de ceci.

help-serve-meet = Où chercher des nœuds que personne ne lui a présentés
help-serve-meet-long = Où chercher des nœuds que personne ne lui a présentés.

    Une adresse fixe qui garde des déclarations signées sur l’endroit où
    sont les nœuds. Tout ce qui y est lu est vérifié ici.

help-serve-no-meet = N’utilise aucun point de rencontre
help-serve-no-meet-long = N’utilise aucun point de rencontre.

    Ce nœud n’est alors atteignable que par ceux qui ont reçu une
    invitation et par les nœuds de ce réseau, et par personne d’autre.

help-serve-plain = Dit les lignes au lieu de dessiner l’écran
help-serve-plain-long = Dit les lignes au lieu de dessiner l’écran.

    Hors d’un terminal il dit toujours les lignes ; ceci le demande aussi
    dans un terminal.

help-serve-plain-light = Dit les lignes, ce que cette édition fait toujours
help-serve-plain-long-light = Dit les lignes, ce que cette édition fait toujours.

    Cette édition n’a pas d’écran. L’option est acceptée pour qu’une même
    commande marche dans les deux éditions.

help-say-index = Laquelle, de 0 à { $last }, tapée dans la base où l’on compte
    (--count-in). Les mots ne sont pas encore écrits

help-status-sources = Liste toutes les adresses que tient ce nœud : à qui elles sont, où et
    quand on en a entendu parler la première fois, et la dernière

help-status-json = Ce que ce nœud a observé, en JSON pour un programme. Il ne contient
    ni adresse ni port

help-join-address = Une invitation (`333:host:port`) de quelqu’un qui l’a déjà

help-ping-address = Une invitation (`333:host:port`), ou une adresse : `host`, `host:port`,
    `[::1]:port` ou `quelquechose.onion` (atteint par Tor)

help-pack-file = Le fichier à écrire. Il ne doit pas encore exister

help-pack-undo = Annule ici un empaquetage, pour un déménagement abandonné
help-pack-undo-long = Annule ici un empaquetage, pour un déménagement abandonné.

    Seulement si le fichier n’a été dépaqueté nulle part : sinon, cela en
    fait deux.

help-unpack-file = Le fichier qu’a écrit `333 pack`

help-tell-order = L’ordre, tel qu’on le taperait dans l’écran

help-tell-order-light = L’ordre, écrit comme `tor on` ou `bridge <line>`

help-service-install-flags = Les options de `run`, telles que vous les taperiez après lui

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = Affiche l’aide
help-print-help-more = Affiche l’aide (plus avec '--help')
help-print-help-summary = Affiche l’aide (un résumé avec '-h')
help-print-version = Affiche la version
help-print-this = Affiche ce message ou l’aide des sous-commandes données
help-print-for = Affiche l’aide des sous-commandes

help-start = Lance ce nœud en arrière-plan, maintenant et après chaque redémarrage

help-stop = Arrête ce nœud, et le garde arrêté après un redémarrage

help-restart = Arrête ce nœud, puis le relance en arrière-plan

help-logs = Montre les dernières lignes écrites par ce nœud en arrière-plan

help-logs-follow = Continue d’afficher les nouvelles lignes, là où systemd les garde

help-invite = Montre l’invitation qu’utilisent les autres pour rejoindre par ce nœud

help-status-all = Montre tout ce que sait ce nœud, avec le sens de chaque partie

help-languages-tag = La langue à garder, en étiquette : `ko`, `en`. `en` revient à l’anglais

help-start-example = Exemple : 333 start

help-stop-example = Exemple : 333 stop

help-restart-example = Exemple : 333 restart

help-status-example = Exemple : 333 status --all

help-logs-example = Exemple : 333 logs -f

help-id-example = Exemple : 333 name

help-invite-example = Exemple : 333 invite

help-bootstrap-example = Exemple : 333 begin

help-serve-example = Exemple : 333 run --tor

help-say-example = Exemple : 333 say 7

help-join-example = Exemple : 333 join 333:192.0.2.7:3333

help-languages-example = Exemple : 333 language fr

help-ping-example = Exemple : 333 ping 333:192.0.2.7:3333

help-pack-example = Exemple : 333 pack node.333

help-unpack-example = Exemple : 333 unpack node.333

help-moved-example = Exemple : 333 moved

help-tell-example = Exemple : 333 tell tor on

help-service-example = Exemple : 333 service status

help-service-install-example = Exemple : 333 service install --tor

help-service-uninstall-example = Exemple : 333 service uninstall

help-service-status-example = Exemple : 333 service status

help-service-check-example = Exemple : 333 service check

help-start-flags = Les options de `run`, telles que vous les taperiez après lui. Gardées
    pour chaque démarrage suivant jusqu’à ce que d’autres soient données

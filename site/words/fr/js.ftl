# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = Copier
js-copied = Copié
js-selected = Sélectionné
js-state-awake = Le nœud de ce site est éveillé
js-state-not-running = Le nœud de ce site ne tourne pas
js-in-hours = dans { $h } h { $m } min
js-in-minutes = dans { $m } min
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = époque n° { $n } de cette lignée

## The network

js-network-state-founder = Sur aucun rôle
js-network-state-ok = Répond à cette époque
js-network-state-quiet = Silencieux à cette époque
js-network-state-later = Compté à partir d’une époque ultérieure
js-network-state-seen = Vu, pas au rôle
js-network-awake = Éveillé
js-network-not-running = Ne tourne pas
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · le vôtre
js-network-find-bad = Le nom d’un nœud est en hexadécimal ; tapez au moins ses 6 premiers caractères.
js-network-find-none = Le nœud de ce site n’a vu aucun nœud de ce nom.
js-network-find-many = { $count } nœuds commencent ainsi. Tapez davantage du nom.
js-network-find-marked = Marqué comme le vôtre sur cet appareil.
js-network-select = Choisissez un nœud pour voir ce que le nœud de ce site sait de lui.
js-network-role-founder = Fondateur de cette lignée
js-network-role-site = Le nœud de ce site
js-network-role-yours = Le vôtre, sur cet appareil
js-network-role-none = Un nœud du réseau
js-network-col-name = Nom
js-network-col-state = Cette époque
js-network-col-given = A reçu le fichier
js-network-col-counted = Compté depuis
js-network-col-answered = Dernière réponse
js-network-col-said = A dit
js-network-col-reached = Joint
js-network-row-said = A dit à cette époque
js-network-row-handed = A remis le fichier à
js-network-row-testimony = Témoignage
js-network-given-by = époque { $epoch }, par { $sponsor }
js-network-given-founder = Par personne. Il a commencé cette lignée.
js-network-given-none = Pas au rôle
js-network-epoch = époque { $epoch }
js-network-epoch-now = { $epoch } (cette époque)
js-network-epoch-ago = { $epoch } (il y a { $ago })
js-network-more = et { $count } de plus dans le tableau ci-dessous
js-network-nothing = Rien
js-network-reach-direct = Directement
js-network-reach-tor = Par Tor
js-network-reach-tor-short = Tor
js-network-reach-unknown = Inconnu
js-network-testimony = interrogé par { $asked }, a interrogé { $asking } (3 dernières époques)
js-network-copy-name = Copier le nom
js-network-select-name = Sélectionnez le nom ci-dessus
js-network-mine = C’est mon nœud
js-network-tag-founder = fondateur
js-network-tag-site = ce site
js-network-tag-yours = le vôtre
js-network-empty = Le nœud de ce site n’a encore vu aucun autre nœud.
js-network-this-node = Ce nœud
js-network-yes = Oui
js-network-no = Non
js-network-none = Aucun

## Where we are

js-map-watch = Suivre en direct
js-map-stop = Arrêter de suivre
js-map-read-at = Lu à { $read_at } UTC.
js-map-unreadable = Le tableau n’a pas pu être lu à l’instant.
js-map-tor = Tor
js-map-nowhere = Nulle part où la périphérie puisse placer
js-map-nobody = Personne ne dit où il est.
js-map-all = Tous ceux qui le disent
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = Sur le réseau, sans dire où
js-map-dot = { $count ->
    [one] { $count } nœud
   *[other] { $count } nœuds
  }

## The board

js-board-said = Dit à l’époque { $epoch } par { $node }
js-board-site = le nœud de ce site
js-board-tor = par Tor

## Take the program

js-start-machine-linux-x86_64 = Linux sur x86-64
js-start-machine-linux-aarch64 = Linux sur ARM 64 bits
js-start-machine-linux-armv6 = Linux sur ARM 32 bits
js-start-machine-macos-aarch64 = un Mac avec Apple silicon
js-start-machine-macos-x86_64 = un Mac avec une puce Intel
js-start-machine-windows-x86_64 = Windows
js-start-phone = Cela ressemble à un téléphone ou à une tablette, et le programme est fait pour un ordinateur qui reste allumé. Choisissez cet ordinateur ici.
js-start-unknown = Ce navigateur ne dit pas sur quoi il tourne. Choisissez votre machine ici.
js-start-sure = Ce navigateur dit tourner sur { $machine }, c’est donc ce qui est choisi ici.
js-start-mac = Ce navigateur dit être sur un Mac sans dire quelle puce, donc Apple silicon est choisi ici. L’installateur le demande à la machine elle-même.
js-start-linux = Ce navigateur dit être sous Linux sans dire quel processeur, donc x86-64 est choisi ici. L’installateur le demande à la machine elle-même.
js-start-chosen = Choisi ci-dessus

## The story on the home page, drawn

js-story-file = 333.txt · 3 octets
js-story-gave = Je te l’ai remis
js-story-received = Je l’ai reçu de toi
js-story-signed = signé
js-story-minutes = 333 min
js-story-epochs = 333 époques
js-story-now = maintenant
js-story-answering = répondent
js-story-roll = au rôle
js-story-years = { $years } ans

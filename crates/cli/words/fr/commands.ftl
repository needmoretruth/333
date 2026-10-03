### What the shared parts of the commands say.

commands-clock-at-zero = { $epoch }. L’horloge de cette machine indique 1970 : ce nœud se croit
    donc au commencement des temps. Personne ne lui remettra rien ni ne
    témoignera pour lui tant que l’horloge n’est pas à l’heure.
    .keyword = époque

commands-called-first = la première clé créée a été choisie.
    .keyword = choisie

commands-called = { $not_called ->
        [one] { $not_called } clé a été créée et non choisie. celle-ci l’a été.
       *[other] { $not_called } clés ont été créées et non choisies. celle-ci l’a été.
    }
    .keyword = choisie

commands-torn = { $bytes } octets d’une entrée inachevée ont été retirés du registre
    .keyword = tronqué

commands-record = { $epochs ->
        [one] { $epochs } époque déjà répondue, aucune ouverte à révision
       *[other] { $epochs } époques déjà répondues, aucune ouverte à révision
    }
    .keyword = registre

commands-witnessed = { $statements ->
        [one] { $statements } déclaration qu’une autre clé a signée sur ce nœud. Elle est
            gardée après la fin de son époque, car rien d’autre d’elle ne
            survit à la fenêtre.
       *[other] { $statements } déclarations que d’autres clés ont signées sur ce nœud. Elles
            sont gardées après la fin de leurs époques, car rien d’autre
            d’elles ne survit à la fenêtre.
    }
    .keyword = témoin

commands-unseen = rien n’a été signé sur ce nœud, à aucune époque. Il atteint les
    autres, mais les autres ne l’atteignent pas, et seul le second compte :
    celui qui est tiré au sort pour interroger doit arriver. Deux choses
    en sont la cause : un routeur qui n’envoie pas le port 3333 à cette
    machine, ou une adresse que personne n’a reçue. `run --tor` n’a
    besoin ni de l’un ni de l’autre : une adresse onion s’atteint
    derrière n’importe quel routeur, et ce client emporte déjà Tor.
    .keyword = caché

commands-roll-alone = 1 d’entre nous, ce nœud
    .keyword = rôle

commands-roll = { $members } d’entre nous
    .keyword = rôle

commands-known = là où { $addresses } d’entre nous ont dit de chercher
    .keyword = connu

commands-holding = le fichier, et peut le transmettre
    .keyword = détient

commands-keeping = tout, pour toujours. Cela n’apporte rien à ce nœud : chaque
    déclaration porte sa propre signature et se vérifie pareil où qu’elle
    soit gardée. Il n’y a ni archive officielle ni archiviste.
    .keyword = garde

commands-ignored = { $admissions } admissions illisibles
    .keyword = ignoré

commands-learned-where = où sont { $addresses } de plus d’entre nous
    .keyword = appris

commands-rejoined = { $members } de plus d’entre nous par leur nom, d’un nœud qui en
    connaissait { $were }. Nous étions deux comptes, et le compte est un.
    .keyword = réuni

commands-learned-names = { $members } de plus d’entre nous par leur nom
    .keyword = appris

commands-heard = { $speakers } d’entre nous parlent
    .keyword = entendu

commands-carried = { $statements ->
        [one] { $statements } déclaration sur une époque encore ouverte
       *[other] { $statements } déclarations sur des époques encore ouvertes
    }
    .keyword = porté

commands-exchange = { $node }  époque { $epoch }  { $clocks }  ({ $liveness })
    .keyword = témoin

commands-answered-the-challenge = a répondu au défi que nous avons choisi
commands-spoke-first = a parlé en premier, ce qui prouve seulement qu’il a parlé

commands-clocks-together = horloges d’accord
commands-clocks-ahead = son horloge a { $apart } d’avance sur la nôtre
commands-clocks-behind = son horloge a { $apart } de retard sur la nôtre
commands-hours-and-minutes = { $hours } h { $minutes } min
commands-minutes-and-seconds = { $minutes } min { $seconds } s
commands-seconds = { $seconds } s

commands-waking = Tor. le chemin caché met du temps à s’ouvrir.
    .keyword = réveil

commands-waking-through = Tor, par { $bridges ->
        [one] { $bridges } pont
       *[other] { $bridges } ponts
    }. le chemin caché met du temps à s’ouvrir.
    .keyword = réveil

commands-no-tor = pas de connexion Tor après { $seconds } s
commands-starting-tor = démarrage du client Tor

# What a handover puts a signature under, read back.
commands-signed-giving = vous avez dit : je vous ai remis le fichier à l’époque { $epoch }.
    ils ont dit : j’ai reçu de vous le fichier à l’époque { $epoch }.
    c’est écrit de deux mains, et aucune main ne peut le reprendre.
    .keyword = signé

commands-signed-taking = ils ont dit : je vous ai remis le fichier à l’époque { $epoch }.
    vous avez dit : j’ai reçu de vous le fichier à l’époque { $epoch }.
    c’est écrit de deux mains, et aucune main ne peut le reprendre.
    .keyword = signé

commands-brimming = { $statements ->
        [one] { $statements } déclaration ne tenait pas dans un tour et attend le suivant
       *[other] { $statements } déclarations ne tenaient pas dans un tour et attendent le suivant
    }
    .keyword = déborde

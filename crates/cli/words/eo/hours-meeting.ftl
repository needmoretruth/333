### `333 run`: leaving this node's address at a meeting point, and reading everyone else's.

hours-meeting-unreadable = { $place } ne legeblis: { $why }
    .keyword = renkonto

hours-meeting-read-failed-inside = legi { $place } fiaskis ene de ĉi tiu nodo: { $why }
    .keyword = renkonto

hours-meeting-left = lasis la adreson de ĉi tiu nodo ĉe { $place }
    .keyword = renkonto

hours-meeting-stopped = ĉi tiu nodo haltis antaŭ ol { $place } respondis
    .keyword = renkonto

hours-meeting-leaving-failed-inside = lasi la adreson de ĉi tiu nodo ĉe { $place } fiaskis ene de
    ĉi tiu nodo: { $why }
    .keyword = renkonto

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place } akceptas unu deklaron minute de ĉiu interreta adreso,
    kaj ricevis unu de ĉi tiu adreso antaŭ malpli ol minuto.{ $holding }
    Ĉi tiu nodo lasas sian adreson denove { $when }.
    .keyword = renkonto

hours-meeting-full = { $place } akceptis ĉiujn deklarojn, kiujn ĝi akceptas en tago, kaj
    akceptas pliajn post noktomezo UTC. Ĝi ankoraŭ legeblas.{ $holding }
    Ĉi tiu nodo lasas sian adreson denove { $next_epoch }.
    .keyword = renkonto

hours-meeting-full-until = { $place } akceptis ĉiujn deklarojn, kiujn ĝi akceptas en tago, kaj
    akceptas pliajn post noktomezo UTC, post { $midnight }. Ĝi ankoraŭ
    legeblas.{ $holding }
    Ĉi tiu nodo lasas sian adreson denove { $next_epoch }.
    .keyword = renkonto

hours-meeting-holds-from = Ĝi ankoraŭ tenas la adreson de ĉi tiu nodo el epoko { $epoch }.
hours-meeting-holds-nothing = Ĝi tenas nenion de ĉi tiu nodo.

hours-meeting-at-the-next-epoch = ĉe la sekva epoko, post { $wait }
hours-meeting-in = post { $wait }

hours-meeting-did-not-reach = la adreso de ĉi tiu nodo ne atingis { $place }: { $why }
    .keyword = renkonto

hours-meeting-not-taken = { $place } ne akceptis la adreson de ĉi tiu nodo: { $why }
    .keyword = renkonto

hours-meeting-seconds = { $seconds ->
        [one] { $seconds } sekundo
       *[other] { $seconds } sekundoj
    }

hours-meeting-nobody = neniu diras sian lokon ĉe { $place }
    .keyword = renkonto

hours-meeting-newer = { $fresh ->
        [one] { $fresh } pli nova adreso
       *[other] { $fresh } pli novaj adresoj
    } ĉe { $place }
    .keyword = renkonto

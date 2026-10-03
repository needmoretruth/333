### `333 run`: carrying out what the node was told, from its screen or another terminal.

serve-carrying-not-written-who = notante, kiu respondis: { $why }
    .keyword = fiaskis

serve-carrying-unheard = { $why }
    .keyword = neaŭdita

serve-carrying-unbegun = { $why }
    .keyword = ne ekis

serve-carrying-refused = { $why }
    .keyword = rifuzita

serve-carrying-holding-the-file = { $roll } el ni en la nomlisto, kaj la dosiero estas ĉi tie
    .keyword = tenas

serve-carrying-holding-no-file = { $roll } el ni en la nomlisto, kaj ĉi tiu nodo ne ricevis la dosieron
    .keyword = tenas

serve-carrying-unread-holding = kion ĉi tiu nodo tenas: { $why }
    .keyword = nelegita

serve-carrying-already-up = la nevidebla adreso jam staras. `tor off` malstarigas ĝin.
    .keyword = staras

serve-carrying-unraised = { $why }
    .keyword = nestara

serve-carrying-tor-off = la onion-adreso ĉesas respondi nun. Kio jam estis dirita pri ĝi,
    validas ĝis ĝi estos forgesita, du epokojn post kiam ĝi estis dirita.
    .keyword = nevidita

serve-carrying-none-up = ne staras nevidebla adreso por malstarigi.
    .keyword = staras

serve-carrying-too-late = Tor jam funkcias, kaj ponto aldonita nun ŝanĝas nenion pri la
    konekto, kiun ĝi jam faris. Anstataŭe restartigu la nodon kun ĝi.
    .keyword = malfrue

serve-carrying-bridged = { $bridges ->
        [one] { $bridges } ponto estos uzata
       *[other] { $bridges } pontoj estos uzataj
    } la venontan fojon, kiam Tor startos.
    .keyword = pontoj

serve-carrying-helper = { $program } estos lanĉata por ĉiu malklarigita ponto.
    .keyword = pontoj

serve-carrying-not-an-address = { $typed } ne estas adreso: { $why }
    .keyword = nelegita

serve-carrying-stopping = petita el alia terminalo.
    .keyword = haltas

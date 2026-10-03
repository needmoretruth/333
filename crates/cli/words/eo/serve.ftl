### `333 run`.

serve-nothing-listening = nenio aŭskultus: --no-direct bezonas --tor

serve-name = { $name }
    .keyword = nomo

serve-waiting-for-the-file = ĉi tiu nodo ne ricevis la dosieron, do nenio estas kalkulata por ĝi
    ankoraŭ kaj estas ankoraŭ nenio por ke iu atestu. Ĝi ne povas fari
    ĝin. Ĝi alvenas nur de iu, kiu jam tenas ĝin, kaj vi ambaŭ
    subskribas la transdonon. Petu inviton, poste
    `333 join 333:ilia.adreso:3333`. Respondi intertempe kostas nenion
    kaj estas kiel homoj trovas vin.
    .keyword = atendas

serve-hand = invito nomas lokon, ne personon. Kiu respondas tie, pruvas sian
    identecon, tenante sian ŝlosilon.
    .keyword = fido

serve-invite = { $invitation }
    .keyword = invito

serve-answer = { $bound }
    .keyword = respondo

serve-nearby = dirante en ĉi tiu reto, ke io ĉi tie parolas 333, kaj aŭskultante
    la aliajn. Ne la nomo de ĉi tiu nodo: kio eliras, estas kion
    retpordo-skanado de la sama reto trovus. --no-mdns tenas ĉi tiun
    nodon ekster ĝi.
    .keyword = proksime

serve-nearby-failed = ne eblis komenci diri en ĉi tiu reto, ke ĉi tiu nodo estas ĉi tie: { $why }
    .keyword = proksime

serve-meet = { $place } estas kie ĉi tiu nodo serĉas homojn, al kiuj neniu prezentis
    ĝin. Ĉio legita tie estas subskribita de tiu, kiu diris ĝin, kaj
    nenio tie estas kredata. --no-meet tenas ĉi tiun nodon for de ĝi.
    .keyword = renkonto

serve-listener-stopped = aŭskultilo neatendite haltis

serve-farewell = finiĝis en epoko { $epoch }. Kiu estas lotita demandi pri vi, dum ĉi
    tio ne funkcias, subskribas, ke ĝi demandis kaj aŭdis nenion, kaj
    tion legas via fenestro. Ĝi longas { $window } epokojn, kaj ĝi
    moviĝas.
    .keyword = nodo

serve-farewell-on-no-roll = finiĝis en epoko { $epoch }. Vi estas en neniu nomlisto, do neniu
    eliras por demandi pri vi, kaj nenio estas subskribita pri vi, dum ĉi
    tio ne funkcias.
    .keyword = nodo

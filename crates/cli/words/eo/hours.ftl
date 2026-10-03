### `333 run`: keeping the hours, at every epoch boundary.

hours-failed-marking = registrante ĉi tiun epokon: { $why }
    .keyword = fiaskis

hours-failed-sources = notante, de kie adresoj venis: { $why }
    .keyword = fiaskis

hours-not-leaving = ne lasante ĉi tiun adreson ĉe { $place }. Ĝi atingas ĉi tiun nodon de
    ĉi tie kaj de nenie alie, kaj fremdulo, kiu vokus ĝin, atingus ion
    propran.
    .keyword = renkonto

hours-sealing = sigelante la adreson de ĉi tiu nodo

hours-failed-keeping-address = konservante la propran adreson de ĉi tiu nodo: { $why }
    .keyword = fiaskis

hours-failed-saying-where = dirante, kie ĉi tiu nodo estas: { $why }
    .keyword = fiaskis

hours-forgot = { $epochs ->
        [one] { $epochs } epoko
       *[other] { $epochs } epokoj
    }. nenio dirita pri ili nun povus ŝanĝi verdikton.
    .keyword = forgesis

hours-failed-forgetting = forgesante malnovajn deklarojn: { $why }
    .keyword = fiaskis

hours-minutes = { $minutes ->
        [one] { $minutes } minuto
       *[other] { $minutes } minutoj
    }

hours-minutes-and-seconds = { $minutes }m { $seconds }s
hours-hours-and-minutes = { $hours }h { $minutes }m

hours-epochs-answered-for = { $epochs ->
        [one] { $epochs } epoko respondita
       *[other] { $epochs } epokoj responditaj
    }

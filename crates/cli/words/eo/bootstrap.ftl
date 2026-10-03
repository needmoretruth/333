### `333 bootstrap`: beginning a line of your own, when there is nobody to join.

bootstrap-name = { $name }
    .keyword = nomo

bootstrap-vigil = `333 start` funkciigas ĝin, por ke ĝi respondu.
    .keyword = nodo

bootstrap-already-has-it = ĉi tiu nodo jam havas la dosieron. Estas nenio por komenci.

bootstrap-stop = { $already ->
        [one] { $already } el ni diras
       *[other] { $already } el ni diras
    }, kie oni povas atingi ilin, ĉe { $meet }. Komenci sola
    nun estigus duan linion apud ilia sen kialo. Malfermu { $board }
    en retumilo, prenu unu el la invitoj kaj lanĉu `333 join` per ĝi.
    Se vi legis tion kaj ankoraŭ volas komenci, `--anyway` diras tion.
    .keyword = haltu

bootstrap-not-the-file = kio revenis, ne estas la dosiero

bootstrap-begun = la dosiero estas en la dosierujo de ĉi tiu nodo, kaj ĉi tiu nodo estas
    la komenco de sia propra linio. Neniu subskribis por transdoni ĝin,
    ĉar neniu transdonis ĝin, kaj ĉiu, kiu legas la registron de ĉi tiu
    nodo, povas vidi tion.

    Tio estas la pozicio de la fondinto, ne ordinara. Nomlisto akceptas
    tiun, kiu ricevis la dosieron, do nodo, kiu ricevis ĝin de neniu,
    estas en neniu nomlisto: neniu venos ĉi tien por demandi ion, kaj
    ĉi tiu nodo neniam estas lotita por demandi iun. Ĝi tamen povas iri
    al tiuj lotitaj por demandi ĝin kaj tiel esti atestita.

    Kiun ajn vi poste donos la dosieron, tiu estas akceptita laŭ la
    ordinara maniero, kun subskriboj de vi ambaŭ, kaj estas kalkulata
    ekde tiu momento.
    .keyword = ekigita

bootstrap-reading-the-board = legante la afiŝtabulon ĉe { $place }

bootstrap-asking = { $meet } pri la dosiero
    .keyword = petas

bootstrap-asking-for-the-file = petante la dosieron de { $meet }

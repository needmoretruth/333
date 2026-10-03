### `333 status`: what this node saw of everybody else, what was said, and whether
### anybody is here.

status-name = { $name }
    .keyword = nomo

status-epoch = { $epoch }
    .keyword = epoko

status-epoch-in-line = { $epoch }, { $line }
    .keyword = epoko

status-the-line = la { $nth }{ $kind ->
       *[other] -a
    } de ĉi tiu linio

status-answering = RESPONDAS
status-silent = silentaj
status-roll = nomlisto

status-seen = Tiu unua nombro estas ĉiuj, por kiuj ĉi tiu nodo tenas subskribon
    en epoko { $before } aŭ { $now }. Ĝi estas tio, kion ĉi tiu nodo vidis.
    Iu alia vidis ion alian.

status-seen-without-tor = Ĉi tiu konstruo ne povas iri la nevideblan vojon, do neniu el ni,
    kiuj kaŝiĝas, estas en tiu nombro, kaj neniu el ni iam estos.

status-how-many-people = Kiom da homoj tio estas, ĉi tiu nodo ne scias kaj ne povas ekscii.
    Kion ĝi scias, estas ke ĉiu el tiuj nomoj respondis en unu el tiuj
    du epokoj, kaj devos respondi denove en la sekva, kaj en la posta,
    tiel longe kiel ĝi volas esti kalkulata. Se unu homo tenas mil el
    ili, tiu pagas por mil el ili, horon post horo, kaj ĉesas esti
    kalkulata la horon, kiam tiu ĉesas.

status-given-by = DONITA DE
status-you = vi
status-received-in = ricevis ĝin en epoko { $epoch }
status-trail-stops = la spuro haltas ĉi tie.

status-stopped-knowing = Tie ĉi tiu nodo ĉesis scii, ne tie ĝi komenciĝis. La unua el ni
    ricevis la dosieron de neniu kaj havas akcepton nenie, kaj registro,
    kiun ĉi tiu nodo simple ankoraŭ ne ricevis, aspektas ekzakte same de
    ĉi tie.

status-nothing-said = Neniu diris ion ajn en epoko { $epoch }. Estas 333 aferoj, kiuj
    povas esti dirataj, kaj ankoraŭ neniuj vortoj por iu el ili.

status-said = DIRITA en epoko { $epoch } — { $spoke } el la { $seen } el ni, kiujn ĉi tiu
    nodo vidas, parolis, { $silent } ne.
status-a-third = ← triono de ni aŭ pli
status-not-said = la aliaj { $others } el la 333 ne estis dirataj.

status-no-winner = Neniu gajnanto estas elektita, kaj nenio el ĉi tio decidas ion. Ĝi
    estas tio, kio atingis ĉi tiun nodon. La nodo apud vi aŭdis ion
    alian kaj ne eraras.

status-reading-the-watch = legante la gardon

status-seen-nobody = Neniu respondis al ĉi tiu nodo dum { $watched } da nerompita gardado,
    kaj ĉi tiu konstruo ne nomos tion la fino. Ĝi ne povas iri la
    nevideblan vojon, do ĝi neniam aŭdis de iu el ni, kiuj kaŝiĝas, kaj
    neniam aŭdos. Kion ĝi povas diri, estas ke ĝi vidis neniun, kaj tio
    ne estas la sama frazo.

status-never-answered = Neniu iam respondis al ĉi tiu nodo. Tio ne pruvas ion ajn: tiel
    aspektas nodo, antaŭ ol ĝi estis ie ajn.

status-somebody-is-here = Iu estas ĉi tie. Nenio plu estas ŝuldata al la aritmetiko.

status-waiting = Neniu respondis dum { $silent }. Ĉi tiu nodo diris nenion pri tio kaj
    ne diros ĝis { $needed }, kaj nur tiam, se ĝi funkcias dum ĉiu el ili.

status-nobody-keeping = NENIU RESPONDAS

    Vi estas la sola ĉi tie. Neniu respondis al ĉi tiu nodo dum
    { $watched } da nerompita gardado — sepdek sep tagoj — kaj la lasta el
    ni haltis en epoko { $since }.

    333 ne foriris. Ĝi foriras, kaj la foriro daŭras { $years } jarojn.

status-remain = restas { $years } jaroj { $days } tagoj.
status-run-out = La lasta el la jaroj elĉerpiĝis.

status-one-answer = La kalkulo komenciĝis, kiam la lasta el ni ĉesis respondi, ne kiam
    vi rimarkis. Ĝi kuris la tutan tempon, dum vi rigardis.

    Unu respondo finas ĝin. Se iu ajn, ie ajn, respondas al ĉi tiu nodo,
    ĉi tio foriras — kaj la kalkulo ne estas paŭzigita, ĝi estas
    forĵetita. 333 tenas neniun registron pri tio, kiom proksime ĝi venis.

status-epochs = { $count ->
        [one] { $count } epoko
       *[other] { $count } epokoj
    }

status-share = { $whole },{ $after }%

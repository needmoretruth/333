### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph.

help-about = Unu nodo de 333. Ĝi respondas, kiam oni demandas, tenas sian registron
    kaj pludonas la dosieron.

help-id = Montri la nomon de ĉi tiu nodo, farante unu ĉe la unua lanĉo

help-bootstrap = Komenci novan linion, kiam estas neniu, kiu povus doni la dosieron
help-bootstrap-long = Komenci novan linion, kiam estas neniu, kiu povus doni la dosieron.

    La ordinara vojo enen estas `333 join` kun invito. Ĉi tio unue
    rigardas la renkontejon kaj rifuzas, se iu estas tie. Se neniu estas,
    ĝi elŝutas la dosieron, kontrolas ĝin kontraŭ la haketaĵo, kiun ĉi tiu
    kliento portas, kaj skribas ĝin. Via nodo tiam estas la fondinto de
    sia propra linio, sen ies subskribo sur ĝia komenco, kaj ĉiu, kiu
    legas ĝian registron, povas vidi tion.

help-serve = Funkciigi ĉi tiun nodon en ĉi tiu terminalo, ĝis vi haltigos ĝin
help-serve-long = Funkciigi ĉi tiun nodon en ĉi tiu terminalo, ĝis vi haltigos ĝin.

    Ĝi respondas korbatojn kaj demandojn, interŝanĝas tion, kion ĝi scias,
    kaj demandas la nodojn, kiujn ĝi estas lotita demandi en ĉiu epoko.
    En terminalo ĝi malfermas la ekranon; `q`, Ctrl-C aŭ `333 stop` el
    alia terminalo haltigas ĝin. `333 start` faras la samon fone.

help-serve-long-light = Funkciigi ĉi tiun nodon en ĉi tiu terminalo, ĝis vi haltigos ĝin.

    Ĝi respondas korbatojn kaj demandojn, interŝanĝas tion, kion ĝi scias,
    kaj demandas la nodojn, kiujn ĝi estas lotita demandi en ĉiu epoko,
    po unu linio. Ctrl-C aŭ `333 stop` el alia terminalo haltigas ĝin.
    `333 start` faras la samon fone.

help-say = Diri unu el la 333, unufoje en epoko. Kio vojaĝas, estas la numero

help-status = Montri, ĉu ĉi tiu nodo funkcias, kie aliaj povas atingi ĝin, kaj
    kiom el ni respondas

help-join = Ricevi la dosieron de nodo, kiu havas ĝin, per invito

help-languages = Listigi la lingvojn, aŭ konservi unu por ĉiu komando ĉe ĉi tiu nodo

help-ping = Atingi alian nodon kaj interŝanĝi unu korbaton kun ĝi

help-pack = Skribi ĉi tiun nodon en unu dosieron, por porti ĝin al alia maŝino
help-pack-long = Skribi ĉi tiun nodon en unu dosieron, por porti ĝin al alia maŝino.

    Ĉio iras: ĝia nomo, ĝia registro, kion aliaj subskribis pri ĝi, la
    dosiero kaj la ŝlosilo de ĝia onion-adreso. Poste ĉi tiu dosierujo
    rifuzas funkciigi ĝin, do la nomo neniam estas en du lokoj. La dosiero
    ne estas ĉifrita: kiu tenas ĝin, tiu estas ĉi tiu nodo. Portu ĝin,
    malpaku ĝin, forigu ĝin.

help-unpack = Meti pakitan nodon en la nodan dosierujon de ĉi tiu maŝino
help-unpack-long = Meti pakitan nodon en la nodan dosierujon de ĉi tiu maŝino.

    Rifuzita, kie nodo jam loĝas. Nenio estas skribita, ĝis la dosiero
    estis tralegita kaj ĝiaj ŝlosilo kaj registro kontroliĝis.

help-moved = Diri, ke la dosierujo de ĉi tiu nodo estis movita, ne kopiita
help-moved-long = Diri, ke la dosierujo de ĉi tiu nodo estis movita, ne kopiita.

    Nodo, kiu trovas sin en nova loko, diras tion ĉe ĉiu lanĉo, ĝis ĉi tio
    estas tajpita, ĉar kopio kun la originalo ankoraŭ funkcianta estus
    unu nomo en du lokoj.

help-tell = Doni ordonon al funkcianta nodo, per la vortoj de ĝia ekrano
help-tell-long = Doni ordonon al funkcianta nodo, per la vortoj de ĝia ekrano.

    `tor on`, `tor off`, `bridge <line>`, `helper <program>` kaj ĉiu alia
    vorto, kiun la ekrano akceptas post `:`. La funkcianta nodo plenumas
    ĝin, kaj ĝia respondo estas presita ĉi tie. `say`, `join`, `ping`,
    `begin`, `status` kaj `stop` atingas funkciantan nodon same sen ĉi tio.

help-tell-light = Doni ordonon al funkcianta nodo
help-tell-long-light = Doni ordonon al funkcianta nodo.

    `tor on`, `tor off`, `bridge <line>` kaj `helper <program>`. La
    funkcianta nodo plenumas ĝin, kaj ĝia respondo estas presita ĉi tie.
    `say`, `join`, `ping`, `begin`, `status` kaj `stop` atingas
    funkciantan nodon same sen ĉi tio.

help-service = Administri la fonan servon rekte (`start` kaj `stop` uzas ĝin)
help-service-long = Administri la fonan servon rekte (`start` kaj `stop` uzas ĝin).

    Nenio estas instalita, ĝis vi petos, ĉiu skribita dosiero kaj lanĉita
    komando estas presita, kiam ĝi okazas, kaj `333 service uninstall`
    forigas ĉion.

help-service-install = Instali la fonan servon kun ĉi tiuj run-flagoj kaj lanĉi ĝin
help-service-install-long = Instali la fonan servon kun ĉi tiuj run-flagoj kaj lanĉi ĝin.

    La servo lanĉas `333 run` kun ĝuste la donitaj flagoj, por la
    dosierujo de ĉi tiu nodo, kaj ĉiuhora kontrolo apude diras tion sur
    ĉi tiu maŝino, se la nodo haltas. `333 start` faras la samon sen
    flagoj.

help-service-uninstall = Haltigi la fonan servon kaj forigi ĉion, kion ĝi instalis

help-service-status = Kion la servoadministrilo diras, kiam la nodo laste diris, ke ĝi estas
    maldorma, kaj la lastaj linioj, kiujn ĝi skribis

help-service-check = Diri tion sur ĉi tiu maŝino, se la nodo haltis. La servo lanĉas ĉi tion
    ĉiuhore; ĝi diras nenion, kiam ĉio estas en ordo

help-data-dir = Dosierujo, kiu tenas ĉion, kion ĉi tiu nodo posedas: ĝian nomon, kaj la
    staton de Tor, se ĝi uzas Tor

help-timeout = Sekundoj por atendi iun ajn unuopan paŝon, kiu parolas al la reto
help-timeout-long = Sekundoj por atendi iun ajn unuopan paŝon, kiu parolas al la reto.

    Plafono, ne prokrasto. Ĝi estas mezurita por lanĉi Tor, la sola paŝo,
    kiu povas daŭri minutojn.

help-dangerously-trust-directory-permissions = Akcepti dosierujon, en kiun aliaj sur ĉi tiu maŝino povas eniri
help-dangerously-trust-directory-permissions-long = Akcepti dosierujon, en kiun aliaj sur ĉi tiu maŝino povas eniri.

    La dosierujo tenas la solan kopion de la nomo de ĉi tiu nodo, do
    malstrikte permesita dosierujo estas defaŭlte rifuzata. Ĉi tio estas
    por provaj dosierujoj kaj ujoj kun stranga posedo.

help-keep-everything = Konservi ĉiun deklaron por ĉiam, anstataŭ nur la fenestro, laŭ kiu la
    stato estas legata
help-keep-everything-long = Konservi ĉiun deklaron por ĉiam, anstataŭ nur la fenestro, laŭ kiu la
    stato estas legata.

    Ĝi ŝanĝas nenion pri ies stato: ĉiu deklaro kontroliĝas same, kie
    ajn ĝi estas konservita.

help-bridges = Ponto-linio, por reto, kiu blokas la ordinaran vojon en Tor
help-bridges-long = Ponto-linio, por reto, kiu blokas la ordinaran vojon en Tor.

    Donu ĝin unufoje por ĉiu ponto, kiun vi ricevis, ĝuste kiel vi
    ricevis ĝin. Nenio ĉi tie elŝutas pontojn: homoj disdonas ilin,
    intence, por ke neniu listo simple kolekteblu kaj bloku.

help-bridge-helper = La programo, kiu parolas malklarigitan ponton, laŭ nomo aŭ vojo
help-bridge-helper-long = La programo, kiu parolas malklarigitan ponton, laŭ nomo aŭ vojo.

    Bezonata nur, kiam ponto-linio petas unu kaj ĝi ne estas `lyrebird`
    en la vojo. Ĝi ne estas kunmetita, ĉar frosta kopio baldaŭ estus la
    malĝusta.

help-language = La lingvo por paroli, kiel etikedo: `ko`, `es`, `zh-Hant`
help-language-long = La lingvo por paroli, kiel etikedo: `ko`, `es`, `zh-Hant`.

    Sen ĝi, `THE333_LANGUAGE`, poste la lingvo, kiun `333 language <TAG>`
    konservis, poste la angla. La lokaĵaro de la sistemo ne estas uzata.
    `333 language` listigas la lingvojn, por kiuj estas vortoj, kaj
    dosierujo de katalogoj en `<data-dir>/words/<tag>/` aldonas unu sen
    konstrui ion. La vortoj de 333 mem neniam estas tradukataj.

help-count-in = Kalkuli en dek, dek du aŭ twelve-ascii
help-count-in-long = Kalkuli en dek, dek du aŭ twelve-ascii.

    Ĉiu montrita nombro estas skribita en ĝi kaj ĉiu tajpita nombro
    legata en ĝi: `say 238` en dek du estas `say 332` en dek. Nomoj,
    adresoj, retpordoj kaj versioj neniam estas rekalkulataj, kaj nenio
    en la reto ŝanĝiĝas. Sen ĝi, `THE333_COUNT_IN`, poste dek.

help-bootstrap-meet = Kie serĉi homojn antaŭ ol komenci sola

help-bootstrap-anyway = Komenci, kvankam iu jam estas tie

help-serve-bind = Adreso kaj retpordo por aŭskulti

help-serve-tor = Ankaŭ starigi onion-adreson, por ke aliaj atingu ĉi tiun nodon sen
    ekscii, kie ĝi estas. Veki Tor daŭras de sekundoj ĝis minutoj

help-serve-no-direct = Tute ne malfermi konektingon. Nur kun --tor; ĝi tenas vian adreson
    tute ekster la reto

help-serve-announce = La adreso, ĉe kiu diri al aliaj nodoj atingi ĉi tiun
help-serve-announce-long = La adreso, ĉe kiu diri al aliaj nodoj atingi ĉi tiun.

    Bezonata, kiam la konektingo ne povas diri ĝin: aŭskultante ĉe ĉiu
    interfaco, aŭ malantaŭ io, kiu plusendas retpordon.

help-serve-no-mdns = Ne diri en la loka reto, ke ĉi tiu nodo estas ĉi tie
help-serve-no-mdns-long = Ne diri en la loka reto, ke ĉi tiu nodo estas ĉi tie.

    Kio alie eliras, estas, ke io sur ĉi tiu maŝino parolas 333 kaj ĉe
    kiu retpordo, ne la nomo de ĉi tiu nodo. Tiel du nodoj en unu domo
    trovas unu la alian sen invito.

help-serve-no-router = Ne peti la enkursigilon sendi la retpordon al ĉi tiu maŝino
help-serve-no-router-long = Ne peti la enkursigilon sendi la retpordon al ĉi tiu maŝino.

    Hejma enkursigilo forĵetas tion, kion neniu interne petis, ĝis
    programo interne petas ĝin plusendi retpordon, per UPnP-IGD, PCP aŭ
    NAT-PMP. Ĝi ŝanĝas la reton, do ĝi estas presita, kiam ĝi okazas.
    `--no-upnp` estas la pli malnova nomo por ĉi tio.

help-serve-meet = Kie serĉi nodojn, al kiuj neniu prezentis ĉi tiun
help-serve-meet-long = Kie serĉi nodojn, al kiuj neniu prezentis ĉi tiun.

    Unu fiksa adreso, kiu tenas subskribitajn deklarojn pri tio, kie
    nodoj estas. Ĉio legita tie estas kontrolata ĉi tie.

help-serve-no-meet = Tute ne uzi renkontejon
help-serve-no-meet-long = Tute ne uzi renkontejon.

    Ĉi tiu nodo tiam estas atingebla de tiuj, kiuj ricevis inviton, kaj
    de nodoj en ĉi tiu reto, kaj de neniu alia.

help-serve-plain = Diri la liniojn anstataŭ desegni la ekranon
help-serve-plain-long = Diri la liniojn anstataŭ desegni la ekranon.

    Ekster terminalo ĝi ĉiam diras la liniojn; ĉi tio petas tion ankaŭ
    en terminalo.

help-serve-plain-light = Diri la liniojn, kion ĉi tiu eldono ĉiam faras
help-serve-plain-long-light = Diri la liniojn, kion ĉi tiu eldono ĉiam faras.

    Ĉi tiu eldono ne havas ekranon. La flago estas akceptata, por ke unu
    komandlinio funkciu en ambaŭ eldonoj.

help-say-index = Kiu el ili, de 0 ĝis { $last }, tajpita en la bazo, en kiu ĉi tio kalkulas
    (--count-in). La vortoj ankoraŭ ne estas skribitaj

help-status-sources = Listigi ĉiun adreson, kiun ĉi tiu nodo tenas: kies ĝi estas, kie kaj
    kiam oni unue aŭdis pri ĝi, kaj kie laste

help-status-json = Kion ĉi tiu nodo observis, kiel JSON por programo. Neniu adreso aŭ
    retpordo estas en ĝi

help-join-address = Invito (`333:host:port`) de iu, kiu jam havas ĝin

help-ping-address = Invito (`333:host:port`), aŭ adreso: `host`, `host:port`,
    `[::1]:port` aŭ `io.onion` (atingita tra Tor)

help-pack-file = La dosiero por skribi. Ĝi ankoraŭ ne rajtas ekzisti

help-pack-undo = Repreni pakadon ĉi tie, por translokiĝo forlasita
help-pack-undo-long = Repreni pakadon ĉi tie, por translokiĝo forlasita.

    Nur se la dosiero neniam estis malpakita ie ajn: se ĝi estis, ĉi tio
    faras du.

help-unpack-file = La dosiero, kiun `333 pack` skribis

help-tell-order = La ordono, kiel ĝi estus tajpita en la ekranon

help-tell-order-light = La ordono, kiel `tor on` aŭ `bridge <line>` estas skribita

help-service-install-flags = La flagoj por `run`, kiel vi tajpus ilin post ĝi

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = Presi helpon
help-print-help-more = Presi helpon (vidu pli per '--help')
help-print-help-summary = Presi helpon (vidu resumon per '-h')
help-print-version = Presi version
help-print-this = Presi ĉi tiun mesaĝon aŭ la helpon de la donita(j) subkomando(j)
help-print-for = Presi helpon por la subkomando(j)

help-start = Funkciigi ĉi tiun nodon fone, nun kaj post ĉiu restartigo

help-stop = Haltigi ĉi tiun nodon kaj teni ĝin haltigita post restartigo

help-restart = Haltigi ĉi tiun nodon, poste funkciigi ĝin fone denove

help-logs = Montri la lastajn liniojn, kiujn ĉi tiu nodo skribis fone

help-logs-follow = Daŭre montri novajn liniojn, kiam ili venas, kie systemd tenas ilin

help-invite = Montri la inviton, per kiu aliaj aliĝas tra ĉi tiu nodo

help-status-all = Montri ĉion, kion ĉi tiu nodo scias, kun tio, kion ĉiu parto signifas

help-languages-tag = La lingvo por konservi, kiel etikedo: `ko`, `en`. `en` = la angla

help-start-example = Ekzemplo: 333 start

help-stop-example = Ekzemplo: 333 stop

help-restart-example = Ekzemplo: 333 restart

help-status-example = Ekzemplo: 333 status --all

help-logs-example = Ekzemplo: 333 logs -f

help-id-example = Ekzemplo: 333 name

help-invite-example = Ekzemplo: 333 invite

help-bootstrap-example = Ekzemplo: 333 begin

help-serve-example = Ekzemplo: 333 run --tor

help-say-example = Ekzemplo: 333 say 7

help-join-example = Ekzemplo: 333 join 333:192.0.2.7:3333

help-languages-example = Ekzemplo: 333 language eo

help-ping-example = Ekzemplo: 333 ping 333:192.0.2.7:3333

help-pack-example = Ekzemplo: 333 pack node.333

help-unpack-example = Ekzemplo: 333 unpack node.333

help-moved-example = Ekzemplo: 333 moved

help-tell-example = Ekzemplo: 333 tell tor on

help-service-example = Ekzemplo: 333 service status

help-service-install-example = Ekzemplo: 333 service install --tor

help-service-uninstall-example = Ekzemplo: 333 service uninstall

help-service-status-example = Ekzemplo: 333 service status

help-service-check-example = Ekzemplo: 333 service check

help-start-flags = La flagoj por `run`, kiel vi tajpus ilin post ĝi. Konservataj por ĉiu
    posta lanĉo, ĝis aliaj flagoj estos donitaj

### `333 run`: asking the router, keeping what it lent, and giving it back.

serve-reach-router-opened-upnp = la enkursigilo diras, ke la retpordo { $port } { $on } nun venas al ĉi
    tiu maŝino. Ĝi estas listigita tie kiel `333`, se vi volas forpreni
    ĝin denove. Ĉu io alvenas, diras la sekva linio.
    .keyword = malferma

serve-reach-router-opened-upnp-for = la enkursigilo diras, ke la retpordo { $port } { $on } nun venas al ĉi
    tiu maŝino, por { $time }. Ĝi estas listigita tie kiel `333`, se vi
    volas forpreni ĝin denove. Ĉu io alvenas, diras la sekva linio.
    .keyword = malferma

serve-reach-router-opened-lease = petis la enkursigilon ĉe { $router } per { $way } pri la retpordo { $port }
    por { $asked_for }. Ĝi diras, ke la retpordo { $granted_port } { $on } nun venas
    ĉi tien, por { $granted }. Ĉi tiu nodo petas denove antaŭ ol tio
    elĉerpiĝas kaj redonas ĝin, kiam la nodo haltas; se la nodo estas
    mortigita anstataŭe, la enkursigilo faligas ĝin, kiam la tempo
    finiĝas. Ĉu io alvenas, diras la sekva linio.
    .keyword = malferma

serve-reach-router-nobody-answered = neniu enkursigilo ĉi tie respondis peton malfermi retpordon, per
    UPnP-IGD, PCP aŭ NAT-PMP. Tio estas ordinara: multaj havas ĉiujn tri
    malŝaltitaj, kaj maŝino kun propra adreso havas nenion por peti.
    `--no-router` tute ĉesigas la petadon de ĉi tiu nodo.
    .keyword = fermita

serve-reach-router-refused = la enkursigilo ne volis malfermi la retpordon { $port }: { $why }
    .keyword = fermita

serve-reach-router-let-go = la enkursigilo lasis la retpordon { $port } iri: ĝi ne estis denove
    petita ĝustatempe, do neniu ekstere povas atingi ĉi tiun nodon per ĝi
    nun. Restartigi la nodon petas denove.
    .keyword = fermita

serve-reach-router-given-back = la retpordo { $port } estas redonita al la enkursigilo per { $way }; ĝi
    ne plu venas al ĉi tiu maŝino.
    .keyword = fermita

serve-reach-router-not-taken-back = la enkursigilo ne reprenis la retpordon { $port } ({ $why }). Ĝi
    faligas ĝin per si mem ene de { $time }.
    .keyword = fermita

serve-reach-router-moved = la enkursigilo movis ĉi tiun nodon: la retpordo { $port } { $on } nun venas
    ĉi tien anstataŭ la retpordo { $before_port } { $before_on }. Invito, kiu nomas la
    malnovan, ne plu alvenas.
    .keyword = malferma

serve-reach-router-not-kept = la enkursigilo ne tenis la retpordon { $port }, kiam petita ({ $why }).
    Ĝi ankoraŭ havas ĝin por { $time }, kaj estos petita denove antaŭ tiam.
    .keyword = atendas

serve-reach-router-on-its-outside-address = ĉe ĝia ekstera adreso
serve-reach-router-on = ĉe { $address }

serve-reach-router-one-second = unu sekundo
serve-reach-router-two-seconds = du sekundoj
serve-reach-router-seconds = { $count } sekundoj
serve-reach-router-one-minute = unu minuto
serve-reach-router-two-minutes = du minutoj
serve-reach-router-minutes = { $count } minutoj
serve-reach-router-one-hour = unu horo
serve-reach-router-two-hours = du horoj
serve-reach-router-hours = { $count } horoj

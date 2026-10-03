### `333 service` on Linux: systemd.

service-systemd-no-configuration = ĉi tiu sistemo nomas neniun agordan dosierujon por ĉi tiu uzanto

service-systemd-no-session = systemd ne tenas seancon por ĉi tiu uzanto ĉi tie ({ $why }). Ĝi
    komencas unu, kiam ĉi tiu uzanto ensalutas, ĉe la konzolo aŭ per ssh,
    kaj ne per su aŭ sudo. Ensalutu kiel ĉi tiu uzanto kaj lanĉu ĉi tion
    denove.

service-systemd-wrote-over = { $path }, anstataŭ tiu, kiu estis tie
    .keyword = skribis

service-systemd-linger-already = jam ŝaltita por { $user }. Ĝi tenas la nodon funkcianta post elsaluto,
    kaj lanĉas ĝin ĉe starto, kiam neniu estas ensalutinta.
    .keyword = linger

service-systemd-linger-on = ŝaltita por { $user }. Ĝi tenas la nodon funkcianta post elsaluto, kaj
    lanĉas ĝin ĉe starto, kiam neniu estas ensalutinta.
    .keyword = linger

service-systemd-linger-not-on = ne ŝaltita: { $why }. Sen ĝi la nodo haltas, kiam vi elsalutas, kaj
    atendas, ke vi ensalutu denove post restartigo.
    `sudo loginctl enable-linger { $user }` ŝaltas ĝin.
    .keyword = linger

service-systemd-linger-off = denove malŝaltita, kiel antaŭ la instalo.
    .keyword = linger

service-systemd-linger-left = lasita kiel ĝi estis. La instalo ne ŝaltis ĝin.
    .keyword = linger

service-systemd-not-answering = nekonata: systemd ne respondas por ĉi tiu uzanto
service-systemd-restarting = haltigita, kaj rekomencota 333 sekundojn post la halto
service-systemd-stopping = haltanta
service-systemd-failed = fiaskis ({ $result })
service-systemd-stopped = haltigita
service-systemd-at-every-boot = { $state }, kaj lanĉata ĉe ĉiu starto
service-systemd-not-again = { $state }, kaj { $file_state }: ĝi ne rekomencas per si mem

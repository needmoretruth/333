### `333 status`: where the others are, as far as this node knows.

status-known-another-copy = ALIA KOPIO DE ĈI TIU NOMO

status-known-sighting = Deklaro subskribita per la ŝlosilo de ĉi tiu nodo, kiun ĉi tiu nodo
    neniam faris, diras, ke ĝi estas ĉe { $address }, en epoko { $said_in }.
    Ĝi alvenis { $arrived }, en epoko { $epoch }.

status-known-either = Aŭ ĉi tiu dosierujo estis kopiita kaj la kopio lanĉita, aŭ iu alia
    havas la ŝlosilon. Du nodoj kun unu nomo kontraŭdiras unu la alian en
    ĉiu epoko, pri kiu iu el ili estas demandata. Haltigu unu el ili;
    `333 pack` estas kiel nodo translokiĝas. Nenio ĉi tie haltigas iun
    kopion por vi: malnovan deklaron iu ajn povas reludi, kaj nodon, kiu
    haltus vidante ĝin, povus malŝalti kiu ajn tenas kopion de ĝia
    ŝlosilo.

status-known-nowhere = ankoraŭ nenie por frapi. Invito donita al `333 ping` aŭ
    `333 join` estas tenata, kaj la nodo frapas tie de tiam.
    .keyword = KONATA

status-known-held = { $held ->
        [one] { $held } adreso
       *[other] { $held } adresoj
    }, laŭ kie ĉiu estis unue aŭdita
    .keyword = KONATA

status-known-by-hand = mane
status-known-this-network = ĉi tiu reto
status-known-meeting-point = renkontejo
status-known-from-us = de { $peers } el ni
status-known-not-noted = ne notita

status-known-where-heard = Kie ĉiu estis aŭdita, diras nenion pri tio, ĉu iu respondas tie.
status-known-sources-lists = `333 status --sources` listigas ilin.

status-known-sources = FONTOJ
status-known-nobody-answered = neniu ankoraŭ respondis ĉi tie
status-known-at = ĉe
status-known-first = unue
status-known-last = laste
status-known-from-before = tenita de antaŭ ol ĉi tiu nodo notis, de kie adresoj venis
status-known-when = { $from }, en epoko { $epoch }

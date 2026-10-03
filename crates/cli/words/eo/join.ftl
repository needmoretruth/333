### `333 join`: asking a node that holds the file to hand it over.

join-name = { $name }
    .keyword = nomo

join-knocking = { $address }
    .keyword = frapas

join-silence = neniu estis atingita ĉe { $address }. Tio ne pruvas, ke 333 finiĝis.
    Ĉi tiu kliento portas la haketaĵon de la dosiero kaj ne la dosieron:
    ne ekzistas vojo enen krom de iu, kiu tenas ĝin.
    .keyword = silento

join-knocking-on = frapante ĉe { $address }
join-exchanging = interŝanĝante korbatojn
join-asking = petante la dosieron
join-no-answer = neniu respondo de { $address } post { $seconds } s

join-given = de { $giver }
    .keyword = donita

join-joined = en epoko { $epoch }
    .keyword = aliĝis

join-holding = la dosiero, kaj povas pludoni ĝin
    .keyword = tenas

join-roll = { $members } el ni
    .keyword = nomlisto

join-counted = ekde epoko { $epoch }, kaj eĉ ne unu epokon pli frue: du limoj for, inter
    { $least } kaj { $most } minutoj, laŭ kie en ĉi tiu epoko vi alvenis.
    Ĝis tiam, respondu ĉion, kion oni demandas de vi. Kio estas atestita
    en tiu tempo, estas la tuta pruvo, ke vi iam ajn estis ĉi tie.
    .keyword = kalkulo

join-vigil = `333 start` tenas ĝin funkcianta de nun. Nenio atesteblas pri nodo,
    kiun neniu povas atingi, kaj ĉi tiu peco estas atestita unufoje aŭ
    neniam.
    .keyword = nodo

join-already-given = ĉi tiu nodo jam tenas la dosieron, donitan de { $giver } en epoko { $epoch }.
    Estas nenio por peti, kaj nenio estis petita.
join-same-handover = ĉi tiu nodo kaj { $peer } jam pasigis la dosieron inter si en epoko
    { $epoch }. Redonita en la sama epoko, ĝi estas tiu transdono legita
    de la alia flanko, kaj akceptas neniun, do nenio estis petita.

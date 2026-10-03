### `333 run`: orders from other terminals on this machine.

serve-told-not-here = en ĉi tiu sistemo ili atingas ĉi tiun nodon tra ĝia ekrano kaj nenie
    alie. Dua 333 lanĉita apud ĝi estas rifuzata.
    .keyword = ordonoj

serve-told-cannot = ne akcepteblaj el aliaj terminaloj: { $why }
    .keyword = ordonoj

serve-told-not-private = la konektingo ne povis esti farita privata

serve-told-taking = el ajna terminalo sur ĉi tiu maŝino, per la vortoj de la ekrano:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    Ĉi tiu nodo plenumas ilin kaj respondas tie.
    .keyword = ordonoj

serve-told-taking-light = el ajna terminalo sur ĉi tiu maŝino:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    Ĉi tiu nodo plenumas ilin kaj respondas tie.
    .keyword = ordonoj

serve-told-old-socket = malnova konektingo estas en la vojo kaj restas tie: { $why }
    .keyword = ordonoj

serve-told-not-a-socket = { $path } ekzistas kaj ne estas konektingo, do ĝi restas netuŝita, kaj
    nenio povas esti transdonita al ĉi tiu nodo el alia terminalo, ĝis
    ĝi estos movita.
    .keyword = ordonoj

serve-told-no-longer = ne plu estas akceptataj el aliaj terminaloj: { $why }
    .keyword = ordonoj

serve-told-not-the-owner = nur kiu posedas la dosierujon de ĉi tiu nodo povas ordoni al ĝi
    .keyword = rifuzita

serve-told-too-long = tio estas pli longa ol ajna ordono. La plej longa havas { $bytes } bajtojn.
    .keyword = nelegita

serve-told-other-version = ĉi tiu nodo parolas { $ours } kaj estis demandita en { $theirs }. La
    demandinta 333 estas alia versio ol la funkcianta; lanĉu tiun.
    .keyword = rifuzita

serve-told-unread = { $why }
    .keyword = nelegita

serve-told-asked = { $order }, el alia terminalo
    .keyword = petita

serve-told-unheard = nenio plu plenumas ordonojn
    .keyword = neaŭdita

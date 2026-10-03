### `333 invite`: the line others use to join through this node.

invite-line = { $invitation }
    .keyword = invito

invite-how = donu ĉi tiun linion al la alia persono. Tiu lanĉas
    `333 join { $invitation }` sur sia maŝino.
    .keyword = kiel

invite-none = ankoraŭ neniu. Ĉi tiu nodo ne trovis adreson, kiun aliaj povas atingi.
    .keyword = invito

invite-none-next = `333 start` funkciigas ĝin; demandu denove post kelkaj minutoj.
    Malantaŭ enkursigilo, kiun neniu malfermis,
    `333 service install --tor` funkciigas ĝin kun onion-adreso, kiu
    bezonas neniun enkursigilon.
    .keyword = sekve

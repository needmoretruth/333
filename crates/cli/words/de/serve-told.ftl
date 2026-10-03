### `333 run`: orders from other terminals on this machine.

serve-told-not-here = auf diesem System erreichen sie diesen Knoten über seinen
    Bildschirm und sonst nirgends. Ein zweites 333 daneben wird
    abgelehnt.
    .keyword = Befehle

serve-told-cannot = können nicht aus anderen Terminals kommen: { $why }
    .keyword = Befehle

serve-told-not-private = der Socket ließ sich nicht privat machen

serve-told-taking = aus jedem Terminal dieses Rechners, in den Worten des Bildschirms:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    Dieser Knoten führt sie aus und antwortet dort.
    .keyword = Befehle

serve-told-taking-light = aus jedem Terminal dieses Rechners:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    Dieser Knoten führt sie aus und antwortet dort.
    .keyword = Befehle

serve-told-old-socket = ein alter Socket ist im Weg und bleibt dort: { $why }
    .keyword = Befehle

serve-told-not-a-socket = { $path } ist da und ist kein Socket, also bleibt es unberührt, und
    aus einem anderen Terminal kann diesem Knoten nichts gegeben werden,
    bis es verschoben ist.
    .keyword = Befehle

serve-told-no-longer = kommen nicht mehr aus anderen Terminals: { $why }
    .keyword = Befehle

serve-told-not-the-owner = nur wem das Verzeichnis dieses Knotens gehört, kann ihm etwas sagen
    .keyword = Absage

serve-told-too-long = das ist länger als jeder Befehl. Der längste hat { $bytes } Bytes.
    .keyword = unlesbar

serve-told-other-version = dieser Knoten spricht { $ours } und wurde in { $theirs } gefragt. Das
    fragende 333 ist eine andere Version als die laufende; nimm die.
    .keyword = Absage

serve-told-unread = { $why }
    .keyword = unlesbar

serve-told-asked = { $order }, aus einem anderen Terminal
    .keyword = gefragt

serve-told-unheard = nichts führt mehr Befehle aus
    .keyword = ungehört

### `333 run`: the door, who gets in and who is turned away.

serve-door-over-tor = tra tor

serve-door-full = { $caller }: ĉi tiu pordo estas plena
    .keyword = forsendi

serve-door-silence-exchange = { $seconds } s da tio, do ni lasis iri
    .keyword = silento

serve-door-silence-greeting = { $seconds } s kaj eĉ ne unu vorto, do la pordo denove estas libera
    .keyword = silento

serve-door-knock = ĉi tiu nodo atingis sian propran pordon
    .keyword = frapo

serve-door-broken-heartbeat = { $caller } haltis antaŭ ol la korbato finiĝis: { $why }
    .keyword = rompita

serve-door-broken-exchange = { $caller } haltis antaŭ ol la interŝanĝo finiĝis: { $why }
    .keyword = rompita

serve-door-refused = { $caller }: { $why }
    .keyword = rifuzita

serve-door-failed-answering = respondante al { $caller }: { $why }
    .keyword = fiaskis

### `333 run`: the door, who gets in and who is turned away.

serve-door-over-tor = über Tor

serve-door-full = { $caller }: diese Tür ist voll
    .keyword = abgewies

serve-door-silence-exchange = { $seconds } s davon, also lassen wir los
    .keyword = Stille

serve-door-silence-greeting = { $seconds } s und kein Wort, also ist die Tür wieder frei
    .keyword = Stille

serve-door-knock = dieser Knoten erreichte seine eigene Haustür
    .keyword = klopft

serve-door-broken-heartbeat = { $caller } brach vor dem Ende des Herzschlags ab: { $why }
    .keyword = kaputt

serve-door-broken-exchange = { $caller } brach vor dem Ende des Tauschs ab: { $why }
    .keyword = kaputt

serve-door-refused = { $caller }: { $why }
    .keyword = Absage

serve-door-failed-answering = antworte { $caller }: { $why }
    .keyword = Fehler

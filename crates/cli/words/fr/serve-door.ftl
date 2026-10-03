### `333 run`: the door, who gets in and who is turned away.

serve-door-over-tor = par tor

serve-door-full = { $caller } : cette porte est pleine
    .keyword = renvoyé

serve-door-silence-exchange = { $seconds } s ainsi, alors nous lâchons
    .keyword = silence

serve-door-silence-greeting = { $seconds } s sans un mot, la porte est donc de nouveau libre
    .keyword = silence

serve-door-knock = ce nœud a atteint sa propre porte
    .keyword = frappe

serve-door-broken-heartbeat = { $caller } s’est arrêté avant la fin du battement : { $why }
    .keyword = rompu

serve-door-broken-exchange = { $caller } s’est arrêté avant la fin de l’échange : { $why }
    .keyword = rompu

serve-door-refused = { $caller } : { $why }
    .keyword = refusé

serve-door-failed-answering = réponse à { $caller } : { $why }
    .keyword = échec

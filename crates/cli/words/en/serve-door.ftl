### `333 serve`: the door, who gets in and who is turned away.

serve-door-over-tor = over tor

serve-door-full = { $caller }: this door is full
    .keyword = turned

serve-door-silence-exchange = { $seconds } s of it, so we let go
    .keyword = silence

serve-door-silence-greeting = { $seconds } s and not a word said, so the door is free again
    .keyword = silence

serve-door-knock = this node reached its own front door
    .keyword = knock

serve-door-broken-heartbeat = { $caller } stopped before the heartbeat was done: { $why }
    .keyword = broken

serve-door-broken-exchange = { $caller } stopped before the exchange was done: { $why }
    .keyword = broken

serve-door-refused = { $caller }: { $why }
    .keyword = refused

serve-door-failed-answering = answering { $caller }: { $why }
    .keyword = failed

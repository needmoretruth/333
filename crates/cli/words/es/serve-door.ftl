### `333 run`: the door, who gets in and who is turned away.

serve-door-over-tor = por tor

serve-door-full = { $caller }: esta puerta está llena
    .keyword = rehúsa

serve-door-silence-exchange = { $seconds } s así, así que lo soltamos
    .keyword = silencio

serve-door-silence-greeting = { $seconds } s sin decir palabra, así que la puerta vuelve a quedar libre
    .keyword = silencio

serve-door-knock = este nodo llegó a su propia puerta
    .keyword = llama

serve-door-broken-heartbeat = { $caller } se detuvo antes de terminar el latido: { $why }
    .keyword = roto

serve-door-broken-exchange = { $caller } se detuvo antes de terminar el intercambio: { $why }
    .keyword = roto

serve-door-refused = { $caller }: { $why }
    .keyword = negado

serve-door-failed-answering = respondiendo a { $caller }: { $why }
    .keyword = falló

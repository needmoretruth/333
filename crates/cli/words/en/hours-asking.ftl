### `333 run`: trading with the other nodes once an epoch, and asking whoever was drawn.

hours-asking-failed-gathering = gathering what this node could pass on: { $why }
    .keyword = failed

hours-asking-quiet = { $address }: { $why }
    .keyword = quiet

hours-asking-unended = the trading did not finish within { $time } of this epoch, and the
    rest of the hours will not wait for it
    .keyword = unended

hours-asking-reading-address = reading a peer's address
hours-asking-exchanging-heartbeats = exchanging heartbeats
hours-asking-trading = trading statements
hours-asking-putting = putting the question
hours-asking-sealing-presenting = sealing what this node came to say
hours-asking-saying-what-for = saying what this node came for
hours-asking-sealing-silence = sealing what did not happen

hours-asking-no-answer = no answer
hours-asking-did-not-answer = { $address } did not answer
hours-asking-did-not-finish = { $address } did not finish the heartbeat
hours-asking-neither = { $address } neither asked nor hung up
hours-asking-within = { $what } within the { $seconds } s the window allows
hours-asking-nothing-within = nothing within the { $seconds } s the window allows

hours-asking-going = nobody outside can open a connection to this node, so it goes to the
    { $drawn } of us drawn to ask it this epoch. The draw falls out of the epoch
    and the keys, so this node knows who they are without being told.
    .keyword = going

hours-asking-unknown-drawn-by = drawn to be asked by one of us that nobody has said the whereabouts of
    .keyword = unknown

hours-asking-unasked = epoch { $epoch }: { $why }
    .keyword = unasked

hours-asking-drawn = epoch { $epoch } — to ask { $asked } of us. Nobody chose that: the names fall out of
    the epoch and the keys, identically on every machine.
    .keyword = drawn

hours-asking-unknown-drawn-to-ask = drawn to ask one of us that nobody has said the whereabouts of
    .keyword = unknown

hours-asking-unheard = epoch { $epoch }: { $why }
    .keyword = unheard

hours-asking-witness = epoch { $epoch } answered by { $prover }
    .keyword = witness

hours-asking-silence = epoch { $epoch }: { $why }
    .keyword = silence

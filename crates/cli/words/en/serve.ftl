### `333 serve`.

serve-nothing-listening = nothing would be listening: --no-direct needs --tor

serve-name = { $name }
    .keyword = name

serve-waiting-for-the-file = this node has not been given the file, so nothing is counted for it
    yet and there is nothing yet for anybody to witness. It cannot make
    one. It only ever arrives from somebody who already holds it, and
    the two of you sign for the handover. Ask for an
    invitation, then `333 join 333:their.address:3333`. Answering in the
    meantime costs nothing and is how people find you.
    .keyword = waiting

serve-hand = an invitation names a place, not a person. it swears to nothing;
    whoever answers there proves themselves by holding a key.
    .keyword = hand

serve-invite = { $invitation }
    .keyword = invite

serve-answer = { $bound }
    .keyword = answer

serve-nearby = saying on this network that something here speaks 333, and
    listening for the others. Not this node's name: what goes out
    is what a port scan of the same network would find. --no-mdns
    keeps this node off it.
    .keyword = nearby

serve-nearby-failed = could not start saying on this network that this node is here: { $why }
    .keyword = nearby

serve-meet = { $place } is where this node looks for people nobody introduced it to.
    Everything read there is signed by whoever said it, and nothing
    there is believed. --no-meet keeps this node away from it.
    .keyword = meet

serve-listener-stopped = a listener stopped unexpectedly

serve-farewell = ended in epoch { $epoch }. Whoever is drawn to ask for you while this
    is not running signs that they asked and heard nothing, and
    that is what your window reads. It is { $window } epochs long, and it
    moves.
    .keyword = vigil

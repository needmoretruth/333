### `333 run`: carrying out what the node was told, from its screen or another terminal.

serve-carrying-not-written-who = writing down who answered: { $why }
    .keyword = failed

serve-carrying-unheard = { $why }
    .keyword = unheard

serve-carrying-unbegun = { $why }
    .keyword = unbegun

serve-carrying-refused = { $why }
    .keyword = refused

serve-carrying-holding-the-file = { $roll } of us on the roll, and the file is here
    .keyword = holding

serve-carrying-holding-no-file = { $roll } of us on the roll, and this node has not been given the file
    .keyword = holding

serve-carrying-unread-holding = what this node holds: { $why }
    .keyword = unread

serve-carrying-already-up = the unseen address is already up. `tor off` takes it down.
    .keyword = standing

serve-carrying-unraised = { $why }
    .keyword = unraised

serve-carrying-tor-off = the onion address stops answering now. What was already said about
    it stands until it is forgotten, two epochs from when it was said.
    .keyword = unseen

serve-carrying-none-up = there is no unseen address up to take down.
    .keyword = standing

serve-carrying-too-late = Tor is already running, and a bridge added now changes nothing about
    the connection it already made. Restart the node with it instead.
    .keyword = too late

serve-carrying-bridged = { $bridges ->
        [one] { $bridges } bridge
       *[other] { $bridges } bridges
    } will be used the next time Tor starts.
    .keyword = bridged

serve-carrying-helper = { $program } will be run for any obfuscated bridge.
    .keyword = bridged

serve-carrying-not-an-address = { $typed } is not an address: { $why }
    .keyword = unread

serve-carrying-stopping = asked from another terminal.
    .keyword = stopping

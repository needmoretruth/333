### `333 run`: orders from other terminals on this machine.

serve-told-not-here = on this system they reach this node through its screen and nowhere
    else. A second 333 started beside it is refused.
    .keyword = orders

serve-told-cannot = cannot be taken from other terminals: { $why }
    .keyword = orders

serve-told-not-private = the socket could not be made private

serve-told-taking = from any terminal on this machine, in the screen's words:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    This node carries them out and answers there.
    .keyword = orders

serve-told-taking-light = from any terminal on this machine:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    This node carries them out and answers there.
    .keyword = orders

serve-told-old-socket = an old socket is in the way and stays there: { $why }
    .keyword = orders

serve-told-not-a-socket = { $path } is there and is not a socket, so it is left alone, and
    nothing can be handed to this node from another terminal until
    it is moved.
    .keyword = orders

serve-told-no-longer = are no longer taken from other terminals: { $why }
    .keyword = orders

serve-told-not-the-owner = only whoever owns this node's directory can tell it anything
    .keyword = refused

serve-told-too-long = that is longer than any order. The longest is { $bytes } bytes.
    .keyword = unread

serve-told-other-version = this node speaks { $ours } and was asked in { $theirs }. The 333 that asked
    is a different version from the one running; run that one.
    .keyword = refused

serve-told-unread = { $why }
    .keyword = unread

serve-told-asked = { $order }, from another terminal
    .keyword = asked

serve-told-unheard = nothing is carrying orders out any more
    .keyword = unheard

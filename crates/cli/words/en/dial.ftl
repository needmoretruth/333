### Reaching another node, whichever way its address says to.
##
## A line here ends in `{` and the next begins with `""}` where the printed line is
## wider than a line of this file may be; the placeable spanning the two prints no
## line break.

dial-would-show = this node keeps its address unseen, so it will not open a connection {
    ""}to { $address }, which would show it

dial-no-answer = no answer after { $seconds } s

dial-waking = somebody worth reaching is at an unseen address and Tor is not up.
    The first bootstrap takes seconds to minutes, and nothing is
    asked of anybody until it is over.
    .keyword = waking

dial-unwoken = Tor did not start: { $why }
    Unseen addresses are skipped this epoch. The nodes behind them
    have not failed to answer — nothing reached them to ask.
    .keyword = unwoken

dial-connecting = connecting to { $address }
dial-without-tor = this client was built without Tor, so it cannot reach { $address }

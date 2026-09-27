### `333 bootstrap`: beginning a line of your own, when there is nobody to join.

bootstrap-name = { $name }
    .keyword = name

bootstrap-vigil = run `333 serve` to answer.
    .keyword = vigil

bootstrap-already-has-it = this node already has the file. There is nothing to begin.

bootstrap-stop = { $already ->
        [one] { $already } of us is
       *[other] { $already } of us are
    } saying where they can be reached at { $meet }. Beginning on
    your own now would start a second line beside theirs for no
    reason. Open { $board } in a browser, take one of the invitations, and
    run `333 join` with it instead.
    If you have read that and still mean to begin, `--anyway` says so.
    .keyword = stop

bootstrap-not-the-file = what came back is not the file

bootstrap-begun = the file is in this node's directory and this node is the start of its
    own line. Nobody signed for handing it over, because nobody did, and
    anybody reading this node's record can see that.

    That is the founder's position and not an ordinary one. A roll
    admits whoever received the file, so a node that received it from
    nobody is on no roll: nobody will come here to ask anything, and
    this node is never drawn to ask anybody. It can still go to the
    ones drawn to ask it and be witnessed that way.

    Whoever you hand the file to afterwards is admitted the ordinary
    way, with both of you signing, and is counted from that moment.
    .keyword = begun

bootstrap-reading-the-board = reading the board at { $place }

bootstrap-asking = { $meet } for the file
    .keyword = asking

bootstrap-asking-for-the-file = asking { $meet } for the file

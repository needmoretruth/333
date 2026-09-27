### What the shared parts of the commands say: opening a node, trading statements,
### one exchange, and the clock.

commands-clock-at-zero = { $epoch }. This machine's clock says it is 1970, so this node believes it is
    at the beginning of time. Nobody will hand it anything and nobody will
    witness it until the clock is set.
    .keyword = epoch

commands-called-first = the first key made was called.
    .keyword = called

commands-called = { $not_called ->
        [one] { $not_called } key was made and not called. this one was.
       *[other] { $not_called } keys were made and not called. this one was.
    }
    .keyword = called

commands-torn = { $bytes } bytes of an unfinished entry were dropped from the record
    .keyword = torn

commands-record = { $epochs ->
        [one] { $epochs } epoch
       *[other] { $epochs } epochs
    } already answered for, none of them open to revision
    .keyword = record

commands-witnessed = { $statements } statements other keys signed about this node. They are kept
    after the epochs they belong to are gone, because nothing else of
    them survives the window.
    .keyword = witness

commands-unseen = nothing has been signed about this node, in any epoch. Reaching out
    works and being reached does not, and only the second one is counted:
    whoever is drawn to ask has to arrive. Two things do this. A router
    that does not send port 3333 to this machine, and an address nobody
    was given. `serve --tor` needs neither — an onion address is reachable
    from behind any router, and this client already carries Tor.
    .keyword = unseen

commands-roll-alone = 1 of us, which is this node
    .keyword = roll

commands-roll = { $members } of us
    .keyword = roll

commands-known = where { $addresses } of us said to look
    .keyword = known

commands-holding = the file, and able to pass it on
    .keyword = holding

commands-keeping = everything, for ever. It buys this node nothing: every statement
    carries its own signature and verifies the same wherever it was
    kept. There is no archive of record and there is no archivist.
    .keyword = keeping

commands-ignored = { $admissions } admissions that could not be read
    .keyword = ignored

commands-learned-where = where { $addresses } more of us are
    .keyword = learned

commands-rejoined = { $members } more of us by name, from a node that knew { $were }. There were
    two of us and now the counting is one count.
    .keyword = rejoined

commands-learned-names = { $members } more of us by name
    .keyword = learned

commands-heard = { $speakers } of us speak
    .keyword = heard

commands-carried = { $statements } statements about epochs still open
    .keyword = carried

commands-exchange = { $node }  epoch { $epoch }  { $clocks }  ({ $liveness })
    .keyword = witness

commands-answered-the-challenge = answered the challenge we chose
commands-spoke-first = spoke first, which proves only that it spoke

commands-clocks-together = clocks together
commands-clocks-ahead = their clock { $apart } ahead of ours
commands-clocks-behind = their clock { $apart } behind ours
commands-hours-and-minutes = { $hours }h { $minutes }m
commands-minutes-and-seconds = { $minutes }m { $seconds }s
commands-seconds = { $seconds }s


commands-waking = Tor. the unseen road takes a while to open.
    .keyword = waking

commands-waking-through = Tor, through { $bridges ->
        [one] { $bridges } bridge
       *[other] { $bridges } bridges
    }. the unseen road takes a while to open.
    .keyword = waking

commands-no-tor = no Tor connection after { $seconds } s
commands-starting-tor = starting the Tor client

# What a handover puts a signature under, read back. The closing line is the same
# at both ends: it is the one formula both sides of the act speak.
commands-signed-giving = you said: I handed the file to you in epoch { $epoch }.
    they said: I received the file from you in epoch { $epoch }.
    it is written in two hands, and neither hand can take it back.
    .keyword = signed

commands-signed-taking = they said: I handed the file to you in epoch { $epoch }.
    you said: I received the file from you in epoch { $epoch }.
    it is written in two hands, and neither hand can take it back.
    .keyword = signed

commands-brimming = { $statements } statements would not fit in one run and wait for the next
    .keyword = brimming

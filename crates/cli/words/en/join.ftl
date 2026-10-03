### `333 join`: asking a node that holds the file to hand it over.

join-name = { $name }
    .keyword = name

join-knocking = { $address }
    .keyword = knocking

join-silence = nobody was reached at { $address }. That is not proof that 333 is
    over. This client carries the hash of the file and not the file:
    there is no way in except from someone who holds it.
    .keyword = silence

join-knocking-on = knocking on { $address }
join-exchanging = exchanging heartbeats
join-asking = asking for the file
join-no-answer = no answer from { $address } after { $seconds } s

join-given = by { $giver }
    .keyword = given

join-joined = in epoch { $epoch }
    .keyword = joined

join-holding = the file, and able to pass it on
    .keyword = holding

join-roll = { $members } of us
    .keyword = roll

join-counted = from epoch { $epoch }, and not one epoch sooner: two boundaries away, between
    { $least } and { $most } minutes, depending on where in this epoch you arrived.
    Until then, answer everything that is asked of you. What is witnessed
    in that time is the whole of the proof that you were ever here at all.
    .keyword = counted

join-vigil = `333 start` keeps it running from now on. Nothing can be witnessed of
    a node nobody can reach, and this stretch is witnessed once or never.
    .keyword = node

join-already-given = this node already holds the file, given by { $giver } in epoch { $epoch }.
    There is nothing to ask for, and nothing was asked.
join-same-handover = this node and { $peer } already passed the file between them in epoch
    { $epoch }. Handed back in the same epoch it is that handover read from
    the other side, and admits nobody, so nothing was asked.

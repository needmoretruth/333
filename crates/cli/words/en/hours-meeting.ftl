### `333 run`: leaving this node's address at a meeting point, and reading everyone else's.

hours-meeting-unreadable = { $place } could not be read: { $why }
    .keyword = meet

hours-meeting-read-failed-inside = reading { $place } failed inside this node: { $why }
    .keyword = meet

hours-meeting-left = left this node's address at { $place }
    .keyword = meet

hours-meeting-stopped = this node stopped before { $place } answered
    .keyword = meet

hours-meeting-leaving-failed-inside = leaving this node's address at { $place } failed inside this node: { $why }
    .keyword = meet

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place } takes one statement a minute from each internet address,
    and had one from this address less than a minute ago.{ $holding }
    This node leaves its address again { $when }.
    .keyword = meet

hours-meeting-full = { $place } has taken all the statements it takes in a day and takes
    more after midnight UTC. It can still be read.{ $holding }
    This node leaves its address again { $next_epoch }.
    .keyword = meet

hours-meeting-full-until = { $place } has taken all the statements it takes in a day and takes
    more after midnight UTC, in { $midnight }. It can still be read.{ $holding }
    This node leaves its address again { $next_epoch }.
    .keyword = meet

hours-meeting-holds-from = It still holds this node's address from epoch { $epoch }.
hours-meeting-holds-nothing = It holds nothing from this node.

hours-meeting-at-the-next-epoch = at the next epoch, in { $wait }
hours-meeting-in = in { $wait }

hours-meeting-did-not-reach = this node's address did not reach { $place }: { $why }
    .keyword = meet

hours-meeting-not-taken = { $place } did not take this node's address: { $why }
    .keyword = meet

hours-meeting-seconds = { $seconds ->
        [one] { $seconds } second
       *[other] { $seconds } seconds
    }

hours-meeting-nobody = nobody is saying where they are at { $place }
    .keyword = meet

hours-meeting-newer = { $fresh ->
        [one] { $fresh } newer address
       *[other] { $fresh } newer addresses
    } at { $place }
    .keyword = meet

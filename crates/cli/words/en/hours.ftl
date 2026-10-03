### `333 run`: keeping the hours, at every epoch boundary.

hours-failed-marking = recording this epoch: { $why }
    .keyword = failed

hours-failed-sources = writing down where addresses came from: { $why }
    .keyword = failed

hours-not-leaving = not leaving this address at { $place }. It reaches this node from here
    and from nowhere else, and a stranger who dialled it would
    reach something of their own.
    .keyword = meet

hours-sealing = sealing this node's address

hours-failed-keeping-address = keeping this node's own address: { $why }
    .keyword = failed

hours-failed-saying-where = saying where this node is: { $why }
    .keyword = failed

hours-forgot = { $epochs ->
        [one] { $epochs } epoch
       *[other] { $epochs } epochs
    }. nothing said about them now could change a verdict.
    .keyword = forgot

hours-failed-forgetting = forgetting old statements: { $why }
    .keyword = failed

hours-minutes = { $minutes ->
        [one] { $minutes } minute
       *[other] { $minutes } minutes
    }

hours-minutes-and-seconds = { $minutes }m { $seconds }s
hours-hours-and-minutes = { $hours }h { $minutes }m

hours-epochs-answered-for = { $epochs ->
        [one] { $epochs } epoch
       *[other] { $epochs } epochs
    } answered for

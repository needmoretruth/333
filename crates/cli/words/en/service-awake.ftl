### `333 service`: the line a running vigil writes to say it is still awake.

service-awake-failed = writing that the vigil is awake, in { $root }: { $why }. Nothing on this
    machine can tell that it is being kept until that works again.
    .keyword = failed

service-awake-never-kept = not kept. The service is installed and has never said it was awake.
    `333 service status` says why.
    .keyword = vigil

service-awake-not-kept-since = not kept since { $at }, { $ago } ago. `333 service status` says why.
    .keyword = vigil

service-awake-under-a-minute = under a minute

service-awake-minutes = { $minutes ->
        [one] { $minutes } minute
       *[other] { $minutes } minutes
    }

service-awake-epochs = { $epochs ->
        [one] { $epochs } epoch
       *[other] { $epochs } epochs
    }

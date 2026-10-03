### `333 service`: the line a running node writes to say it is still awake.

service-awake-failed = writing that the node is awake, in { $root }: { $why }. Nothing on this
    machine can tell that it is running until that works again.
    .keyword = failed

service-awake-never-kept = not running. The service is installed and the node has never said it
    was awake. `333 service status` says why.
    .keyword = node

service-awake-not-kept-since = not running since { $at }, { $ago } ago. `333 service status` says why.
    .keyword = node

service-awake-under-a-minute = under a minute

service-awake-minutes = { $minutes ->
        [one] { $minutes } minute
       *[other] { $minutes } minutes
    }

service-awake-epochs = { $epochs ->
        [one] { $epochs } epoch
       *[other] { $epochs } epochs
    }

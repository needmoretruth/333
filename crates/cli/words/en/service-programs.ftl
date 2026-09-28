### `333 service`: running the system's own programs, and saying so.

service-programs-ran = { $command }
    .keyword = ran

service-programs-did-not-succeed = `{ $command }` did not succeed: { $why }
service-programs-ended-with = it ended with { $status }
service-programs-no-such = there is no { $program } on this system
service-programs-not-started = { $program } could not be started: { $why }

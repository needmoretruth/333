### `333 service`: running the system's own programs, and saying so.

service-programs-ran = { $command }
    .keyword = lief

service-programs-did-not-succeed = `{ $command }` hat nicht geklappt: { $why }
service-programs-ended-with = es endete mit { $status }
service-programs-no-such = auf diesem System gibt es kein { $program }
service-programs-not-started = { $program } ließ sich nicht starten: { $why }

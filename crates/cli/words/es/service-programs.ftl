### `333 service`: running the system's own programs, and saying so.

service-programs-ran = { $command }
    .keyword = ejecutó

service-programs-did-not-succeed = `{ $command }` no salió bien: { $why }
service-programs-ended-with = terminó con { $status }
service-programs-no-such = no hay { $program } en este sistema
service-programs-not-started = no se pudo iniciar { $program }: { $why }

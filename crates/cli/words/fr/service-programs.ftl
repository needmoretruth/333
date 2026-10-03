### `333 service`: running the system's own programs, and saying so.

service-programs-ran = { $command }
    .keyword = lancé

service-programs-did-not-succeed = `{ $command }` n’a pas réussi : { $why }
service-programs-ended-with = il s’est terminé avec { $status }
service-programs-no-such = il n’y a pas de { $program } sur ce système
service-programs-not-started = { $program } n’a pas pu démarrer : { $why }

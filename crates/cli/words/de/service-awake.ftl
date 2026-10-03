### `333 service`: the line a running node writes to say it is still awake.

service-awake-failed = schreibe, dass der Knoten wach ist, in { $root }: { $why }. Nichts
    auf diesem Rechner kann sehen, dass er läuft, bis das wieder geht.
    .keyword = Fehler

service-awake-never-kept = läuft nicht. Der Dienst ist installiert, und der Knoten hat nie
    gesagt, er sei wach. `333 service status` sagt, warum.
    .keyword = Knoten

service-awake-not-kept-since = läuft nicht seit { $at }, vor { $ago }. `333 service status` sagt,
    warum.
    .keyword = Knoten

service-awake-under-a-minute = weniger als einer Minute

service-awake-minutes = { $minutes ->
        [one] { $minutes } Minute
       *[other] { $minutes } Minuten
    }

service-awake-epochs = { $epochs ->
        [one] { $epochs } Epoche
       *[other] { $epochs } Epochen
    }

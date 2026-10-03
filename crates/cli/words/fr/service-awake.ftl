### `333 service`: the line a running node writes to say it is still awake.

service-awake-failed = inscription de l’éveil du nœud, dans { $root } : { $why }. Rien sur
    cette machine ne peut savoir qu’il tourne avant que cela remarche.
    .keyword = échec

service-awake-never-kept = ne tourne pas. Le service est installé et le nœud n’a jamais dit
    qu’il était éveillé. `333 service status` dit pourquoi.
    .keyword = nœud

service-awake-not-kept-since = ne tourne plus depuis { $at }, il y a { $ago }. `333 service status`
    dit pourquoi.
    .keyword = nœud

service-awake-under-a-minute = moins d’une minute

service-awake-minutes = { $minutes ->
        [one] { $minutes } minute
       *[other] { $minutes } minutes
    }

service-awake-epochs = { $epochs ->
        [one] { $epochs } époque
       *[other] { $epochs } époques
    }

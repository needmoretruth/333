### `333 service`: the line a running node writes to say it is still awake.

service-awake-failed = skribante, ke la nodo estas maldorma, en { $root }: { $why }. Nenio sur
    ĉi tiu maŝino povas scii, ke ĝi funkcias, ĝis tio denove funkcios.
    .keyword = fiaskis

service-awake-never-kept = ne funkcias. La servo estas instalita, kaj la nodo neniam diris, ke
    ĝi estas maldorma. `333 service status` diras kial.
    .keyword = nodo

service-awake-not-kept-since = ne funkcias ekde { $at }, antaŭ { $ago }. `333 service status` diras kial.
    .keyword = nodo

service-awake-under-a-minute = malpli ol minuto

service-awake-minutes = { $minutes ->
        [one] { $minutes } minuto
       *[other] { $minutes } minutoj
    }

service-awake-epochs = { $epochs ->
        [one] { $epochs } epoko
       *[other] { $epochs } epokoj
    }

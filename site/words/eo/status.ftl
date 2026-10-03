# Status: the network now, how it has been one epoch at a time, and this site's machine.

status-meta-title = Stato · 333
status-meta-description = Kiel fartas la reto de 333 nun kaj kiel ĝi fartis, epokon post epoko: nodoj en la nomlisto, nodoj, kiuj respondas, deklaroj sur la afiŝtabulo, kaj ĉu la nodo kaj la maŝino de ĉi tiu retejo daŭre funkciis.
status-heading = Stato
status-lede = La nodo de ĉi tiu retejo rigardas la reton ĉiujn 15 sekundojn kaj notas la nombrojn unufoje en ĉiu epoko.
status-now-title = Nun
status-roll = En la nomlisto, inkluzive de la fondinto
status-saying = Dirantaj, kie ili estas
status-tor = El tiuj, per Tor
status-site-node = La nodo de ĉi tiu retejo
status-time-title = Laŭ la tempo
status-time-lede = Unu specimeno por ĉiu epoko: la lastaj nombroj, kiujn la nodo de ĉi tiu retejo donis antaŭ la fino de la epoko. Breĉo en linio estas epoko, kiun neniu notis.
status-chart-recent-title = La lastaj 333 epokoj
status-chart-all-title = Ĉio notita
# Under each chart, the same numbers as text. Every $variable is a number or a date
# (YYYY-MM-DD, UTC), or a dash when there is none.
status-chart-summary = Epokoj { $first } ĝis { $last }, de { $from } ĝis { $to }. En la nomlisto: plej malalte { $roll_low }, plej alte { $roll_high }, laste { $roll_latest }. Respondas: plej malalte { $answering_low }, plej alte { $answering_high }, laste { $answering_latest }.
status-chart-too-few = Malpli ol du epokoj estas notitaj por ĉi tiu periodo, do ankoraŭ ne estas linio por desegni.
status-machine-title = La maŝino de ĉi tiu retejo
status-release = Eldono
status-deployed = Instalita
status-observed = Laste rigardis la nodon
status-age = { $seconds ->
    [one] antaŭ 1 sekundo.
   *[other] antaŭ { $seconds } sekundoj.
}
status-observed-running = Ĝi ruliĝis.
status-observed-not-running = Ĝi ne ruliĝis.
status-uptime = Maŝino funkcias de
status-uptime-value = { $days ->
    [one] 1 tago
   *[other] { $days } tagoj
}, { $hours ->
    [one] 1 horo
   *[other] { $hours } horoj
}
status-elsewhere = Ĉiuj nodoj estas sur <a href="{ $base }/network">la reta paĝo</a>, kaj kie ili estas, sur <a href="{ $base }/map">la mapo</a>.
status-json = La samaj nombroj por programo: <a href="/api/status">/api/status</a>.

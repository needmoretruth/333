# Status: the network now, how it has been one epoch at a time, and this site's machine.

status-meta-title = État · 333
status-meta-description = Comment va le réseau 333 maintenant et comment il a été, époque par époque : nœuds au rôle, nœuds qui répondent, déclarations sur le tableau, et si le nœud et la machine du site ont continué de tourner.
status-heading = État
status-lede = Le nœud de ce site regarde le réseau toutes les 15 secondes et note les chiffres une fois par époque.
status-now-title = Maintenant
status-roll = Au rôle, fondateur compris
status-saying = Disent où ils sont
status-tor = Dont par Tor
status-site-node = Le nœud de ce site
status-time-title = Au fil du temps
status-time-lede = Un relevé par époque : les derniers chiffres donnés par le nœud de ce site avant la fin de l’époque. Un trou dans une ligne est une époque que personne n’a notée.
status-chart-recent-title = Les 333 dernières époques
status-chart-all-title = Tout ce qui a été noté
# Under each chart, the same numbers as text. Every $variable is a number or a date
# (YYYY-MM-DD, UTC), or a dash when there is none.
status-chart-summary = Époques { $first } à { $last }, du { $from } au { $to }. Au rôle : minimum { $roll_low }, maximum { $roll_high }, dernier { $roll_latest }. Répondent : minimum { $answering_low }, maximum { $answering_high }, dernier { $answering_latest }.
status-chart-too-few = Moins de deux époques sont notées pour cette période, il n’y a donc pas encore de ligne à tracer.
status-machine-title = La machine de ce site
status-release = Version
status-deployed = Déployée
status-observed = Dernier regard sur le nœud
status-age = { $seconds ->
    [one] il y a 1 seconde.
   *[other] il y a { $seconds } secondes.
}
status-observed-running = Il tournait.
status-observed-not-running = Il ne tournait pas.
status-uptime = Machine allumée depuis
status-uptime-value = { $days ->
    [one] 1 jour
   *[other] { $days } jours
}, { $hours ->
    [one] 1 heure
   *[other] { $hours } heures
}
status-elsewhere = Tous les nœuds sont listés sur <a href="{ $base }/network">la page du réseau</a>, et leur emplacement sur <a href="{ $base }/map">la carte</a>.
status-json = Les mêmes chiffres pour un programme : <a href="/api/status">/api/status</a>.

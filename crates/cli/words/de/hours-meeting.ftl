### `333 run`: leaving this node's address at a meeting point, and reading everyone else's.

hours-meeting-unreadable = { $place } war nicht lesbar: { $why }
    .keyword = Treff

hours-meeting-read-failed-inside = das Lesen von { $place } schlug in diesem Knoten fehl: { $why }
    .keyword = Treff

hours-meeting-left = Adresse dieses Knotens bei { $place } hinterlassen
    .keyword = Treff

hours-meeting-stopped = dieser Knoten stoppte, bevor { $place } antwortete
    .keyword = Treff

hours-meeting-leaving-failed-inside = die Adresse dieses Knotens bei { $place } zu hinterlassen,
    schlug in diesem Knoten fehl: { $why }
    .keyword = Treff

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place } nimmt eine Aussage pro Minute von jeder
    Internetadresse an und hatte vor weniger als einer Minute eine von
    dieser.{ $holding }
    Dieser Knoten hinterlässt seine Adresse wieder { $when }.
    .keyword = Treff

hours-meeting-full = { $place } hat alle Aussagen eines Tages angenommen und nimmt nach
    Mitternacht UTC wieder welche an. Lesen geht noch.{ $holding }
    Dieser Knoten hinterlässt seine Adresse wieder { $next_epoch }.
    .keyword = Treff

hours-meeting-full-until = { $place } hat alle Aussagen eines Tages angenommen und nimmt nach
    Mitternacht UTC wieder welche an, in { $midnight }. Lesen geht noch.{ $holding }
    Dieser Knoten hinterlässt seine Adresse wieder { $next_epoch }.
    .keyword = Treff

hours-meeting-holds-from = Er hält die Adresse dieses Knotens noch aus Epoche { $epoch }.
hours-meeting-holds-nothing = Er hält nichts von diesem Knoten.

hours-meeting-at-the-next-epoch = in der nächsten Epoche, in { $wait }
hours-meeting-in = in { $wait }

hours-meeting-did-not-reach = die Adresse dieses Knotens erreichte { $place } nicht: { $why }
    .keyword = Treff

hours-meeting-not-taken = { $place } nahm die Adresse dieses Knotens nicht an: { $why }
    .keyword = Treff

hours-meeting-seconds = { $seconds ->
        [one] { $seconds } Sekunde
       *[other] { $seconds } Sekunden
    }

hours-meeting-nobody = niemand sagt bei { $place }, wo er ist
    .keyword = Treff

hours-meeting-newer = { $fresh ->
        [one] { $fresh } neuere Adresse
       *[other] { $fresh } neuere Adressen
    } bei { $place }
    .keyword = Treff

### Reaching another node, whichever way its address says to.

dial-would-show = dieser Knoten hält seine Adresse verdeckt, also öffnet er keine
    Verbindung zu { $address }, die sie zeigen würde

dial-no-answer = keine Antwort nach { $seconds } s

dial-waking = jemand, den es zu erreichen lohnt, hat eine verdeckte Adresse, und
    Tor läuft nicht. Der erste Start dauert Sekunden bis Minuten, und
    bis dahin wird niemand etwas gefragt.
    .keyword = weckt

dial-unwoken = Tor ist nicht gestartet: { $why }
    Verdeckte Adressen werden in dieser Epoche übersprungen. Die Knoten
    dahinter haben nicht versäumt zu antworten — nichts hat sie
    erreicht, um zu fragen.
    .keyword = schläft

dial-connecting = verbinde mit { $address }
dial-without-tor = dieser Client wurde ohne Tor gebaut, also kann er { $address } nicht erreichen

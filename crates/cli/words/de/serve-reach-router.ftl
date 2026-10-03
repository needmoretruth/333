### `333 run`: asking the router, keeping what it lent, and giving it back.

serve-reach-router-opened-upnp = der Router sagt, Port { $port } { $on } kommt jetzt zu diesem
    Rechner. Er steht dort als `333`, falls du ihn wieder wegnehmen
    willst. Ob etwas ankommt, sagt die nächste Zeile.
    .keyword = offen

serve-reach-router-opened-upnp-for = der Router sagt, Port { $port } { $on } kommt jetzt zu diesem
    Rechner, für { $time }. Er steht dort als `333`, falls du ihn wieder
    wegnehmen willst. Ob etwas ankommt, sagt die nächste Zeile.
    .keyword = offen

serve-reach-router-opened-lease = den Router bei { $router } über { $way } um Port { $port } für
    { $asked_for } gebeten. Er sagt, Port { $granted_port } { $on } kommt jetzt
    hierher, für { $granted }. Dieser Knoten fragt vor Ablauf erneut und
    gibt ihn beim Stoppen zurück; wird der Knoten stattdessen beendet,
    lässt der Router ihn nach Ablauf fallen. Ob etwas ankommt, sagt die
    nächste Zeile.
    .keyword = offen

serve-reach-router-nobody-answered = kein Router hier hat auf eine Bitte, einen Port zu öffnen,
    geantwortet, weder über UPnP-IGD noch PCP noch NAT-PMP. Das ist
    üblich: viele haben alle drei aus, und ein Rechner mit eigener
    Adresse muss nichts erbitten. `--no-router` lässt diesen Knoten gar
    nicht erst fragen.
    .keyword = zu

serve-reach-router-refused = der Router wollte Port { $port } nicht öffnen: { $why }
    .keyword = zu

serve-reach-router-let-go = der Router hat Port { $port } losgelassen: er wurde nicht
    rechtzeitig erneut erbeten, also erreicht niemand draußen diesen
    Knoten mehr darüber. Ein Neustart des Knotens fragt erneut.
    .keyword = zu

serve-reach-router-given-back = Port { $port } ist über { $way } an den Router zurückgegeben; er
    kommt nicht mehr zu diesem Rechner.
    .keyword = zu

serve-reach-router-not-taken-back = der Router hat Port { $port } nicht zurückgenommen ({ $why }). Er
    lässt ihn von selbst innerhalb von { $time } fallen.
    .keyword = zu

serve-reach-router-moved = der Router hat diesen Knoten verschoben: Port { $port } { $on }
    kommt jetzt hierher statt Port { $before_port } { $before_on }. Eine
    Einladung mit dem alten kommt nicht mehr an.
    .keyword = offen

serve-reach-router-not-kept = der Router hat Port { $port } auf Bitte nicht behalten ({ $why }).
    Er hat ihn noch für { $time } und wird vorher erneut gefragt.
    .keyword = wartet

serve-reach-router-on-its-outside-address = auf seiner äußeren Adresse
serve-reach-router-on = auf { $address }

serve-reach-router-one-second = eine Sekunde
serve-reach-router-two-seconds = zwei Sekunden
serve-reach-router-seconds = { $count } Sekunden
serve-reach-router-one-minute = eine Minute
serve-reach-router-two-minutes = zwei Minuten
serve-reach-router-minutes = { $count } Minuten
serve-reach-router-one-hour = eine Stunde
serve-reach-router-two-hours = zwei Stunden
serve-reach-router-hours = { $count } Stunden

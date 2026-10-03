### What a command says when another 333 already has this node's directory.

elsewhere-the-vigil = der Knoten, der in diesem Verzeichnis läuft
elsewhere-the-vigil-by-number = der Knoten, der in diesem Verzeichnis läuft (Prozess { $pid })
elsewhere-another = ein anderes 333
elsewhere-another-by-number = ein anderes 333 (Prozess { $pid })

elsewhere-done = ausgeführt von { $vigil }.
    .keyword = erledigt

elsewhere-failed = { $vigil } hat das nicht getan.
    .keyword = Fehler

elsewhere-finding-its-name = { $who } sucht noch den Namen
    dieses Knotens. Führe das erneut aus, wenn er einen hat.
    .keyword = belegt

elsewhere-already-keeping = { $who } läuft hier schon, und ein Verzeichnis ist ein Knoten.
    Von hier aus kann man ihm Dinge sagen: `333 say 7`,
    `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`. Ein zweiter
    Knoten braucht ein eigenes Verzeichnis, angegeben mit --data-dir.
    .keyword = belegt

elsewhere-keeping = { $who } läuft hier, und
    { $why }
    .keyword = belegt

elsewhere-nobody-to-tell = in diesem Verzeichnis läuft kein Knoten, also gibt es niemanden,
    dem man es sagen kann. `333 start` oder `333 run` startet ihn, dann
    klappt das.
    .keyword = niemand

elsewhere-busy = { $who } hat das Verzeichnis dieses
    Knotens und ist kein laufender Knoten, dem man das geben kann. Hier
    wurde nichts gelesen oder geschrieben. Führe das erneut aus, wenn er
    fertig ist.
    .keyword = belegt

elsewhere-busy-cannot-be-handed = { $who } hat das Verzeichnis dieses
    Knotens. Auf diesem System kann man einem laufenden 333 noch nichts
    aus einem anderen Terminal geben, also wurde hier nichts gelesen
    oder geschrieben. Tippe es in seinem Bildschirm nach `:` ein, oder
    stoppe ihn und führe das erneut aus.
    .keyword = belegt

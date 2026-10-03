### `333 run`: carrying out what the node was told, from its screen or another terminal.

serve-carrying-not-written-who = schreibe auf, wer geantwortet hat: { $why }
    .keyword = Fehler

serve-carrying-unheard = { $why }
    .keyword = ungehört

serve-carrying-unbegun = { $why }
    .keyword = nicht

serve-carrying-refused = { $why }
    .keyword = Absage

serve-carrying-holding-the-file = { $roll } von uns auf der Liste, und die Datei ist hier
    .keyword = hält

serve-carrying-holding-no-file = { $roll } von uns auf der Liste, und dieser Knoten hat die Datei nicht bekommen
    .keyword = hält

serve-carrying-unread-holding = was dieser Knoten hält: { $why }
    .keyword = unlesbar

serve-carrying-already-up = die verdeckte Adresse ist schon offen. `tor off` schließt sie.
    .keyword = offen

serve-carrying-unraised = { $why }
    .keyword = zu

serve-carrying-tor-off = die Onion-Adresse antwortet ab jetzt nicht mehr. Was schon über
    sie gesagt wurde, gilt, bis es vergessen ist, zwei Epochen danach.
    .keyword = verdeckt

serve-carrying-none-up = es ist keine verdeckte Adresse offen, die man schließen könnte.
    .keyword = offen

serve-carrying-too-late = Tor läuft schon, und eine jetzt hinzugefügte Brücke ändert nichts
    an der Verbindung, die es schon aufgebaut hat. Starte den Knoten
    stattdessen damit neu.
    .keyword = zu spät

serve-carrying-bridged = { $bridges ->
        [one] { $bridges } Brücke wird
       *[other] { $bridges } Brücken werden
    } beim nächsten Start von Tor genutzt.
    .keyword = Brücke

serve-carrying-helper = { $program } wird für jede verschleierte Brücke gestartet.
    .keyword = Brücke

serve-carrying-not-an-address = { $typed } ist keine Adresse: { $why }
    .keyword = unlesbar

serve-carrying-stopping = aus einem anderen Terminal verlangt.
    .keyword = stoppt

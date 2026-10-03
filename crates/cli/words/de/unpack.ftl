### `333 unpack`: putting a packed node into this machine's node directory.

-unpack-nothing = Es wurde nichts entpackt.
-unpack-so-nothing = also wurde nichts entpackt.

unpack-kept = in diesem Verzeichnis lebt schon ein Knoten. Um daneben zu
    entpacken, gib ihm mit --data-dir ein eigenes Verzeichnis.

unpack-seed-in = der { $seed } in { $file }

unpack-not-one-node = diese Datei sagt, sie halte { $claimed }, und der Schlüssel darin ist { $name }. { -unpack-not-one }
-unpack-not-one = Das ist nicht ein Knoten, und nichts wurde entpackt.

unpack-taken = ein anderes 333 hat das Zielverzeichnis genommen. { -unpack-nothing }

unpack-elsewhere = 333 --data-dir <ein anderes Verzeichnis> unpack { $file }

unpack-could-not-open = in { $target } lebt schon ein Knoten, und er { -unpack-could-not-be-opened } Um daneben zu entpacken: { $elsewhere }
-unpack-could-not-be-opened = ließ sich nicht öffnen, um zu sagen, was er hält. { -unpack-nothing }

unpack-no-record = noch keine Chronik
unpack-epochs-of-record = { $epochs ->
        [one] { $epochs } Epoche Chronik
       *[other] { $epochs } Epochen Chronik
    }
unpack-holding = hält die Datei
unpack-not-holding = hält die Datei nicht

unpack-occupied = in { $target } lebt schon ein Knoten:
    { $name }, { $epochs }, { $holding }.
    Darüber zu entpacken verlöre all das für immer, { -unpack-so-nothing }
    Um daneben zu entpacken, gib ihm ein eigenes Verzeichnis:
    { $elsewhere }

unpack-holds-files = { $target } enthält Dateien und keinen Knoten. { -unpack-its-own } Woanders: { $elsewhere }
-unpack-its-own = Ein Knoten wird in ein eigenes Verzeichnis entpackt, { -unpack-so-nothing }

unpack-opening-the-record = öffne die Chronik
unpack-torn = die Chronik in dieser Datei ist zerrissen, also kein ganzer Knoten. { -unpack-nothing }
unpack-reading-the-record = lese die Chronik
unpack-does-not-verify = die Chronik in dieser Datei lässt sich nicht prüfen. { -unpack-nothing }
unpack-another-key = die Chronik in dieser Datei schrieb ein anderer Schlüssel. { -unpack-nothing }

unpack-not-a-place = { $target } ist kein Verzeichnis, in das ein Knoten kann
unpack-making-room = schaffe Platz in { $target }
unpack-putting = lege den Knoten nach { $target }

unpack-name = { $name }
    .keyword = Name

unpack-record-none = noch keine
    .keyword = Chronik

unpack-record = { $epochs ->
        [one] { $epochs } Epoche, geprüft
       *[other] { $epochs } Epochen, geprüft
    }
    .keyword = Chronik

unpack-holding-the-file = die Datei
    .keyword = hält

unpack-onion-key = der Schlüssel seiner Onion-Adresse, also kam die Adresse mit
    .keyword = verdeckt

unpack-unpacked = nach { $target },
    aus einer Datei, gepackt am { $packed }.
    Das ist jetzt der Knoten, und jene Datei auch: lösche { $file }
    .keyword = entpackt

unpack-next = { $serve }
    .keyword = weiter

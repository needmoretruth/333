### `333 pack`: writing this node into one file, to be carried to another machine.

pack-name-the-file = nenne die Datei, in die dieser Knoten gepackt wird: 333 pack <FILE>

pack-no-node = in { $root } gibt es keinen Knoten zu packen. Nichts wurde geschrieben.

pack-already-exists = { $file } existiert schon. { -pack-never-over }
-pack-never-over = Packen schreibt eine neue Datei, nie über eine alte; nenne eine andere.

pack-not-marked = { $file } wurde geschrieben, aber das Verzeichnis nicht als gepackt markiert. { -pack-until-it-is }
-pack-until-it-is = Bis dahin lebt dieser Knoten in beiden: { -pack-delete-that-file }
-pack-delete-that-file = lösche die Datei, bevor hier etwas läuft.

pack-creating = lege { $file } an

pack-name = { $name }
    .keyword = Name

pack-record-none = noch keine
    .keyword = Chronik

pack-record = { $epochs ->
        [one] { $epochs } Epoche, geht mit
       *[other] { $epochs } Epochen, gehen mit
    }
    .keyword = Chronik

pack-witnessed = { $statements ->
        [one] { $statements } Aussage, die ein anderer Schlüssel über ihn signiert hat,
            geht mit
       *[other] { $statements } Aussagen, die andere Schlüssel über ihn signiert haben,
            gehen mit
    }
    .keyword = Zeuge

pack-holding = die Datei, geht mit
    .keyword = hält

pack-onion-key = der Schlüssel seiner Onion-Adresse, also geht die Adresse mit
    .keyword = verdeckt

pack-carrying = diesen Knoten, in { $file }.
    Diese Datei IST dieser Knoten: wer sie hat, kann als dieser Name
    antworten. Trag sie hin, entpacke sie, dann lösche sie; sie ist
    keine Sicherung zum Aufheben. Sie ist nicht verschlüsselt, denn ein
    Passwort wäre eine Sache mehr zum Verlieren, und es zu verlieren,
    verlöre den Namen so sicher wie die Datei. Nur du kannst sie lesen,
    wie dieses Verzeichnis.
    .keyword = trägt

pack-packed = { $bytes } Bytes.
    nichts in { $root } wird wieder als dieser Knoten handeln.
    .keyword = gepackt

pack-next = auf dem anderen Rechner: 333 unpack { $carried }
    wenn der Umzug abgebrochen wird: { $undo }
    .keyword = weiter

pack-not-packed = dieser Knoten wurde nicht gepackt; in { $root } gibt es nichts zurückzunehmen
    .keyword = hier

pack-restored = dieser Knoten lebt wieder in { $root }.
    Die Datei, in die er gepackt wurde, ist noch dieser Name. Wurde sie
    irgendwo entpackt, muss einer der beiden weg, bevor einer läuft;
    wenn nicht, lösche { $file }
    .keyword = zurück

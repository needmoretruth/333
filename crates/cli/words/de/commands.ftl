### What the shared parts of the commands say.

commands-clock-at-zero = { $epoch }. Die Uhr dieses Rechners sagt, es sei 1970, also glaubt
    dieser Knoten, er stehe am Anfang der Zeit. Niemand wird ihm etwas
    übergeben oder ihn bezeugen, bis die Uhr gestellt ist.
    .keyword = Epoche

commands-called-first = der erste erzeugte Schlüssel wurde gewählt.
    .keyword = gewählt

commands-called = { $not_called ->
        [one] { $not_called } Schlüssel wurde erzeugt und nicht gewählt. dieser schon.
       *[other] { $not_called } Schlüssel wurden erzeugt und nicht gewählt. dieser schon.
    }
    .keyword = gewählt

commands-torn = { $bytes } Bytes eines unfertigen Eintrags wurden aus der Chronik entfernt
    .keyword = defekt

commands-record = { $epochs ->
        [one] { $epochs } Epoche schon beantwortet, keine mehr änderbar
       *[other] { $epochs } Epochen schon beantwortet, keine mehr änderbar
    }
    .keyword = Chronik

commands-witnessed = { $statements ->
        [one] { $statements } Aussage, die ein anderer Schlüssel über diesen Knoten
            signiert hat. Sie wird auch nach dem Ende ihrer Epoche
            aufbewahrt, weil sonst nichts von ihr das Fenster überdauert.
       *[other] { $statements } Aussagen, die andere Schlüssel über diesen Knoten
            signiert haben. Sie werden auch nach dem Ende ihrer Epochen
            aufbewahrt, weil sonst nichts von ihnen das Fenster überdauert.
    }
    .keyword = Zeuge

commands-unseen = über diesen Knoten wurde nie etwas signiert, in keiner Epoche.
    Hinausgehen klappt, erreicht werden nicht, und nur das Zweite zählt:
    wer ausgelost wird zu fragen, muss ankommen. Zwei Dinge verursachen
    das: ein Router, der Port 3333 nicht an diesen Rechner schickt, und
    eine Adresse, die niemand bekommen hat. `run --tor` braucht keins
    von beiden: eine Onion-Adresse ist hinter jedem Router erreichbar,
    und dieser Client bringt Tor schon mit.
    .keyword = verdeckt

commands-roll-alone = 1 von uns, nämlich dieser Knoten
    .keyword = Liste

commands-roll = { $members } von uns
    .keyword = Liste

commands-known = wo { $addresses } von uns zu suchen sagten
    .keyword = bekannt

commands-holding = die Datei, und kann sie weitergeben
    .keyword = hält

commands-keeping = alles, für immer. Es bringt diesem Knoten nichts: jede Aussage
    trägt ihre eigene Signatur und prüft sich überall gleich, wo sie
    aufbewahrt wird. Es gibt kein amtliches Archiv und keinen Archivar.
    .keyword = bewahrt

commands-ignored = { $admissions } Aufnahmen, die nicht lesbar waren
    .keyword = ungültig

commands-learned-where = wo { $addresses } weitere von uns sind
    .keyword = gelernt

commands-rejoined = { $members } weitere von uns mit Namen, von einem Knoten, der { $were }
    kannte. Wir waren zwei Zählungen, jetzt ist es eine.
    .keyword = vereint

commands-learned-names = { $members } weitere von uns mit Namen
    .keyword = gelernt

commands-heard = { $speakers } von uns sprechen
    .keyword = gehört

commands-carried = { $statements ->
        [one] { $statements } Aussage über eine noch offene Epoche
       *[other] { $statements } Aussagen über noch offene Epochen
    }
    .keyword = getragen

commands-exchange = { $node }  Epoche { $epoch }  { $clocks }  ({ $liveness })
    .keyword = Zeuge

commands-answered-the-challenge = hat die Aufgabe beantwortet, die wir gewählt haben
commands-spoke-first = hat zuerst gesprochen, was nur beweist, dass er sprach

commands-clocks-together = Uhren gleich
commands-clocks-ahead = ihre Uhr geht { $apart } vor
commands-clocks-behind = ihre Uhr geht { $apart } nach
commands-hours-and-minutes = { $hours } h { $minutes } min
commands-minutes-and-seconds = { $minutes } min { $seconds } s
commands-seconds = { $seconds } s

commands-waking = Tor. der verdeckte Weg braucht eine Weile, bis er offen ist.
    .keyword = weckt

commands-waking-through = Tor, über { $bridges ->
        [one] { $bridges } Brücke
       *[other] { $bridges } Brücken
    }. der verdeckte Weg braucht eine Weile, bis er offen ist.
    .keyword = weckt

commands-no-tor = keine Tor-Verbindung nach { $seconds } s
commands-starting-tor = starte den Tor-Client

# What a handover puts a signature under, read back.
commands-signed-giving = du sagtest: ich habe dir die Datei in Epoche { $epoch } übergeben.
    sie sagten: ich habe die Datei in Epoche { $epoch } von dir bekommen.
    es steht in zwei Handschriften, und keine Hand kann es zurücknehmen.
    .keyword = signiert

commands-signed-taking = sie sagten: ich habe dir die Datei in Epoche { $epoch } übergeben.
    du sagtest: ich habe die Datei in Epoche { $epoch } von dir bekommen.
    es steht in zwei Handschriften, und keine Hand kann es zurücknehmen.
    .keyword = signiert

commands-brimming = { $statements ->
        [one] { $statements } Aussage passte nicht in eine Runde und wartet auf die nächste
       *[other] { $statements } Aussagen passten nicht in eine Runde und warten auf die nächste
    }
    .keyword = voll

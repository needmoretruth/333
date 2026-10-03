### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph.

help-about = Ein Knoten von 333. Er antwortet, wenn er gefragt wird, führt seine
    Chronik und gibt die Datei weiter.

help-id = Zeigt den Namen dieses Knotens und erzeugt beim ersten Mal einen

help-bootstrap = Beginnt eine neue Linie, wenn niemand dir die Datei geben kann
help-bootstrap-long = Beginnt eine neue Linie, wenn niemand dir die Datei geben kann.

    Der gewöhnliche Weg hinein ist `333 join` mit einer Einladung. Dies
    schaut zuerst beim Treffpunkt nach und lehnt ab, wenn dort jemand
    ist. Ist niemand da, holt es die Datei, prüft sie gegen den Hash,
    den dieser Client mitbringt, und schreibt sie auf. Dein Knoten ist
    dann der Gründer seiner eigenen Linie, ohne jemandes Unterschrift an
    seinem Anfang, und jeder, der seine Chronik liest, kann das sehen.

help-serve = Lässt diesen Knoten in diesem Terminal laufen, bis du ihn stoppst
help-serve-long = Lässt diesen Knoten in diesem Terminal laufen, bis du ihn stoppst.

    Er beantwortet Herzschläge und Fragen, tauscht, was er weiß, und
    fragt in jeder Epoche die Knoten, die er ausgelost wurde zu fragen.
    In einem Terminal öffnet er den Bildschirm; `q`, Strg-C oder
    `333 stop` aus einem anderen Terminal stoppt ihn. `333 start` macht
    dasselbe im Hintergrund.

help-serve-long-light = Lässt diesen Knoten in diesem Terminal laufen, bis du ihn stoppst.

    Er beantwortet Herzschläge und Fragen, tauscht, was er weiß, und
    fragt in jeder Epoche die Knoten, die er ausgelost wurde zu fragen,
    Zeile für Zeile. Strg-C oder `333 stop` aus einem anderen Terminal
    stoppt ihn. `333 start` macht dasselbe im Hintergrund.

help-say = Sagt eine der 333, einmal pro Epoche. Was reist, ist die Nummer

help-status = Zeigt, ob dieser Knoten läuft, wo andere ihn erreichen und wie viele
    von uns antworten

help-join = Empfängt die Datei von einem Knoten, der sie hat, mit einer Einladung

help-languages = Listet Sprachen oder speichert eine für alle Befehle dieses Knotens

help-ping = Erreicht einen anderen Knoten und tauscht einen Herzschlag mit ihm

help-pack = Schreibt diesen Knoten in eine Datei, für einen anderen Rechner
help-pack-long = Schreibt diesen Knoten in eine Datei, für einen anderen Rechner.

    Alles kommt mit: sein Name, seine Chronik, was andere über ihn
    signiert haben, die Datei und der Schlüssel seiner Onion-Adresse.
    Danach weigert sich dieses Verzeichnis, ihn laufen zu lassen, damit
    der Name nie an zwei Orten ist. Die Datei ist nicht verschlüsselt:
    wer sie hat, ist dieser Knoten. Trag sie hin, entpacke sie, lösche sie.

help-unpack = Legt einen gepackten Knoten in das Knotenverzeichnis dieses Rechners
help-unpack-long = Legt einen gepackten Knoten in das Knotenverzeichnis dieses Rechners.

    Wird abgelehnt, wo schon ein Knoten lebt. Nichts wird geschrieben,
    bevor die Datei ganz gelesen und ihr Schlüssel und ihre Chronik
    geprüft sind.

help-moved = Meldet: das Verzeichnis wurde verschoben oder umbenannt, nicht kopiert
help-moved-long = Meldet: das Verzeichnis wurde verschoben oder umbenannt, nicht kopiert.

    Ein Knoten, der sich an einem neuen Ort findet, sagt das bei jedem
    Start, bis dies eingegeben wird, denn eine Kopie, deren Original
    noch läuft, wäre ein Name an zwei Orten.

help-tell = Gibt einem laufenden Knoten einen Befehl in Worten seines Bildschirms
help-tell-long = Gibt einem laufenden Knoten einen Befehl in Worten seines Bildschirms.

    `tor on`, `tor off`, `bridge <line>`, `helper <program>` und jedes
    andere Wort, das der Bildschirm nach `:` annimmt. Der laufende Knoten
    führt ihn aus, und seine Antwort steht hier. `say`, `join`, `ping`,
    `begin`, `status` und `stop` erreichen einen laufenden Knoten auch
    ohne dies.

help-tell-light = Gibt einem laufenden Knoten einen Befehl
help-tell-long-light = Gibt einem laufenden Knoten einen Befehl.

    `tor on`, `tor off`, `bridge <line>` und `helper <program>`. Der
    laufende Knoten führt ihn aus, und seine Antwort steht hier. `say`,
    `join`, `ping`, `begin`, `status` und `stop` erreichen einen
    laufenden Knoten auch ohne dies.

help-service = Verwaltet den Hintergrunddienst direkt (`start` und `stop` nutzen ihn)
help-service-long = Verwaltet den Hintergrunddienst direkt (`start` und `stop` nutzen ihn).

    Nichts wird installiert, bevor du es verlangst, jede geschriebene
    Datei und jeder ausgeführte Befehl wird dabei angezeigt, und
    `333 service uninstall` entfernt alles.

help-service-install = Installiert den Hintergrunddienst mit diesen Optionen und startet ihn
help-service-install-long = Installiert den Hintergrunddienst mit diesen Optionen und startet ihn.

    Der Dienst führt `333 run` mit genau den angegebenen Optionen für das
    Verzeichnis dieses Knotens aus, und eine stündliche Prüfung meldet
    es auf diesem Rechner, wenn der Knoten stoppt. `333 start` macht
    dasselbe ohne Optionen.

help-service-uninstall = Stoppt den Hintergrunddienst und entfernt alles, was er installiert hat

help-service-status = Was der Dienstverwalter sagt, wann der Knoten zuletzt sagte, er sei
    wach, und seine letzten Zeilen

help-service-check = Meldet auf diesem Rechner, wenn der Knoten gestoppt ist. Der Dienst
    führt das stündlich aus; wenn alles gut ist, sagt es nichts

help-data-dir = Verzeichnis mit allem, was diesem Knoten gehört: sein Name, und der
    Zustand von Tor, wenn er Tor nutzt

help-timeout = Sekunden Wartezeit für jeden Schritt, der mit dem Netz spricht
help-timeout-long = Sekunden Wartezeit für jeden Schritt, der mit dem Netz spricht.

    Eine Obergrenze, keine Verzögerung. Sie ist für den Start von Tor
    bemessen, den einzigen Schritt, der Minuten dauern kann.

help-dangerously-trust-directory-permissions = Akzeptiert ein Verzeichnis, das andere Nutzer betreten können
help-dangerously-trust-directory-permissions-long = Akzeptiert ein Verzeichnis, das andere Nutzer betreten können.

    Das Verzeichnis hält die einzige Kopie des Namens dieses Knotens,
    also wird ein locker berechtigtes standardmäßig abgelehnt. Das ist
    für Testverzeichnisse und Container mit seltsamen Besitzern.

help-keep-everything = Bewahrt jede Aussage für immer auf statt nur das Fenster, nach dem
    geurteilt wird
help-keep-everything-long = Bewahrt jede Aussage für immer auf statt nur das Fenster, nach dem
    geurteilt wird.

    Es ändert nichts an irgendjemandes Stand: jede Aussage prüft sich
    überall gleich, wo sie aufbewahrt wird.

help-bridges = Eine Brückenzeile, für ein Netz, das den normalen Weg zu Tor sperrt
help-bridges-long = Eine Brückenzeile, für ein Netz, das den normalen Weg zu Tor sperrt.

    Gib sie einmal für jede Brücke an, die du bekommen hast, genau so,
    wie du sie bekommen hast. Hier holt nichts Brücken: Menschen geben
    sie absichtlich weiter, damit keine Liste einfach gesammelt und
    gesperrt werden kann.

help-bridge-helper = Programm für eine verschleierte Brücke, per Name oder Pfad
help-bridge-helper-long = Programm für eine verschleierte Brücke, per Name oder Pfad.

    Nur nötig, wenn eine Brückenzeile eines verlangt und es nicht
    `lyrebird` im Pfad ist. Es liegt nicht bei, weil eine eingefrorene
    Kopie bald die falsche wäre.

help-language = Die Sprache, als Kürzel: `ko`, `es`, `zh-Hant`
help-language-long = Die Sprache, als Kürzel: `ko`, `es`, `zh-Hant`.

    Ohne sie `THE333_LANGUAGE`, dann die von `333 language <TAG>`
    gespeicherte Sprache, dann Englisch. Die Sprache des Systems wird
    nicht genutzt. `333 language` listet die Sprachen, für die es Worte
    gibt, und ein Ordner mit Katalogen in `<data-dir>/words/<tag>/`
    fügt eine hinzu, ohne etwas zu bauen. Die 333 Worte selbst werden
    nie übersetzt.

help-count-in = Zählt in zehn, zwölf oder twelve-ascii
help-count-in-long = Zählt in zehn, zwölf oder twelve-ascii.

    Jede angezeigte Zahl wird darin geschrieben und jede eingetippte
    darin gelesen: `say 238` in zwölf ist `say 332` in zehn. Namen,
    Adressen, Ports und Versionen werden nie umgerechnet, und auf der
    Leitung ändert sich nichts. Ohne sie `THE333_COUNT_IN`, dann zehn.

help-bootstrap-meet = Wo nach Leuten gesucht wird, bevor du allein beginnst

help-bootstrap-anyway = Beginnt, obwohl schon jemand da ist

help-serve-bind = Adresse und Port, auf denen gelauscht wird

help-serve-tor = Öffnet auch eine Onion-Adresse, damit andere diesen Knoten erreichen,
    ohne zu erfahren, wo er ist. Tor zu wecken dauert Sekunden bis Minuten

help-serve-no-direct = Öffnet gar keinen Socket. Nur mit --tor; deine Adresse bleibt ganz
    aus der Leitung

help-serve-announce = Die Adresse, unter der andere Knoten diesen erreichen sollen
help-serve-announce-long = Die Adresse, unter der andere Knoten diesen erreichen sollen.

    Nötig, wenn der Socket sie nicht sagen kann: beim Lauschen auf allen
    Schnittstellen oder hinter etwas, das einen Port weiterleitet.

help-serve-no-mdns = Sagt im lokalen Netz nicht, dass dieser Knoten hier ist
help-serve-no-mdns-long = Sagt im lokalen Netz nicht, dass dieser Knoten hier ist.

    Sonst geht hinaus, dass etwas auf diesem Rechner 333 spricht und auf
    welchem Port, nicht der Name dieses Knotens. So finden sich zwei
    Knoten in einem Haus ohne Einladung.

help-serve-no-router = Bittet den Router nicht, den Port an diesen Rechner zu schicken
help-serve-no-router-long = Bittet den Router nicht, den Port an diesen Rechner zu schicken.

    Ein Heimrouter verwirft, was niemand drinnen angefragt hat, bis ein
    Programm drinnen ihn bittet, einen Port weiterzuleiten, über
    UPnP-IGD, PCP oder NAT-PMP. Das ändert das Netz, also wird es
    angezeigt, wenn es passiert. `--no-upnp` ist der ältere Name dafür.

help-serve-meet = Wo nach Knoten gesucht wird, die diesem niemand vorgestellt hat
help-serve-meet-long = Wo nach Knoten gesucht wird, die diesem niemand vorgestellt hat.

    Eine feste Adresse, die signierte Aussagen darüber hält, wo Knoten
    sind. Alles, was dort gelesen wird, wird hier geprüft.

help-serve-no-meet = Nutzt gar keinen Treffpunkt
help-serve-no-meet-long = Nutzt gar keinen Treffpunkt.

    Dieser Knoten ist dann nur für die erreichbar, die eine Einladung
    bekommen haben, und für Knoten in diesem Netz, sonst für niemanden.

help-serve-plain = Sagt die Zeilen, statt den Bildschirm zu zeichnen
help-serve-plain-long = Sagt die Zeilen, statt den Bildschirm zu zeichnen.

    Außerhalb eines Terminals sagt er immer die Zeilen; dies verlangt
    das auch in einem Terminal.

help-serve-plain-light = Sagt die Zeilen, was diese Ausgabe immer tut
help-serve-plain-long-light = Sagt die Zeilen, was diese Ausgabe immer tut.

    Diese Ausgabe hat keinen Bildschirm. Die Option wird angenommen,
    damit eine Befehlszeile in beiden Ausgaben funktioniert.

help-say-index = Welche davon, von 0 bis { $last }, in der Basis getippt, in der
    gezählt wird (--count-in). Die Worte sind noch nicht geschrieben

help-status-sources = Listet jede Adresse, die dieser Knoten hält: wem sie gehört, wo und
    wann zuerst von ihr gehört wurde, und wo zuletzt

help-status-json = Was dieser Knoten beobachtet hat, als JSON für ein Programm. Keine
    Adresse und kein Port stehen darin

help-join-address = Eine Einladung (`333:host:port`) von jemandem, der sie schon hat

help-ping-address = Eine Einladung (`333:host:port`) oder eine Adresse: `host`,
    `host:port`, `[::1]:port` oder `etwas.onion` (über Tor erreicht)

help-pack-file = Die zu schreibende Datei. Sie darf noch nicht existieren

help-pack-undo = Nimmt hier ein Packen zurück, für einen abgebrochenen Umzug
help-pack-undo-long = Nimmt hier ein Packen zurück, für einen abgebrochenen Umzug.

    Nur wenn die Datei nirgends entpackt wurde: sonst macht das zwei.

help-unpack-file = Die Datei, die `333 pack` geschrieben hat

help-tell-order = Der Befehl, so wie er in den Bildschirm getippt würde

help-tell-order-light = Der Befehl, so geschrieben wie `tor on` oder `bridge <line>`

help-service-install-flags = Die Optionen für `run`, so wie du sie danach tippen würdest

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = Zeigt die Hilfe
help-print-help-more = Zeigt die Hilfe (mehr mit '--help')
help-print-help-summary = Zeigt die Hilfe (Übersicht mit '-h')
help-print-version = Zeigt die Version
help-print-this = Zeigt diese Nachricht oder die Hilfe der angegebenen Unterbefehle
help-print-for = Zeigt die Hilfe der Unterbefehle

help-start = Startet diesen Knoten im Hintergrund, jetzt und nach jedem Neustart

help-stop = Stoppt diesen Knoten und lässt ihn nach einem Neustart gestoppt

help-restart = Stoppt diesen Knoten und lässt ihn wieder im Hintergrund laufen

help-logs = Zeigt die letzten Zeilen, die dieser Knoten im Hintergrund schrieb

help-logs-follow = Zeigt weiter neue Zeilen, sobald sie kommen, wo systemd sie hält

help-invite = Zeigt die Einladung, mit der andere über diesen Knoten beitreten

help-status-all = Zeigt alles, was dieser Knoten weiß, mit der Bedeutung jedes Teils

help-languages-tag = Die zu speichernde Sprache als Kürzel: `ko`, `en`. `en` heißt Englisch

help-start-example = Beispiel: 333 start

help-stop-example = Beispiel: 333 stop

help-restart-example = Beispiel: 333 restart

help-status-example = Beispiel: 333 status --all

help-logs-example = Beispiel: 333 logs -f

help-id-example = Beispiel: 333 name

help-invite-example = Beispiel: 333 invite

help-bootstrap-example = Beispiel: 333 begin

help-serve-example = Beispiel: 333 run --tor

help-say-example = Beispiel: 333 say 7

help-join-example = Beispiel: 333 join 333:192.0.2.7:3333

help-languages-example = Beispiel: 333 language de

help-ping-example = Beispiel: 333 ping 333:192.0.2.7:3333

help-pack-example = Beispiel: 333 pack node.333

help-unpack-example = Beispiel: 333 unpack node.333

help-moved-example = Beispiel: 333 moved

help-tell-example = Beispiel: 333 tell tor on

help-service-example = Beispiel: 333 service status

help-service-install-example = Beispiel: 333 service install --tor

help-service-uninstall-example = Beispiel: 333 service uninstall

help-service-status-example = Beispiel: 333 service status

help-service-check-example = Beispiel: 333 service check

help-start-flags = Die Optionen für `run`, so wie du sie danach tippen würdest. Sie
    gelten für jeden späteren Start, bis andere angegeben werden

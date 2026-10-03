### `333 service`: running the node through logouts and reboots, when asked to.

service-mind = { $node } liegt an einem Ort, den dieses System leert, und der Name
    dieses Knotens ist nirgends sonst. Der Dienst lässt ihn dort laufen,
    bis er geleert wird.
    .keyword = Achtung

service-runs = { $command }
    .keyword = Knoten

service-undo-partial = `333 service uninstall` entfernt, was davon getan wurde.
    .keyword = zurück

service-no-receipt-directory = dieses System nennt kein Konfigurationsverzeichnis für den Beleg

service-wrote-receipt = { $path }, woran `333 service uninstall` erkennt, was rückgängig zu machen ist.
    .keyword = schrieb

service-undo = `333 service uninstall` stoppt den Knoten und macht all das rückgängig.
    Das eigene Verzeichnis des Knotens berührt keins von beiden.
    .keyword = zurück

service-uninstalled = wird nicht mehr von einem Dienst ausgeführt. { $node } bleibt, wie der
    Knoten es ließ: `333 run` lässt ihn von Hand laufen, und `333 start`
    richtet den Dienst wieder ein.
    .keyword = Knoten

service-none-installed = für diesen Nutzer wurde keiner von `333 service install` installiert.
    .keyword = Dienst

service-state = { $state }
    .keyword = Dienst

service-node = { $node }
    .keyword = Knoten

service-last-awake = sagte es zuletzt um { $at }, vor { $ago }
    .keyword = wach

service-never-awake = sagte es nie, in diesem Verzeichnis
    .keyword = wach

service-said-nothing = nichts, was aufbewahrt wurde
    .keyword = sagte

service-said-last = { $lines ->
        [one] die letzte Zeile:
       *[other] die letzten { $lines } Zeilen:
    }
    .keyword = sagte

service-no-manager = dieses System hat keinen Dienstverwalter, den `333 service` fragen
    kann. `333 run --plain` lässt den Knoten unter dem laufen, was hier
    Programme am Laufen hält.

service-not-installed-here = nicht installiert: hier gibt es keinen bekannten Dienstverwalter

# Said by every service manager's own file.

service-creating = lege { $path } an
service-writing = schreibe { $path }
service-removing = entferne { $path }

service-wrote = { $path }
    .keyword = schrieb

service-removed = { $path }
    .keyword = entfernt

service-left = { $path }. `333 service install` hat das nicht geschrieben.
    .keyword = gelassen

service-failed = { $why }
    .keyword = Fehler

service-not-installed = nicht installiert
service-running = läuft
service-starting = startet

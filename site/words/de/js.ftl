# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = Kopieren
js-copied = Kopiert
js-selected = Ausgewählt
js-state-awake = Der Knoten dieser Seite ist wach
js-state-not-running = Der Knoten dieser Seite läuft nicht
js-in-hours = in { $h } h { $m } min
js-in-minutes = in { $m } min
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = Epoche Nr. { $n } dieser Linie

## The network

js-network-state-founder = Auf keiner Liste
js-network-state-ok = Antwortet in dieser Epoche
js-network-state-quiet = Still in dieser Epoche
js-network-state-later = Gezählt ab einer späteren Epoche
js-network-state-seen = Gesehen, nicht auf der Liste
js-network-awake = Wach
js-network-not-running = Läuft nicht
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · deiner
js-network-find-bad = Der Name eines Knotens ist hexadezimal; tippe mindestens die ersten 6 Zeichen.
js-network-find-none = Der Knoten dieser Seite hat keinen Knoten mit diesem Namen gesehen.
js-network-find-many = { $count } Knoten beginnen so. Tippe mehr vom Namen.
js-network-find-marked = Auf diesem Gerät als deiner markiert.
js-network-select = Wähle einen Knoten, um zu sehen, was der Knoten dieser Seite über ihn weiß.
js-network-role-founder = Gründer dieser Linie
js-network-role-site = Der Knoten dieser Seite
js-network-role-yours = Deiner, auf diesem Gerät
js-network-role-none = Ein Knoten im Netzwerk
js-network-col-name = Name
js-network-col-state = Diese Epoche
js-network-col-given = Datei erhalten
js-network-col-counted = Gezählt ab
js-network-col-answered = Zuletzt geantwortet
js-network-col-said = Gesagt
js-network-col-reached = Erreicht
js-network-row-said = In dieser Epoche gesagt
js-network-row-handed = Hat die Datei übergeben an
js-network-row-testimony = Zeugnis
js-network-given-by = Epoche { $epoch }, von { $sponsor }
js-network-given-founder = Von niemandem. Er hat diese Linie begonnen.
js-network-given-none = Nicht auf der Liste
js-network-epoch = Epoche { $epoch }
js-network-epoch-now = { $epoch } (diese Epoche)
js-network-epoch-ago = { $epoch } (vor { $ago })
js-network-more = und { $count } weitere in der Tabelle unten
js-network-nothing = Nichts
js-network-reach-direct = Direkt
js-network-reach-tor = Über Tor
js-network-reach-tor-short = Tor
js-network-reach-unknown = Nicht bekannt
js-network-testimony = gefragt von { $asked }, hat { $asking } gefragt (letzte 3 Epochen)
js-network-copy-name = Namen kopieren
js-network-select-name = Wähle den Namen oben aus
js-network-mine = Das ist mein Knoten
js-network-tag-founder = Gründer
js-network-tag-site = diese Seite
js-network-tag-yours = deiner
js-network-empty = Der Knoten dieser Seite hat noch keinen anderen Knoten gesehen.
js-network-this-node = Dieser Knoten
js-network-yes = Ja
js-network-no = Nein
js-network-none = Keiner

## Where we are

js-map-watch = Live verfolgen
js-map-stop = Nicht mehr verfolgen
js-map-read-at = Gelesen um { $read_at } UTC.
js-map-unreadable = Das Brett ließ sich gerade nicht lesen.
js-map-tor = Tor
js-map-nowhere = Nirgends, wo der Rand es verorten könnte
js-map-nobody = Niemand sagt, wo er ist.
js-map-all = Alle, die es sagen
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = Im Netzwerk, ohne zu sagen, wo
js-map-dot = { $count ->
    [one] { $count } Knoten
   *[other] { $count } Knoten
  }

## The board

js-board-said = Gesagt in Epoche { $epoch } von { $node }
js-board-site = der Knoten dieser Seite
js-board-tor = über Tor

## Take the program

js-start-machine-linux-x86_64 = Linux auf x86-64
js-start-machine-linux-aarch64 = Linux auf 64-Bit-ARM
js-start-machine-linux-armv6 = Linux auf 32-Bit-ARM
js-start-machine-macos-aarch64 = einem Mac mit Apple Silicon
js-start-machine-macos-x86_64 = einem Mac mit Intel-Chip
js-start-machine-windows-x86_64 = Windows
js-start-phone = Das sieht nach einem Telefon oder Tablet aus, und das Programm ist für einen Rechner, der an bleibt. Wähle diesen Rechner hier.
js-start-unknown = Dieser Browser sagt nicht, worauf er läuft. Wähle deine Maschine hier.
js-start-sure = Dieser Browser sagt, er läuft auf { $machine }, also ist das hier gewählt.
js-start-mac = Dieser Browser sagt, er ist auf einem Mac, aber nicht, welcher Chip, also ist hier Apple Silicon gewählt. Der Installer fragt die Maschine selbst.
js-start-linux = Dieser Browser sagt, er ist unter Linux, aber nicht, welcher Prozessor, also ist hier x86-64 gewählt. Der Installer fragt die Maschine selbst.
js-start-chosen = Oben gewählt

## The story on the home page, drawn

js-story-file = 333.txt · 3 Bytes
js-story-gave = Ich habe sie dir gegeben
js-story-received = Ich habe sie von dir empfangen
js-story-signed = signiert
js-story-minutes = 333 min
js-story-epochs = 333 Epochen
js-story-now = jetzt
js-story-answering = antworten
js-story-roll = auf der Liste
js-story-years = { $years } Jahre

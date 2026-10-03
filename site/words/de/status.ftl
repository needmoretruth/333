# Status: the network now, how it has been one epoch at a time, and this site's machine.

status-meta-title = Status · 333
status-meta-description = Wie es dem 333-Netzwerk jetzt geht und wie es ihm ging, Epoche für Epoche: Knoten auf der Liste, Knoten, die antworten, Aussagen auf dem Brett und ob der Knoten und die Maschine dieser Seite weitergelaufen sind.
status-heading = Status
status-lede = Der Knoten dieser Seite sieht alle 15 Sekunden ins Netzwerk und schreibt die Zahlen einmal pro Epoche auf.
status-now-title = Jetzt
status-roll = Auf der Liste, Gründer eingeschlossen
status-saying = Sagen, wo sie sind
status-tor = Davon über Tor
status-site-node = Der Knoten dieser Seite
status-time-title = Im Verlauf
status-time-lede = Ein Messpunkt pro Epoche: die letzten Zahlen, die der Knoten dieser Seite vor dem Ende der Epoche lieferte. Eine Lücke in einer Linie ist eine Epoche, die niemand aufgeschrieben hat.
status-chart-recent-title = Die letzten 333 Epochen
status-chart-all-title = Alles Aufgeschriebene
# Under each chart, the same numbers as text. Every $variable is a number or a date
# (YYYY-MM-DD, UTC), or a dash when there is none.
status-chart-summary = Epochen { $first } bis { $last }, vom { $from } bis { $to }. Auf der Liste: niedrigster Wert { $roll_low }, höchster { $roll_high }, letzter { $roll_latest }. Antworten: niedrigster Wert { $answering_low }, höchster { $answering_high }, letzter { $answering_latest }.
status-chart-too-few = Für diesen Abschnitt sind weniger als zwei Epochen aufgeschrieben, also gibt es noch keine Linie zu zeichnen.
status-machine-title = Die Maschine dieser Seite
status-release = Release
status-deployed = Ausgerollt
status-observed = Letzter Blick auf den Knoten
status-age = { $seconds ->
    [one] vor 1 Sekunde.
   *[other] vor { $seconds } Sekunden.
}
status-observed-running = Er lief.
status-observed-not-running = Er lief nicht.
status-uptime = Maschine läuft seit
status-uptime-value = { $days ->
    [one] 1 Tag
   *[other] { $days } Tagen
}, { $hours ->
    [one] 1 Stunde
   *[other] { $hours } Stunden
}
status-elsewhere = Alle Knoten stehen auf <a href="{ $base }/network">der Netzwerkseite</a>, und wo sie sind, auf <a href="{ $base }/map">der Karte</a>.
status-json = Dieselben Zahlen für ein Programm: <a href="/api/status">/api/status</a>.

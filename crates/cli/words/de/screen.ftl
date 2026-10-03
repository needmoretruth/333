### The screen: what it says in the log pane about what was typed into it.

screen-asked = { $typed }
    .keyword = gefragt
screen-unheard = nichts führt mehr Befehle aus
    .keyword = ungehört
screen-unread = { $why }
    .keyword = unlesbar
screen-refused = { $why }
    .keyword = Absage

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = eine Taste war nicht lesbar: { $why }. Die folgenden vielleicht schon.
    .keyword = Tastatur
screen-keyboard-gone = die Tastatur war nicht lesbar ({ $why }), also hat sich der
    Bildschirm geschlossen und der Knoten mit ihm. `333 run --plain`
    lässt den Knoten ohne Tastatur laufen.

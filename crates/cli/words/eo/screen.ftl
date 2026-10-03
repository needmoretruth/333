### The screen: what it says in the log pane about what was typed into it.

screen-asked = { $typed }
    .keyword = petita
screen-unheard = nenio plu plenumas ordonojn
    .keyword = neaŭdita
screen-unread = { $why }
    .keyword = nelegita
screen-refused = { $why }
    .keyword = rifuzita

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = klavo ne legeblis: { $why }. La klavoj post ĝi eble legeblos.
    .keyword = klavaro
screen-keyboard-gone = la klavaro ne legeblis ({ $why }), do la ekrano fermiĝis kaj la nodo
    kun ĝi. `333 run --plain` funkciigas la nodon sen klavaro.

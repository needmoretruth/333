### The screen: what it says in the log pane about what was typed into it.

screen-asked = { $typed }
    .keyword = demandé
screen-unheard = plus rien n’exécute les ordres
    .keyword = inouï
screen-unread = { $why }
    .keyword = non lu
screen-refused = { $why }
    .keyword = refusé

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = une touche est illisible : { $why }. Les suivantes le seront peut-être.
    .keyword = clavier
screen-keyboard-gone = le clavier est illisible ({ $why }), l’écran s’est donc fermé, et le
    nœud avec lui. `333 run --plain` lance le nœud sans clavier.

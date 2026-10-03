### The screen: what it says in the log pane about what was typed into it.

screen-asked = { $typed }
    .keyword = asked
screen-unheard = nothing is carrying orders out any more
    .keyword = unheard
screen-unread = { $why }
    .keyword = unread
screen-refused = { $why }
    .keyword = refused

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = a key could not be read: { $why }. The keys after it may be.
    .keyword = keyboard
screen-keyboard-gone = the keyboard could not be read ({ $why }), so the screen has closed and
    the node with it. `333 run --plain` runs the node with no keyboard.

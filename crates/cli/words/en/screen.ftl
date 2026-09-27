### The screen: what it says in the vigil's pane about what was typed into it.

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
    the vigil with it. `333 serve --plain` keeps a vigil with no keyboard.

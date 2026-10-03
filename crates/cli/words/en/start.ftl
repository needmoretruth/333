### `333 start` and `333 restart`: running this node in the background.

start-no-node = there is no node in { $home } yet

start-no-node-next = `333 join <invitation>` joins with an invitation from someone who
    runs 333. `333 begin` starts on your own.

start-in-a-terminal = already, in a terminal. Stop it there, or with `333 stop`, and
    then `333 start` runs it in the background.
    .keyword = running

start-already = already, in the background.
    .keyword = running

start-started = in the background, now and after every reboot. `333 status` shows
    how it is doing; `333 stop` stops it.
    .keyword = started

start-elsewhere = the background service on this machine runs the node in { $other }.
    `333 service uninstall` removes it, and then `333 start` again.

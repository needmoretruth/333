### `333 stop`: stopping this node, and keeping it stopped after a reboot.

stop-stopped = stopped. It stays stopped after a reboot. `333 start` runs it again.
    .keyword = stopped

stop-stopped-in-a-terminal = stopped the node that was running in a terminal.
    .keyword = stopped

stop-not-running = no, so there was nothing to stop.
    .keyword = running

stop-refused = the node running in a terminal did not stop. Stop it there with
    Ctrl-C, or `q` in its screen.

stop-still-running = asked the node to stop, and it is still running after
    { $seconds } seconds.

stop-cannot-ask-here = it is running in a terminal, and on this system it cannot be
    asked to stop from another one. Stop it there with Ctrl-C.

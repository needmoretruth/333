# Status: the network now, how it has been one epoch at a time, and this site's machine.

status-meta-title = Status · 333
status-meta-description = How the 333 network is doing now and how it has been, epoch by epoch: nodes on the roll, nodes answering, statements on the board, and whether the site's own node and machine kept running.
status-heading = Status
status-lede = This site's node looks at the network every 15 seconds and writes the numbers down once an epoch.
status-now-title = Now
status-roll = On the roll, founder included
status-saying = Saying where they are
status-tor = Of those, through Tor
status-site-node = This site's node
status-time-title = Over time
status-time-lede = One sample per epoch: the last numbers this site's node gave before the epoch ended. A gap in a line is an epoch nobody wrote down.
status-chart-recent-title = The last 333 epochs
status-chart-all-title = Everything written down
# Under each chart, the same numbers as text. Every $variable is a number or a date
# (YYYY-MM-DD, UTC), or a dash when there is none.
status-chart-summary = Epochs { $first } to { $last }, from { $from } to { $to }. On the roll: lowest { $roll_low }, highest { $roll_high }, latest { $roll_latest }. Answering: lowest { $answering_low }, highest { $answering_high }, latest { $answering_latest }.
status-chart-too-few = Fewer than two epochs are written down for this stretch, so there is no line to draw yet.
status-machine-title = This site's machine
status-release = Release
status-deployed = Deployed
status-observed = Last look at the node
status-age = { $seconds ->
    [one] 1 second ago.
   *[other] { $seconds } seconds ago.
}
status-observed-running = It was running.
status-observed-not-running = It was not running.
status-uptime = Machine up for
status-uptime-value = { $days ->
    [one] 1 day
   *[other] { $days } days
}, { $hours ->
    [one] 1 hour
   *[other] { $hours } hours
}
status-elsewhere = Every node is listed on <a href="{ $base }/network">the network page</a>, and where they are on <a href="{ $base }/map">the map</a>.
status-json = The same numbers for a program: <a href="/api/status">/api/status</a>.

### `333 ping`: one heartbeat with another node.

ping-name = { $name }
    .keyword = name

ping-knocking = { $address }
    .keyword = knocking

ping-writing-who-answered = writing down who answered
ping-knocking-on = knocking on { $address }
ping-unfinished = { $address } took the connection and did not finish the exchange within { $seconds } s
ping-exchanging = exchanging heartbeats

### `333 serve`: asking the router, keeping what it lent, and giving it back.

serve-reach-router-opened-upnp = the router says port { $port } { $on } now comes to this machine. It
    is listed there as `333` if you want to take it away again. Whether
    anything arrives is the next line.
    .keyword = opened

serve-reach-router-opened-upnp-for = the router says port { $port } { $on } now comes to this machine, for { $time }. It
    is listed there as `333` if you want to take it away again. Whether
    anything arrives is the next line.
    .keyword = opened

serve-reach-router-opened-lease = asked the router at { $router } over { $way } for port { $port } for { $asked_for }. It says port
    { $granted_port } { $on } now comes here, for { $granted }. This node asks again before that
    runs out and gives it back when the vigil ends; stopped any other
    way, the router drops it by itself when the time is up. Whether
    anything arrives is the next line.
    .keyword = opened

serve-reach-router-nobody-answered = no router here answered a request to open a port, over UPnP-IGD, PCP
    or NAT-PMP. That is ordinary: plenty have all three turned off, and a
    machine with an address of its own has nothing to ask. `--no-router`
    stops this node asking at all.
    .keyword = closed

serve-reach-router-refused = the router would not open port { $port }: { $why }
    .keyword = closed

serve-reach-router-let-go = the router let port { $port } go: it was not asked again in time, so nobody
    outside can reach this node on it now. Starting the vigil again
    asks again.
    .keyword = closed

serve-reach-router-given-back = port { $port } is given back to the router over { $way }; it no longer
    comes to this machine.
    .keyword = closed

serve-reach-router-not-taken-back = the router did not take port { $port } back ({ $why }). It drops it by
    itself within { $time }.
    .keyword = closed

serve-reach-router-moved = the router moved this node: port { $port } { $on } now comes here instead of
    port { $before_port } { $before_on }. An invitation naming the old one no longer arrives.
    .keyword = opened

serve-reach-router-not-kept = the router did not keep port { $port } when asked ({ $why }). It still has it
    for { $time }, and is asked again before then.
    .keyword = waiting

serve-reach-router-on-its-outside-address = on its outside address
serve-reach-router-on = on { $address }

serve-reach-router-one-second = one second
serve-reach-router-two-seconds = two seconds
serve-reach-router-seconds = { $count } seconds
serve-reach-router-one-minute = one minute
serve-reach-router-two-minutes = two minutes
serve-reach-router-minutes = { $count } minutes
serve-reach-router-one-hour = one hour
serve-reach-router-two-hours = two hours
serve-reach-router-hours = { $count } hours

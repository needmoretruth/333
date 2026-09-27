### `333 serve`: whether anybody outside can actually get in.

serve-reach-unanswered-at-the-end = the router had not answered when the vigil ended. Whatever it
    agreed to runs out by itself within { $time }.
    .keyword = closed

serve-reach-shut-behind-another = the router says this household is at { $seen }, which is not an
    address on the open internet: another router, or the provider's
    shared address, stands between it and everybody else, and
    nothing here can ask that one. `333 serve --tor` needs no router
    change at all.
    .keyword = shut

serve-reach-open = port { $port } reaches this machine from outside. This node knocked at
    { $outside } and answered itself, so that address is one you can
    hand to anybody.
    .keyword = open

serve-reach-invite = { $invitation }
    .keyword = invite

serve-reach-shut-somebody-else = something answered at { $outside } and it was not this node. That port
    on your address belongs to something else, so an invitation naming
    it would send people to the wrong machine.
    .keyword = shut

serve-reach-shut-unfinished = something at { $outside } took the connection and did not finish a
    heartbeat: { $why }. An invitation naming it is not one to hand out.
    .keyword = shut

serve-reach-shut-nothing = nothing answered at { $outside }, so as far as the outside world can
    tell this node is not listening. Either the router in front of it
    was never told to send port { $port } here, or it will not let a machine
    inside it dial its own outside address. `333 serve --tor` needs no
    router change at all and works from any network, including the
    ones that hand out no reachable address in the first place.
    .keyword = shut

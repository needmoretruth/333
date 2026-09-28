### `333 status`: where the others are, as far as this node knows, and where it
### heard of each of them.
##
## A line here ends in `{` and the next begins with `""}` (or with a variable)
## where the printed line is wider than a line of this file may be; the placeable
## spanning the two prints no line break.

status-known-another-copy = ANOTHER COPY OF THIS NAME

status-known-sighting = A statement signed with this node's key, which this node never {
    ""}made, says
    it is at { $address }, in epoch { $said_in }.
    It arrived { $arrived }, in epoch { $epoch }.

status-known-either = Either this directory was copied and the copy was started, or {
    ""}somebody else
    has the key. Two nodes on one name contradict each other in every {
    ""}epoch either
    is asked about. Stop one of them; `333 pack` is how a node moves. {
    ""}Nothing here
    stops either copy for you: an old statement can be replayed by {
    ""}anybody, and a
    node that stopped on seeing one could be switched off by whoever {
    ""}holds a copy
    of its key.

status-known-nowhere = nowhere to knock yet. An invitation given to `333 ping` or
    `333 join` is kept, and the vigil knocks there from then on.
    .keyword = KNOWN

status-known-held = { $held ->
        [one] { $held } address
       *[other] { $held } addresses
    }, by where each was first heard of
    .keyword = KNOWN

status-known-by-hand = by hand
status-known-this-network = this network
status-known-meeting-point = a meeting point
status-known-from-us = from { $peers } of us
status-known-not-noted = not noted

status-known-where-heard = Where each was heard of says nothing about whether anybody {
    ""}answers there.
status-known-sources-lists = `333 status --sources` lists them.

status-known-sources = SOURCES
status-known-nobody-answered = nobody has answered here yet
status-known-at = at
status-known-first = first
status-known-last = last
status-known-from-before = held from before this node wrote down where addresses came from
status-known-when = { $from }, in epoch { $epoch }

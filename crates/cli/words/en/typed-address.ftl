### Reading an address somebody typed: what was wrong with one, and how one is written.

typed-address-refused = { $why }. An address is host:port, as in node.example:3333, and
    an invitation is 333: and an address, as in 333:node.example:3333.
typed-address-refused-announce = { $why }. An address is host:port, as in node.example:3333.
typed-address-refused-bind = { $typed } is not an address to listen on. It is an IP address and
    a port, as in 0.0.0.0:3333. The address alone listens on port 3333,
    and :port alone listens on every address.

typed-address-no-tag = an invitation starts with 333:
typed-address-too-long = an invitation is at most { $most } characters, and this one is { $length }
typed-address-not-canonical = one of us is one place, spelled one way, and the invitation is { $canonical }
typed-address-wrong-tag = an invitation starts with 333:, not { $number }:
typed-address-empty = no address was given
typed-address-bad-port = { $port } is not a port, which is a number from 1 to 65535
typed-address-unclosed = an address that opens with [ has to close it with ]
typed-address-scheme = { $scheme }:// belongs to a web address and not to an address here
typed-address-not-a-host = "{ $host }" is not a host name or an IP address
typed-address-not-an-onion = { $host } is not an onion address, which is { $letters } letters and digits
    before .onion

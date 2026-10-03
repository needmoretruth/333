### Reading an address somebody typed: what was wrong with one, and how one is written.

typed-address-refused = { $why }. Adreso estas host:port, kiel node.example:3333, kaj
    invito estas 333: kaj adreso, kiel 333:node.example:3333.
typed-address-refused-announce = { $why }. Adreso estas host:port, kiel node.example:3333.
typed-address-refused-bind = { $typed } ne estas adreso por aŭskulti. Ĝi estas IP-adreso kaj
    retpordo, kiel 0.0.0.0:3333. La adreso sola aŭskultas ĉe la retpordo
    3333, kaj :port sola aŭskultas ĉe ĉiu adreso.

typed-address-no-tag = invito komenciĝas per 333:
typed-address-too-long = invito havas maksimume { $most } signojn, kaj ĉi tiu havas { $length }
typed-address-not-canonical = unu el ni estas unu loko, skribita unu maniere, kaj la invito estas {
    ""}{ $canonical }
typed-address-wrong-tag = invito komenciĝas per 333:, ne per { $number }:
typed-address-empty = neniu adreso estis donita
typed-address-bad-port = { $port } ne estas retpordo, kiu estas nombro de 1 ĝis 65535
typed-address-unclosed = adreso, kiu malfermiĝas per [, devas fermiĝi per ]
typed-address-scheme = { $scheme }:// apartenas al retadreso kaj ne al adreso ĉi tie
typed-address-not-a-host = "{ $host }" ne estas gastiga nomo aŭ IP-adreso
typed-address-not-an-onion = { $host } ne estas onion-adreso, kiu estas { $letters } literoj kaj
    ciferoj antaŭ .onion

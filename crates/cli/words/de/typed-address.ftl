### Reading an address somebody typed: what was wrong with one, and how one is written.

typed-address-refused = { $why }. Eine Adresse ist host:port, wie node.example:3333, und
    eine Einladung ist 333: und eine Adresse, wie 333:node.example:3333.
typed-address-refused-announce = { $why }. Eine Adresse ist host:port, wie node.example:3333.
typed-address-refused-bind = { $typed } ist keine Adresse zum Lauschen. Das ist eine IP-Adresse
    und ein Port, wie 0.0.0.0:3333. Die Adresse allein lauscht auf
    Port 3333, und :port allein lauscht auf allen Adressen.

typed-address-no-tag = eine Einladung beginnt mit 333:
typed-address-too-long = eine Einladung hat höchstens { $most } Zeichen, und diese hat { $length }
typed-address-not-canonical = jeder von uns ist ein Ort, auf eine Weise geschrieben, und die
    Einladung ist { $canonical }
typed-address-wrong-tag = eine Einladung beginnt mit 333:, nicht mit { $number }:
typed-address-empty = keine Adresse angegeben
typed-address-bad-port = { $port } ist kein Port, also keine Zahl von 1 bis 65535
typed-address-unclosed = eine Adresse, die mit [ beginnt, muss mit ] schließen
typed-address-scheme = { $scheme }:// gehört zu einer Webadresse und nicht zu einer Adresse hier
typed-address-not-a-host = „{ $host }“ ist weder ein Hostname noch eine IP-Adresse
typed-address-not-an-onion = { $host } ist keine Onion-Adresse, die { $letters } Buchstaben und
    Ziffern vor .onion hat

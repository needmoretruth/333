### Reading an address somebody typed: what was wrong with one, and how one is written.

typed-address-refused = { $why }. Una dirección es host:puerto, como node.example:3333, y
    una invitación es 333: y una dirección, como 333:node.example:3333.
typed-address-refused-announce = { $why }. Una dirección es host:puerto, como node.example:3333.
typed-address-refused-bind = { $typed } no es una dirección en la que escuchar. Es una dirección
    IP y un puerto, como 0.0.0.0:3333. La dirección sola escucha en el
    puerto 3333, y :puerto solo escucha en todas las direcciones.

typed-address-no-tag = una invitación empieza por 333:
typed-address-too-long = una invitación tiene como mucho { $most } caracteres, y esta tiene { $length }
typed-address-not-canonical = cada uno de nosotros es un lugar, escrito de una forma, y la
    invitación es { $canonical }
typed-address-wrong-tag = una invitación empieza por 333:, no por { $number }:
typed-address-empty = no se dio ninguna dirección
typed-address-bad-port = { $port } no es un puerto, que es un número del 1 al 65535
typed-address-unclosed = una dirección que se abre con [ tiene que cerrarse con ]
typed-address-scheme = { $scheme }:// es de una dirección web, no de una dirección de aquí
typed-address-not-a-host = "{ $host }" no es un nombre de host ni una dirección IP
typed-address-not-an-onion = { $host } no es una dirección onion, que son { $letters } letras y cifras
    antes de .onion

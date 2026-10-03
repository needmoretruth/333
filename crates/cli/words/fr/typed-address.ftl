### Reading an address somebody typed: what was wrong with one, and how one is written.

typed-address-refused = { $why }. Une adresse s’écrit hôte:port, comme node.example:3333, et
    une invitation s’écrit 333: puis une adresse, comme
    333:node.example:3333.
typed-address-refused-announce = { $why }. Une adresse s’écrit hôte:port, comme node.example:3333.
typed-address-refused-bind = { $typed } n’est pas une adresse d’écoute. C’est une adresse IP et un
    port, comme 0.0.0.0:3333. L’adresse seule écoute sur le port 3333,
    et :port seul écoute sur toutes les adresses.

typed-address-no-tag = une invitation commence par 333:
typed-address-too-long = une invitation fait au plus { $most } caractères, et celle-ci { $length }
typed-address-not-canonical = chacun de nous est un lieu, écrit d’une seule façon, et l’invitation
    est { $canonical }
typed-address-wrong-tag = une invitation commence par 333:, pas par { $number }:
typed-address-empty = aucune adresse n’a été donnée
typed-address-bad-port = { $port } n’est pas un port, qui est un nombre de 1 à 65535
typed-address-unclosed = une adresse qui ouvre par [ doit fermer par ]
typed-address-scheme = { $scheme }:// appartient à une adresse web, pas à une adresse d’ici
typed-address-not-a-host = « { $host } » n’est ni un nom d’hôte ni une adresse IP
typed-address-not-an-onion = { $host } n’est pas une adresse onion, qui fait { $letters } lettres et
    chiffres avant .onion

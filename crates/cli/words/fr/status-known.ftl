### `333 status`: where the others are, as far as this node knows.

status-known-another-copy = UNE AUTRE COPIE DE CE NOM

status-known-sighting = Une déclaration signée avec la clé de ce nœud, que ce nœud n’a
    jamais faite, dit qu’il est à { $address }, à l’époque { $said_in }.
    Elle est arrivée { $arrived }, à l’époque { $epoch }.

status-known-either = Soit ce répertoire a été copié et la copie lancée, soit quelqu’un
    d’autre a la clé. Deux nœuds sous un même nom se contredisent à
    chaque époque où l’un des deux est interrogé. Arrêtez-en un ;
    `333 pack` est la façon de déménager un nœud. Rien ici n’arrête
    l’une ou l’autre copie pour vous : n’importe qui peut rejouer une
    ancienne déclaration, et un nœud qui s’arrêterait en la voyant
    pourrait être éteint par quiconque détient une copie de sa clé.

status-known-nowhere = nulle part où frapper encore. Une invitation donnée à `333 ping`
    ou `333 join` est gardée, et le nœud y frappe désormais.
    .keyword = CONNU

status-known-held = { $held ->
        [one] { $held } adresse
       *[other] { $held } adresses
    }, selon où chacune a été entendue la première fois
    .keyword = CONNU

status-known-by-hand = à la main
status-known-this-network = ce réseau
status-known-meeting-point = un point de rencontre
status-known-from-us = de { $peers } d’entre nous
status-known-not-noted = non noté

status-known-where-heard = Où chacune a été entendue ne dit rien de qui y répond.
status-known-sources-lists = `333 status --sources` les liste.

status-known-sources = SOURCES
status-known-nobody-answered = personne n’a encore répondu ici
status-known-at = à
status-known-first = première
status-known-last = dernière
status-known-from-before = gardée d’avant que ce nœud note d’où venaient les adresses
status-known-when = { $from }, à l’époque { $epoch }

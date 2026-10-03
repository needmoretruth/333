### Where the others are, and another copy of this node's name.

node-addresses-reading = lecture d’une adresse
node-addresses-keeping = conservation d’une adresse
node-addresses-reading-own = lecture de l’adresse de ce nœud
node-addresses-keeping-own = conservation de l’adresse de ce nœud

node-addresses-another-copy = copie de ce nom existe ailleurs. Une déclaration signée avec la
    clé de ce nœud, que ce nœud n’a jamais faite, dit qu’il est à
    { $address }, à l’époque { $said_in }.
    Elle est arrivée { $from }. Soit ce répertoire a été copié et la copie
    lancée, soit quelqu’un d’autre a la clé. Deux nœuds sous un même nom
    se contredisent à chaque époque où l’un des deux est interrogé.
    Arrêtez-en un ; `333 pack` est la façon de déménager un nœud.
    Celui-ci continue de tourner jusqu’à ce que vous décidiez lequel.
    .keyword = autre

node-addresses-unread = la note de l’origine de chaque adresse est illisible, une nouvelle
    commence donc. Rien de ce que décide ce nœud ne la lit.
    .keyword = non lu

node-addresses-copies = { $copies ->
        [one] une déclaration signée
       *[other] { $copies } déclarations signées
    } avec la clé de ce nœud, qu’il n’a pas faite(s),
    l’ont atteint dans la fenêtre. Une autre copie de ce nom a tourné.
    `333 status` dit où elle disait être.
    .keyword = autre

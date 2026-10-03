### Where the others are, and another copy of this node's name.

node-addresses-reading = lese eine Adresse
node-addresses-keeping = bewahre eine Adresse
node-addresses-reading-own = lese die Adresse dieses Knotens
node-addresses-keeping-own = bewahre die Adresse dieses Knotens

node-addresses-another-copy = Kopie dieses Namens ist da draußen. Eine Aussage, signiert mit dem
    Schlüssel dieses Knotens, die dieser Knoten nie gemacht hat, sagt,
    er sei bei { $address }, in Epoche { $said_in }.
    Sie kam { $from }. Entweder wurde dieses Verzeichnis kopiert und die
    Kopie gestartet, oder jemand anderes hat den Schlüssel. Zwei Knoten
    mit einem Namen widersprechen sich in jeder Epoche, zu der einer von
    ihnen gefragt wird. Stoppe einen; `333 pack` ist der Weg, einen
    Knoten umzuziehen. Dieser läuft weiter, bis du dich entscheidest.
    .keyword = andere

node-addresses-unread = die Notiz, woher jede Adresse kam, war nicht lesbar, also beginnt
    eine neue. Nichts, was dieser Knoten entscheidet, liest sie.
    .keyword = unlesbar

node-addresses-copies = { $copies ->
        [one] eine Aussage
       *[other] { $copies } Aussagen
    }, signiert mit dem Schlüssel dieses Knotens, die er nicht
    gemacht hat, erreichten ihn im Fenster. Eine andere Kopie dieses
    Namens ist gelaufen. `333 status` sagt, wo sie zu sein angab.
    .keyword = andere

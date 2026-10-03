### `333 status`: where the others are, as far as this node knows.

status-known-another-copy = NOCH EINE KOPIE DIESES NAMENS

status-known-sighting = Eine Aussage, signiert mit dem Schlüssel dieses Knotens, die dieser
    Knoten nie gemacht hat, sagt, er sei bei { $address }, in Epoche { $said_in }.
    Sie kam { $arrived }, in Epoche { $epoch }.

status-known-either = Entweder wurde dieses Verzeichnis kopiert und die Kopie gestartet,
    oder jemand anderes hat den Schlüssel. Zwei Knoten mit einem Namen
    widersprechen sich in jeder Epoche, zu der einer von ihnen gefragt
    wird. Stoppe einen; `333 pack` ist der Weg, einen Knoten
    umzuziehen. Nichts hier stoppt eine der Kopien für dich: eine alte
    Aussage kann jeder wieder abspielen, und ein Knoten, der beim
    Anblick einer solchen stoppte, ließe sich von jedem abschalten, der
    eine Kopie seines Schlüssels hat.

status-known-nowhere = noch nirgends anzuklopfen. Eine Einladung an `333 ping` oder
    `333 join` wird aufbewahrt, und der Knoten klopft dort von da an.
    .keyword = BEKANNT

status-known-held = { $held ->
        [one] { $held } Adresse
       *[other] { $held } Adressen
    }, danach, wo von jeder zuerst gehört wurde
    .keyword = BEKANNT

status-known-by-hand = von Hand
status-known-this-network = dieses Netz
status-known-meeting-point = ein Treffpunkt
status-known-from-us = von { $peers } von uns
status-known-not-noted = nicht notiert

status-known-where-heard = Woher eine Adresse kam, sagt nichts darüber, ob dort jemand antwortet.
status-known-sources-lists = `333 status --sources` listet sie.

status-known-sources = QUELLEN
status-known-nobody-answered = hier hat noch niemand geantwortet
status-known-at = bei
status-known-first = zuerst
status-known-last = zuletzt
status-known-from-before = gehalten, bevor dieser Knoten Herkünfte aufschrieb
status-known-when = { $from }, in Epoche { $epoch }

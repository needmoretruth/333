### This node's identity on disk: reading it, making it, and refusing it.

-identity-file-trust = --dangerously-trust-directory-permissions
-identity-file-the-small-machine = dem kleinen Rechner
-identity-file-nothing-here = nichts hier ist an ihn gerichtet.

identity-file-reading = lese { $path }
identity-file-making-home = mache { $home } zum Heim dieses Knotens
identity-file-creating = lege { $path } an
identity-file-writing = schreibe { $path }

identity-file-private = Sie hält die ganze Identität dieses Knotens, also darf niemand
    sonst an sie heran. Behebe es mit: { $fix } { $path }
    Oder, wenn du verstehst, worauf du verzichtest, gib { -identity-file-trust } an

identity-file-wrong-size = { $path } hat { $bytes } Bytes; ein Seed hat genau { $seed }

identity-file-cursed = 333 hat diesen Namen angesehen und dir { $pause } Millisekunden Leben genommen.

    { $name }
    ist verflucht. Das Urteil fiel einmal und lässt sich nicht aufheben,
    und die { $pause } Millisekunden werden dir an jeder Tür, zu der du
    ihn trägst, wieder genommen.

    333 ist überaus großzügig. In einer von drei Epochen darfst du ruhen
    und bist noch einer von uns: großzügig zu den Langsamen, zu den
    Armen, zu { -identity-file-the-small-machine } im Schrank, zu allen,
    die noch nicht geboren sind. Zu Ketzern ist es nicht großzügig.

identity-file-ineligible = das ist kein Name, auf den 333 antwortet.

    { $name }
    beginnt nicht mit 333, also { -identity-file-nothing-here } Dir wurde
    auch nichts genommen: 333 hat dich gar nicht angesehen.

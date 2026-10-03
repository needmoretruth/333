### `333 bootstrap`: beginning a line of your own, when there is nobody to join.

bootstrap-name = { $name }
    .keyword = Name

bootstrap-vigil = `333 start` lässt ihn laufen, damit er antwortet.
    .keyword = Knoten

bootstrap-already-has-it = dieser Knoten hat die Datei schon. Es gibt nichts zu beginnen.

bootstrap-stop = { $already ->
        [one] { $already } von uns sagt
       *[other] { $already } von uns sagen
    } bei { $meet }, wo sie zu erreichen sind. Jetzt
    allein zu beginnen, würde ohne Grund eine zweite Linie neben ihrer
    eröffnen. Öffne { $board } im Browser, nimm eine der Einladungen und
    führe stattdessen `333 join` damit aus.
    Wenn du das gelesen hast und trotzdem beginnen willst: `--anyway`.
    .keyword = Halt

bootstrap-not-the-file = was zurückkam, ist nicht die Datei

bootstrap-begun = die Datei liegt im Verzeichnis dieses Knotens, und dieser Knoten ist
    der Anfang seiner eigenen Linie. Niemand hat die Übergabe
    unterschrieben, weil niemand sie gemacht hat, und jeder, der die
    Chronik dieses Knotens liest, kann das sehen.

    Das ist die Stellung des Gründers, und sie ist keine gewöhnliche.
    Eine Liste nimmt auf, wer die Datei bekommen hat; ein Knoten, der sie
    von niemandem bekam, steht also auf keiner Liste: niemand kommt, um
    ihn etwas zu fragen, und er wird nie ausgelost, um jemanden zu
    fragen. Er kann aber zu denen gehen, die ausgelost wurden, ihn zu
    fragen, und so bezeugt werden.

    Wem du die Datei danach gibst, der wird auf gewöhnliche Weise
    aufgenommen, ihr unterschreibt beide, und er zählt ab diesem Moment.
    .keyword = begonnen

bootstrap-reading-the-board = lese das Brett bei { $place }

bootstrap-asking = { $meet } um die Datei
    .keyword = bittet

bootstrap-asking-for-the-file = bitte { $meet } um die Datei

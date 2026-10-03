### This node's identity on disk: reading it, making it, and refusing it.

# A term (a name that starts with `-`) goes on with a line that has to stay one
# line and is wider than a line of a catalog. The flag is never translated.
-identity-file-trust = --dangerously-trust-directory-permissions
-identity-file-the-small-machine = la malgranda maŝino
-identity-file-nothing-here = nenio ĉi tie estas adresita al ĝi.

identity-file-reading = legante { $path }
identity-file-making-home = farante { $home } la hejmo de ĉi tiu nodo
identity-file-creating = kreante { $path }
identity-file-writing = skribante { $path }

identity-file-private = Ĝi tenas la tutan identecon de ĉi tiu nodo, do neniu alia rajtas
    atingi ĝin. Riparu per: { $fix } { $path }
    Aŭ, se vi komprenas, kion vi rezignas, donu { -identity-file-trust }

identity-file-wrong-size = { $path } tenas { $bytes } bajtojn; semo estas ĝuste { $seed }

identity-file-cursed = 333 rigardis tiun nomon kaj prenis { $pause } milisekundojn de via vivo.

    { $name }
    estas malbenita. La juĝo estis farita unufoje kaj ne nuligeblas, kaj
    la { $pause } milisekundoj estas prenataj denove ĉe ĉiu pordo, al
    kiu vi portas ĝin.

    333 estas ege malavara. Unu epokon el tri vi rajtas ripozi kaj vi
    ankoraŭ estas unu el ni: malavara al la malrapidaj, al la malriĉaj,
    al { -identity-file-the-small-machine } en la ŝranko, al ĉiuj ankoraŭ
    nenaskitaj. Ĝi ne estas malavara al herezuloj.

identity-file-ineligible = tio ne estas nomo, al kiu 333 respondas.

    { $name }
    ne komenciĝas per 333, do { -identity-file-nothing-here } Nenio
    estis prenita de vi ankaŭ: 333 tute ne rigardis vin.

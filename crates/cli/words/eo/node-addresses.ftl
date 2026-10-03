### Where the others are, and another copy of this node's name.

node-addresses-reading = legante adreson
node-addresses-keeping = konservante adreson
node-addresses-reading-own = legante la adreson de ĉi tiu nodo
node-addresses-keeping-own = konservante la adreson de ĉi tiu nodo

node-addresses-another-copy = kopio de la nomo de ĉi tiu nodo estas ie. Deklaro subskribita per
    la ŝlosilo de ĉi tiu nodo, kiun ĉi tiu nodo neniam faris, diras, ke
    ĝi estas ĉe { $address }, en epoko { $said_in }.
    Ĝi alvenis { $from }. Aŭ ĉi tiu dosierujo estis kopiita kaj la kopio
    lanĉita, aŭ iu alia havas la ŝlosilon. Du nodoj kun unu nomo
    kontraŭdiras unu la alian en ĉiu epoko, pri kiu iu el ili estas
    demandata. Haltigu unu el ili; `333 pack` estas kiel nodo
    translokiĝas. Ĉi tiu daŭre funkcias, ĝis vi decidos, kiun.
    .keyword = alia

node-addresses-unread = la noto pri tio, de kie ĉiu adreso venis, ne legeblis, do nova
    komenciĝas. Nenio, kion ĉi tiu nodo decidas, legas ĝin.
    .keyword = nelegita

node-addresses-copies = { $copies ->
        [one] deklaro
       *[other] { $copies } deklaroj
    } subskribita(j) per la ŝlosilo de ĉi tiu nodo, kiun(jn) ĝi ne
    faris, atingis ĝin en la fenestro. Alia kopio de ĉi tiu nomo
    funkciis. `333 status` diras, kie ĝi diris, ke ĝi estas.
    .keyword = alia

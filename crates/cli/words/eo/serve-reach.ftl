### `333 run`: whether anybody outside can actually get in.

serve-reach-unanswered-at-the-end = la enkursigilo ne respondis, kiam la nodo haltis. Kion ajn ĝi
    konsentis, tio elĉerpiĝas per si mem ene de { $time }.
    .keyword = fermita

serve-reach-shut-behind-another = la enkursigilo diras, ke ĉi tiu hejmo estas ĉe { $seen }, kiu ne
    estas adreso en la malferma interreto: alia enkursigilo, aŭ la
    komuna adreso de la provizanto, staras inter ĝi kaj ĉiuj aliaj, kaj
    nenio ĉi tie povas peti tiun. `333 run --tor` bezonas tute neniun
    ŝanĝon de enkursigilo.
    .keyword = fermita

serve-reach-open = la retpordo { $port } atingas ĉi tiun maŝinon de ekstere. Ĉi tiu nodo
    frapis ĉe { $outside } kaj respondis al si mem, do tiun adreson vi
    povas doni al iu ajn.
    .keyword = malferma

serve-reach-invite = { $invitation }
    .keyword = invito

serve-reach-shut-somebody-else = io respondis ĉe { $outside }, kaj ĝi ne estis ĉi tiu nodo. Tiu
    retpordo ĉe via adreso apartenas al io alia, do invito, kiu nomas ĝin,
    sendus homojn al la malĝusta maŝino.
    .keyword = fermita

serve-reach-shut-unfinished = io ĉe { $outside } akceptis la konekton kaj ne finis korbaton:
    { $why }. Invito, kiu nomas ĝin, ne estas por disdoni.
    .keyword = fermita

serve-reach-shut-nothing = nenio respondis ĉe { $outside }, do laŭ tio, kion la ekstera mondo
    povas diri, ĉi tiu nodo ne aŭskultas. Aŭ la enkursigilo antaŭ ĝi
    neniam estis instruita sendi la retpordon { $port } ĉi tien, aŭ ĝi
    ne lasas maŝinon interne voki sian propran eksteran adreson.
    `333 run --tor` bezonas neniun ŝanĝon de enkursigilo kaj funkcias en
    ajna reto.
    .keyword = fermita

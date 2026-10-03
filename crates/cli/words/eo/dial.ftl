### Reaching another node, whichever way its address says to.
##
## A line here ends in `{` and the next begins with `""}` where the printed line is
## wider than a line of this file may be; the placeable spanning the two prints no
## line break.

dial-would-show = ĉi tiu nodo tenas sian adreson nevidebla, do ĝi ne malfermos konekton {
    ""}al { $address }, kio montrus ĝin

dial-no-answer = neniu respondo post { $seconds } s

dial-waking = iu atingenda estas ĉe nevidebla adreso kaj Tor ne funkcias.
    La unua lanĉo daŭras de sekundoj ĝis minutoj, kaj nenio estas
    demandata de iu ajn ĝis ĝi finiĝos.
    .keyword = vekiĝas

dial-unwoken = Tor ne lanĉiĝis: { $why }
    Nevideblaj adresoj estas preterlasataj ĉi-epoke. La nodoj malantaŭ
    ili ne malsukcesis respondi — nenio atingis ilin por demandi.
    .keyword = nevekita

dial-connecting = konektante al { $address }
dial-without-tor = ĉi tiu kliento estis konstruita sen Tor, do ĝi ne povas atingi { $address }

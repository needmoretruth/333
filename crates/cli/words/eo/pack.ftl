### `333 pack`: writing this node into one file, to be carried to another machine.

pack-name-the-file = nomu la dosieron, en kiun paki ĉi tiun nodon: 333 pack <FILE>

pack-no-node = ne estas nodo en { $root } por paki. Nenio estis skribita.

# A term (a name that starts with `-`) goes on with a line that has to stay one
# line and is wider than a line of a catalog.
pack-already-exists = { $file } jam ekzistas. { -pack-never-over }
-pack-never-over = Pakado skribas novan dosieron kaj neniam super malnova; nomu alian.

pack-not-marked = { $file } estas skribita, sed ĉi tiu dosierujo ne markeblis kiel pakita. { -pack-until-it-is }
-pack-until-it-is = Ĝis ĝi estos, ĉi tiu nodo loĝas en ambaŭ: { -pack-delete-that-file }
-pack-delete-that-file = forigu tiun dosieron antaŭ ol io ajn funkcias ĉi tie.

pack-creating = kreante { $file }

pack-name = { $name }
    .keyword = nomo

pack-record-none = ankoraŭ neniu
    .keyword = registro

pack-record = { $epochs ->
        [one] { $epochs } epoko, iranta kun ĝi
       *[other] { $epochs } epokoj, irantaj kun ĝi
    }
    .keyword = registro

pack-witnessed = { $statements ->
        [one] { $statements } deklaro, kiun alia ŝlosilo subskribis pri ĝi, iranta kun ĝi
       *[other] { $statements } deklaroj, kiujn aliaj ŝlosiloj subskribis pri ĝi, irantaj
            kun ĝi
    }
    .keyword = atesto

pack-holding = la dosiero, iranta kun ĝi
    .keyword = tenas

pack-onion-key = la ŝlosilo de ĝia onion-adreso, do la adreso iras kun ĝi
    .keyword = nevidita

pack-carrying = ĉi tiu nodo, en { $file }.
    Tiu dosiero ESTAS ĉi tiu nodo: kiu tenas ĝin, povas respondi kiel ĉi
    tiu nomo. Portu ĝin, malpaku ĝin, poste forigu ĝin; ĝi ne estas
    sekurkopio por teni. Ĝi ne estas ĉifrita, ĉar pasvorto estus unu plia
    afero por perdi, kaj perdi ĝin perdus la nomon tiel certe kiel perdi
    la dosieron. Nur vi povas legi ĝin, kiel ĉi tiun dosierujon.
    .keyword = portata

pack-packed = { $bytes } bajtoj.
    nenio en { $root } agos kiel ĉi tiu nodo denove.
    .keyword = pakita

pack-next = sur la alia maŝino: 333 unpack { $carried }
    se la translokiĝo estas forlasita: { $undo }
    .keyword = sekve

pack-not-packed = ĉi tiu nodo ne estis pakita, do estas nenio por malfari en { $root }
    .keyword = ĉi tie

pack-restored = ĉi tiu nodo loĝas en { $root } denove.
    La dosiero, en kiun ĝi estis pakita, ankoraŭ estas ĉi tiu nomo. Se ĝi
    estis malpakita ie ajn, unu el la du devas foriri antaŭ ol iu el ili
    funkcias; se ne, forigu { $file }
    .keyword = revenis

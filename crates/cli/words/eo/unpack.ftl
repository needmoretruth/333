### `333 unpack`: putting a packed node into this machine's node directory.

# A term (a name that starts with `-`) goes on with a line that has to stay one
# line and is wider than a line of a catalog.
-unpack-nothing = Nenio estis malpakita.
-unpack-so-nothing = do nenio estis malpakita.

unpack-kept = nodo jam loĝas en ĉi tiu dosierujo. Por malpaki apud ĝi anstataŭe,
    donu al ĝi propran dosierujon per --data-dir.

unpack-seed-in = la { $seed } en { $file }

unpack-not-one-node = tiu dosiero diras, ke ĝi tenas { $claimed }, kaj la ŝlosilo en ĝi estas { $name }. { -unpack-not-one }
-unpack-not-one = Ĝi ne estas unu nodo, kaj nenio estis malpakita.

unpack-taken = alia 333 prenis la dosierujon, en kiun ĉi tio malpakis. { -unpack-nothing }

unpack-elsewhere = 333 --data-dir <alia dosierujo> unpack { $file }

unpack-could-not-open = nodo jam loĝas en { $target }, kaj ĝi { -unpack-could-not-be-opened } Por malpaki apud ĝi anstataŭe: { $elsewhere }
-unpack-could-not-be-opened = ne malfermeblis por diri, kion ĝi tenas. { -unpack-nothing }

unpack-no-record = ankoraŭ neniu registro
unpack-epochs-of-record = { $epochs ->
        [one] { $epochs } epoko de registro
       *[other] { $epochs } epokoj de registro
    }
unpack-holding = tenanta la dosieron
unpack-not-holding = ne tenanta la dosieron

unpack-occupied = nodo jam loĝas en { $target }:
    { $name }, { $epochs }, { $holding }.
    Malpaki super ĝi perdus ĉion tion por ĉiam, { -unpack-so-nothing }
    Por malpaki apud ĝi anstataŭe, donu al ĝi propran dosierujon:
    { $elsewhere }

unpack-holds-files = { $target } tenas dosierojn kaj neniun nodon. { -unpack-its-own } Por malpaki aliloke: { $elsewhere }
-unpack-its-own = Nodo estas malpakata en propran dosierujon, { -unpack-so-nothing }

unpack-opening-the-record = malfermante la registron
unpack-torn = la registro en tiu dosiero estas ŝirita, do ĝi ne estas tuta nodo. { -unpack-nothing }
unpack-reading-the-record = legante la registron
unpack-does-not-verify = la registro en tiu dosiero ne kontroliĝas. { -unpack-nothing }
unpack-another-key = la registro en tiu dosiero estis skribita de alia ŝlosilo. { -unpack-nothing }

unpack-not-a-place = { $target } ne estas dosierujo, en kiun nodo povas esti metita
unpack-making-room = farante lokon ĉe { $target }
unpack-putting = metante la nodon en { $target }

unpack-name = { $name }
    .keyword = nomo

unpack-record-none = ankoraŭ neniu
    .keyword = registro

unpack-record = { $epochs ->
        [one] { $epochs } epoko, kontrolita
       *[other] { $epochs } epokoj, kontrolitaj
    }
    .keyword = registro

unpack-holding-the-file = la dosiero
    .keyword = tenas

unpack-onion-key = la ŝlosilo de ĝia onion-adreso, do la adreso venis kun ĝi
    .keyword = nevidita

unpack-unpacked = en { $target },
    el dosiero pakita ĉe { $packed }.
    Ĉi tio estas la nodo nun, kaj ankaŭ tiu dosiero: forigu { $file }
    .keyword = malpakis

unpack-next = { $serve }
    .keyword = sekve

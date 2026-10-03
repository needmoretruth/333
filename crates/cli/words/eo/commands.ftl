### What the shared parts of the commands say.

commands-clock-at-zero = { $epoch }. La horloĝo de ĉi tiu maŝino diras, ke estas 1970, do ĉi tiu
    nodo kredas, ke ĝi estas ĉe la komenco de la tempo. Neniu donos al ĝi
    ion kaj neniu atestos ĝin, ĝis la horloĝo estos ĝustigita.
    .keyword = epoko

commands-called-first = la unua farita ŝlosilo estis vokita.
    .keyword = vokita

commands-called = { $not_called ->
        [one] { $not_called } ŝlosilo estis farita kaj ne vokita. ĉi tiu estis.
       *[other] { $not_called } ŝlosiloj estis faritaj kaj ne vokitaj. ĉi tiu estis.
    }
    .keyword = vokita

commands-torn = { $bytes } bajtoj de nefinita ero estis forigitaj el la registro
    .keyword = ŝirita

commands-record = { $epochs ->
        [one] { $epochs } epoko
       *[other] { $epochs } epokoj
    } jam responditaj, neniu el ili reviziebla
    .keyword = registro

commands-witnessed = { $statements ->
        [one] { $statements } deklaro, kiun alia ŝlosilo subskribis pri ĉi tiu nodo. Ĝi
            restas post kiam ĝia epoko pasis, ĉar nenio alia el ĝi
            travivas la fenestron.
       *[other] { $statements } deklaroj, kiujn aliaj ŝlosiloj subskribis pri ĉi tiu nodo.
            Ili restas post kiam iliaj epokoj pasis, ĉar nenio alia el ili
            travivas la fenestron.
    }
    .keyword = atesto

commands-unseen = nenio estis subskribita pri ĉi tiu nodo, en iu ajn epoko. Atingi
    eksteren funkcias kaj esti atingita ne, kaj nur la dua estas
    kalkulata: kiu estas lotita por demandi, devas alveni. Du aferoj
    kaŭzas tion. Enkursigilo, kiu ne sendas la retpordon 3333 al ĉi tiu
    maŝino, kaj adreso, kiun neniu ricevis. `run --tor` bezonas nek
    unu nek alian — onion-adreso estas atingebla malantaŭ ajna
    enkursigilo, kaj ĉi tiu kliento jam portas Tor.
    .keyword = nevidita

commands-roll-alone = 1 el ni, kiu estas ĉi tiu nodo
    .keyword = nomlisto

commands-roll = { $members } el ni
    .keyword = nomlisto

commands-known = kie { $addresses } el ni diris serĉi
    .keyword = konata

commands-holding = la dosiero, kaj povas pludoni ĝin
    .keyword = tenas

commands-keeping = ĉio, por ĉiam. Tio aĉetas al ĉi tiu nodo nenion: ĉiu deklaro portas
    sian propran subskribon kaj kontroliĝas same, kie ajn ĝi estis
    konservita. Ne ekzistas oficiala arkivo kaj ne ekzistas arkivisto.
    .keyword = konservo

commands-ignored = { $admissions } akceptoj, kiuj ne legeblis
    .keyword = ignoris

commands-learned-where = kie { $addresses } pliaj el ni estas
    .keyword = lernis

commands-rejoined = { $members } pliaj el ni laŭ nomo, de nodo, kiu konis { $were }. Estis
    du el ni kaj nun la kalkulado estas unu kalkulo.
    .keyword = kunigis

commands-learned-names = { $members } pliaj el ni laŭ nomo
    .keyword = lernis

commands-heard = { $speakers } el ni parolas
    .keyword = aŭdis

commands-carried = { $statements ->
        [one] { $statements } deklaro pri epoko ankoraŭ malfermita
       *[other] { $statements } deklaroj pri epokoj ankoraŭ malfermitaj
    }
    .keyword = portata

commands-exchange = { $node }  epoko { $epoch }  { $clocks }  ({ $liveness })
    .keyword = atesto

commands-answered-the-challenge = respondis la defion, kiun ni elektis
commands-spoke-first = parolis unue, kio pruvas nur, ke ĝi parolis

commands-clocks-together = horloĝoj kune
commands-clocks-ahead = ilia horloĝo { $apart } antaŭ la nia
commands-clocks-behind = ilia horloĝo { $apart } malantaŭ la nia
commands-hours-and-minutes = { $hours }h { $minutes }m
commands-minutes-and-seconds = { $minutes }m { $seconds }s
commands-seconds = { $seconds }s

commands-waking = Tor. la nevidebla vojo bezonas iom da tempo por malfermiĝi.
    .keyword = vekiĝas

commands-waking-through = Tor, tra { $bridges ->
        [one] { $bridges } ponto
       *[other] { $bridges } pontoj
    }. la nevidebla vojo bezonas iom da tempo por malfermiĝi.
    .keyword = vekiĝas

commands-no-tor = neniu Tor-konekto post { $seconds } s
commands-starting-tor = lanĉante la Tor-klienton

# What a handover puts a signature under, read back. The closing line is the same
# at both ends: it is the one formula both sides of the act speak.
commands-signed-giving = vi diris: mi transdonis la dosieron al vi en epoko { $epoch }.
    ili diris: mi ricevis la dosieron de vi en epoko { $epoch }.
    ĝi estas skribita per du manoj, kaj neniu mano povas repreni ĝin.
    .keyword = signita

commands-signed-taking = ili diris: mi transdonis la dosieron al vi en epoko { $epoch }.
    vi diris: mi ricevis la dosieron de vi en epoko { $epoch }.
    ĝi estas skribita per du manoj, kaj neniu mano povas repreni ĝin.
    .keyword = signita

commands-brimming = { $statements ->
        [one] { $statements } deklaro ne eniris unu rondon kaj atendas la sekvan
       *[other] { $statements } deklaroj ne eniris unu rondon kaj atendas la sekvan
    }
    .keyword = plena

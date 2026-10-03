# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = Kopii
js-copied = Kopiita
js-selected = Elektita
js-state-awake = La nodo de ĉi tiu retejo estas maldorma
js-state-not-running = La nodo de ĉi tiu retejo ne ruliĝas
js-in-hours = post { $h } h { $m } min
js-in-minutes = post { $m } min
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = la { $n }-a epoko de ĉi tiu linio

## The network

js-network-state-founder = En neniu nomlisto
js-network-state-ok = Respondas en ĉi tiu epoko
js-network-state-quiet = Silenta en ĉi tiu epoko
js-network-state-later = Kalkulata ekde posta epoko
js-network-state-seen = Vidita, ne en la nomlisto
js-network-awake = Maldorma
js-network-not-running = Ne ruliĝas
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · via
js-network-find-bad = La nomo de nodo estas deksesuma; tajpu almenaŭ la unuajn 6 signojn.
js-network-find-none = La nodo de ĉi tiu retejo ne vidis nodon kun tiu nomo.
js-network-find-many = { $count } nodoj komenciĝas tiel. Tajpu pli de la nomo.
js-network-find-marked = Markita kiel via sur ĉi tiu aparato.
js-network-select = Elektu nodon por vidi, kion la nodo de ĉi tiu retejo scias pri ĝi.
js-network-role-founder = Fondinto de ĉi tiu linio
js-network-role-site = La nodo de ĉi tiu retejo
js-network-role-yours = Via, sur ĉi tiu aparato
js-network-role-none = Nodo en la reto
js-network-col-name = Nomo
js-network-col-state = Ĉi tiu epoko
js-network-col-given = Ricevis la dosieron
js-network-col-counted = Kalkulata ekde
js-network-col-answered = Laste respondis
js-network-col-said = Diris
js-network-col-reached = Atingita
js-network-row-said = Diris en ĉi tiu epoko
js-network-row-handed = Transdonis la dosieron al
js-network-row-testimony = Atesto
js-network-given-by = Epoko { $epoch }, de { $sponsor }
js-network-given-founder = De neniu. Ĝi komencis ĉi tiun linion.
js-network-given-none = Ne en la nomlisto
js-network-epoch = Epoko { $epoch }
js-network-epoch-now = { $epoch } (ĉi tiu epoko)
js-network-epoch-ago = { $epoch } (antaŭ { $ago })
js-network-more = kaj { $count } pliaj en la tabelo sube
js-network-nothing = Nenio
js-network-reach-direct = Rekte
js-network-reach-tor = Per Tor
js-network-reach-tor-short = Tor
js-network-reach-unknown = Ne konata
js-network-testimony = demandita de { $asked }, demandis { $asking } (lastaj 3 epokoj)
js-network-copy-name = Kopii la nomon
js-network-select-name = Elektu la nomon supre
js-network-mine = Ĉi tiu estas mia nodo
js-network-tag-founder = fondinto
js-network-tag-site = ĉi tiu retejo
js-network-tag-yours = via
js-network-empty = La nodo de ĉi tiu retejo ankoraŭ ne vidis alian nodon.
js-network-this-node = Ĉi tiu nodo
js-network-yes = Jes
js-network-no = Ne
js-network-none = Neniu

## Where we are

js-map-watch = Sekvi vive
js-map-stop = Ĉesi sekvi
js-map-read-at = Legita je { $read_at } UTC.
js-map-unreadable = La afiŝtabulo ne legeblis ĵus nun.
js-map-tor = Tor
js-map-nowhere = Nenie, kie la rando povus loki ĝin
js-map-nobody = Neniu diras, kie ĝi estas.
js-map-all = Ĉiuj, kiuj diras
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = En la reto, sen diri kie
js-map-dot = { $count ->
    [one] { $count } nodo
   *[other] { $count } nodoj
  }

## The board

js-board-said = Dirita en epoko { $epoch } de { $node }
js-board-site = la nodo de ĉi tiu retejo
js-board-tor = per Tor

## Take the program

js-start-machine-linux-x86_64 = Linukso sur x86-64
js-start-machine-linux-aarch64 = Linukso sur 64-bita ARM
js-start-machine-linux-armv6 = Linukso sur 32-bita ARM
js-start-machine-macos-aarch64 = Mac kun Apple Silicon
js-start-machine-macos-x86_64 = Mac kun Intel-procesoro
js-start-machine-windows-x86_64 = Vindozo
js-start-phone = Ĉi tio aspektas kiel telefono aŭ tabulkomputilo, kaj la programo estas por komputilo, kiu restas ŝaltita. Elektu tiun komputilon ĉi tie.
js-start-unknown = Ĉi tiu retumilo ne diras, sur kio ĝi ruliĝas. Elektu vian maŝinon ĉi tie.
js-start-sure = Ĉi tiu retumilo diras, ke ĝi ruliĝas sur { $machine }, do tio estas elektita ĉi tie.
js-start-mac = Ĉi tiu retumilo diras, ke ĝi estas sur Mac, sed ne kiun procesoron, do Apple Silicon estas elektita ĉi tie. La instalilo demandas la maŝinon mem.
js-start-linux = Ĉi tiu retumilo diras, ke ĝi estas en Linukso, sed ne kiun procesoron, do x86-64 estas elektita ĉi tie. La instalilo demandas la maŝinon mem.
js-start-chosen = Elektita supre

## The story on the home page, drawn

js-story-file = 333.txt · 3 bajtoj
js-story-gave = Mi donis ĝin al vi
js-story-received = Mi ricevis ĝin de vi
js-story-signed = subskribita
js-story-minutes = 333 min
js-story-epochs = 333 epokoj
js-story-now = nun
js-story-answering = respondas
js-story-roll = en la nomlisto
js-story-years = { $years } jaroj

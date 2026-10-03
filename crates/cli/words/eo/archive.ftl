### The one file a node becomes while it is carried: reading and writing it.

# A term (a name that starts with `-`) goes on with a line that has to stay one
# line and is wider than a line of a catalog.
-archive-nothing = Nenio estis malpakita.

archive-not-a-packed-node = tiu dosiero ne estas pakita nodo: ĝia manifesto ne komenciĝas per {
    ""}`{ $heading }`
archive-no-format = la manifesto ne diras, kiun formaton ĝi havas
archive-newer = { -archive-packed-by-newer } { $format }. Ĉi tiu legas ĝis formato { $ours }; { -archive-use-the-newer }
-archive-packed-by-newer = tiun dosieron pakis pli nova kliento, en formato
-archive-use-the-newer = malpaku ĝin per la pli nova.
archive-no-name = la manifesto ne diras, kiun nomon ĝi enhavas

archive-listing = listigante { $dir }
archive-reading = legante { $file }
archive-packing = pakante { $file }
archive-finishing = finante la arkivon
archive-syncing = skribante la arkivon al disko

archive-opening = malfermante { $file }
archive-reading-the-archive = legante la arkivon
archive-not-packed = { $file } ne estas pakita nodo: ĝi ne legeblas kiel tia
archive-reading-the-manifest = legante la manifeston
archive-reading-the-seed = legante la semon
archive-no-manifest = tiu dosiero havas neniun manifeston, do ĝi ne estas pakita nodo
archive-no-seed = tiu dosiero enhavas neniun semon, do estas neniu nomo en ĝi

archive-making = kreante { $dir }
archive-writing = skribante { $file }
archive-reading-a-name = legante nomon en la arkivo
archive-name-not-text = la arkivo enhavas nomon, kiu ne estas teksto, kian neniu nodo havas
archive-not-a-node-file = tiu dosiero enhavas { $name }, kiu ne estas parto de nodo. { -archive-nothing }

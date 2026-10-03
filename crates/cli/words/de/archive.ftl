### The one file a node becomes while it is carried: reading and writing it.

-archive-nothing = Es wurde nichts entpackt.

archive-not-a-packed-node = keine gepackte Knoten-Datei: ihr Manifest beginnt nicht mit `{ $heading }`
archive-no-format = das Manifest sagt nicht, welches Format es hat
archive-newer = { -archive-packed-by-newer } { $format }. Dieser liest bis Format { $ours }; { -archive-use-the-newer }
-archive-packed-by-newer = diese Datei hat ein neuerer Client gepackt, im Format
-archive-use-the-newer = entpacke sie mit dem neueren.
archive-no-name = das Manifest sagt nicht, welchen Namen es enthält

archive-listing = liste { $dir } auf
archive-reading = lese { $file }
archive-packing = packe { $file }
archive-finishing = schließe das Archiv ab
archive-syncing = schreibe das Archiv auf die Platte

archive-opening = öffne { $file }
archive-reading-the-archive = lese das Archiv
archive-not-packed = { $file } ist kein gepackter Knoten: sie lässt sich nicht als einer lesen
archive-reading-the-manifest = lese das Manifest
archive-reading-the-seed = lese den Seed
archive-no-manifest = diese Datei hat kein Manifest, also ist sie kein gepackter Knoten
archive-no-seed = diese Datei enthält keinen Seed, also steckt kein Name darin

archive-making = lege { $dir } an
archive-writing = schreibe { $file }
archive-reading-a-name = lese einen Namen im Archiv
archive-name-not-text = das Archiv enthält einen Namen, der kein Text ist; das hat kein Knoten
archive-not-a-node-file = diese Datei enthält { $name }, was nicht zu einem Knoten gehört. { -archive-nothing }

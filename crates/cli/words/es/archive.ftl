### The one file a node becomes while it is carried: reading and writing it.

-archive-nothing = No se desempaquetó nada.

archive-not-a-packed-node = ese archivo no es un nodo empaquetado: su manifiesto no empieza por `{ $heading }`
archive-no-format = el manifiesto no dice qué formato tiene
archive-newer = { -archive-packed-by-newer } { $format }. Este lee hasta el formato { $ours }; { -archive-use-the-newer }
-archive-packed-by-newer = ese archivo lo empaquetó un cliente más nuevo, en el formato
-archive-use-the-newer = desempaquétalo con el más nuevo.
archive-no-name = el manifiesto no dice qué nombre contiene

archive-listing = listando { $dir }
archive-reading = leyendo { $file }
archive-packing = empaquetando { $file }
archive-finishing = terminando el archivo
archive-syncing = escribiendo el archivo en el disco

archive-opening = abriendo { $file }
archive-reading-the-archive = leyendo el archivo
archive-not-packed = { $file } no es un nodo empaquetado: no se puede leer como tal
archive-reading-the-manifest = leyendo el manifiesto
archive-reading-the-seed = leyendo la semilla
archive-no-manifest = ese archivo no tiene manifiesto, así que no es un nodo empaquetado
archive-no-seed = ese archivo no contiene semilla, así que no hay ningún nombre en él

archive-making = creando { $dir }
archive-writing = escribiendo { $file }
archive-reading-a-name = leyendo un nombre del archivo
archive-name-not-text = el archivo guarda un nombre que no es texto, algo que ningún nodo tiene
archive-not-a-node-file = ese archivo contiene { $name }, que no forma parte de un nodo. { -archive-nothing }

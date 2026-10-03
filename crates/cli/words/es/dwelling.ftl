### Where this node lives, and whether it still lives here.

-dwelling-would-be-one = sería un mismo
-dwelling-anywhere-this = en ningún sitio, esto

dwelling-made-at = creado en
dwelling-unpacked-at = desempaquetado en
dwelling-moved-to = movido, según se dijo, a
dwelling-found-at = abierto por primera vez por este cliente en

dwelling-writing = escribiendo { $file }
dwelling-putting-in-place = colocando { $file }
dwelling-resolving = resolviendo { $dir }

dwelling-elsewhere = este nodo fue { $how } { $was },
    y ahora está en { $now }.
    Si el directorio se movió o se renombró, no es más que eso. Si se
    copió y el original sigue en marcha, es un mismo nombre en dos
    lugares, y cada uno contradirá el registro del otro. Cuando solo
    quede uno, dilo aquí: { $settle }
    .keyword = hogar

dwelling-unread-moment = un momento que no se pudo leer
dwelling-unread-file = un archivo cuyo nombre no se pudo leer

dwelling-packed = este nodo se empaquetó para mudarse el { $at }, en { $into }.
    Vive dondequiera que se desempaquete ese archivo. Ponerlo en marcha
    aquí también { -dwelling-would-be-one } nombre en dos lugares, así
    que nada en { $home } actuará como él.

    Si se abandonó la mudanza y ese archivo no se desempaquetó { -dwelling-anywhere-this }
    lo devuelve: { $undo }

dwelling-unmarking = quitando la marca de que este nodo se empaquetó

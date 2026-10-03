### `333 pack`: writing this node into one file, to be carried to another machine.

pack-name-the-file = di el archivo en que empaquetar este nodo: 333 pack <FILE>

pack-no-node = no hay ningún nodo en { $root } que empaquetar. No se escribió nada.

pack-already-exists = { $file } ya existe. { -pack-never-over }
-pack-never-over = Se escribe un archivo nuevo, nunca encima de otro; elige otro nombre.

pack-not-marked = se escribió { $file }, y este directorio no se pudo marcar como empaquetado. { -pack-until-it-is }
-pack-until-it-is = Mientras no lo esté, este nodo vive en los dos: { -pack-delete-that-file }
-pack-delete-that-file = borra ese archivo antes de que se ejecute nada aquí.

pack-creating = creando { $file }

pack-name = { $name }
    .keyword = nombre

pack-record-none = aún ninguno
    .keyword = registro

pack-record = { $epochs ->
        [one] { $epochs } época, que se va con él
       *[other] { $epochs } épocas, que se van con él
    }
    .keyword = registro

pack-witnessed = { $statements ->
        [one] { $statements } declaración que otra clave firmó sobre él, que se va con él
       *[other] { $statements } declaraciones que otras claves firmaron sobre él, que se van con él
    }
    .keyword = testigo

pack-holding = el archivo, que se va con él
    .keyword = tiene

pack-onion-key = la clave de su dirección onion, así que la dirección se va con él
    .keyword = oculto

pack-carrying = este nodo, en { $file }.
    Ese archivo ES este nodo: quien lo tenga puede responder como este
    nombre. Llévalo, desempaquétalo y luego bórralo; no es una copia de
    seguridad que guardar. No está cifrado, porque una contraseña sería
    una cosa más que perder, y perderla perdería el nombre igual que
    perder el archivo. Solo tú puedes leerlo, como este directorio.
    .keyword = lleva

pack-packed = { $bytes } bytes.
    nada en { $root } volverá a actuar como este nodo.
    .keyword = hecho

pack-next = en la otra máquina: 333 unpack { $carried }
    si se abandona la mudanza: { $undo }
    .keyword = después

pack-not-packed = este nodo no se empaquetó, así que no hay nada que deshacer en { $root }
    .keyword = aquí

pack-restored = este nodo vuelve a vivir en { $root }.
    El archivo en que se empaquetó sigue siendo este nombre. Si se
    desempaquetó en algún sitio, uno de los dos tiene que irse antes de
    que cualquiera se ejecute; si no, borra { $file }
    .keyword = vuelto

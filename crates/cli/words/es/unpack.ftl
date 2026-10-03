### `333 unpack`: putting a packed node into this machine's node directory.

-unpack-nothing = No se desempaquetó nada.
-unpack-so-nothing = así que no se desempaquetó nada.

unpack-kept = ya vive un nodo en este directorio. Para desempaquetar a su lado,
    dale su propio directorio con --data-dir.

unpack-seed-in = la { $seed } en { $file }

unpack-not-one-node = ese archivo dice contener { $claimed }, y la clave que lleva es { $name }. { -unpack-not-one }
-unpack-not-one = No es un solo nodo, y no se desempaquetó nada.

unpack-taken = otro 333 tomó el directorio en que se desempaquetaba esto. { -unpack-nothing }

unpack-elsewhere = 333 --data-dir <otro directorio> unpack { $file }

unpack-could-not-open = ya vive un nodo en { $target }, y { -unpack-could-not-be-opened } Para desempaquetar a su lado: { $elsewhere }
-unpack-could-not-be-opened = no se pudo abrir para ver qué tiene. { -unpack-nothing }

unpack-no-record = aún sin registro
unpack-epochs-of-record = { $epochs ->
        [one] { $epochs } época de registro
       *[other] { $epochs } épocas de registro
    }
unpack-holding = con el archivo
unpack-not-holding = sin el archivo

unpack-occupied = ya vive un nodo en { $target }:
    { $name }, { $epochs }, { $holding }.
    Desempaquetar encima lo perdería todo para siempre, { -unpack-so-nothing }
    Para desempaquetar a su lado, dale su propio directorio:
    { $elsewhere }

unpack-holds-files = { $target } tiene archivos y ningún nodo. { -unpack-its-own } Para desempaquetar en otro sitio: { $elsewhere }
-unpack-its-own = Un nodo se desempaqueta en su propio directorio, { -unpack-so-nothing }

unpack-opening-the-record = abriendo el registro
unpack-torn = el registro de ese archivo está roto, así que no es un nodo entero. { -unpack-nothing }
unpack-reading-the-record = leyendo el registro
unpack-does-not-verify = el registro de ese archivo no se verifica. { -unpack-nothing }
unpack-another-key = el registro de ese archivo lo escribió otra clave. { -unpack-nothing }

unpack-not-a-place = { $target } no es un directorio donde se pueda poner un nodo
unpack-making-room = haciendo sitio en { $target }
unpack-putting = poniendo el nodo en { $target }

unpack-name = { $name }
    .keyword = nombre

unpack-record-none = aún ninguno
    .keyword = registro

unpack-record = { $epochs ->
        [one] { $epochs } época, verificada
       *[other] { $epochs } épocas, verificadas
    }
    .keyword = registro

unpack-holding-the-file = el archivo
    .keyword = tiene

unpack-onion-key = la clave de su dirección onion, así que la dirección vino con él
    .keyword = oculto

unpack-unpacked = en { $target },
    desde un archivo empaquetado el { $packed }.
    Este es ahora el nodo, y también lo es ese archivo: borra { $file }
    .keyword = hecho

unpack-next = { $serve }
    .keyword = después

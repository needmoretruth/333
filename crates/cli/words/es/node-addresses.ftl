### Where the others are, and another copy of this node's name.

node-addresses-reading = leyendo una dirección
node-addresses-keeping = guardando una dirección
node-addresses-reading-own = leyendo la dirección de este nodo
node-addresses-keeping-own = guardando la dirección de este nodo

node-addresses-another-copy = copia de este nombre está ahí fuera. Una declaración firmada con
    la clave de este nodo, que este nodo nunca hizo, dice que está en
    { $address }, en la época { $said_in }.
    Llegó { $from }. O se copió este directorio y se puso en marcha la
    copia, o alguien más tiene la clave. Dos nodos con un mismo nombre se
    contradicen en cada época por la que se pregunte a cualquiera de
    ellos. Detén uno; `333 pack` es la forma de mudar un nodo. Este sigue
    en marcha hasta que decidas cuál.
    .keyword = otra

node-addresses-unread = no se pudo leer la nota de dónde vino cada dirección, así que se
    empieza una nueva. Nada de lo que decide este nodo la lee.
    .keyword = ilegible

node-addresses-copies = { $copies ->
        [one] una declaración firmada
       *[other] { $copies } declaraciones firmadas
    } con la clave de este nodo, que él no hizo, le
    llegaron en la ventana. Otra copia de este nombre ha estado en
    marcha. `333 status` dice dónde dijo que estaba.
    .keyword = otra

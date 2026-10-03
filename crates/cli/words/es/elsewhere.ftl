### What a command says when another 333 already has this node's directory.

elsewhere-the-vigil = el nodo que funciona en este directorio
elsewhere-the-vigil-by-number = el nodo que funciona en este directorio (proceso { $pid })
elsewhere-another = otro 333
elsewhere-another-by-number = otro 333 (proceso { $pid })

elsewhere-done = lo hizo { $vigil }.
    .keyword = hecho

elsewhere-failed = { $vigil } no lo hizo.
    .keyword = falló

elsewhere-finding-its-name = { $who } aún está buscando el nombre
    de este nodo. Vuelve a ejecutar esto cuando lo tenga.
    .keyword = ocupado

elsewhere-already-keeping = { $who } ya funciona aquí, y un directorio es un nodo.
    Desde aquí se le pueden decir cosas: `333 say 7`,
    `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`. Un segundo
    nodo necesita su propio directorio, indicado con --data-dir.
    .keyword = ocupado

elsewhere-keeping = { $who } funciona aquí, y
    { $why }
    .keyword = ocupado

elsewhere-nobody-to-tell = no hay ningún nodo funcionando en este directorio, así que no hay
    a quién decírselo. `333 start` o `333 run` lo pone en marcha, y
    entonces esto funciona.
    .keyword = nadie

elsewhere-busy = { $who } tiene el directorio de este
    nodo y no es un nodo en marcha al que se le pueda pasar esto. Aquí no
    se leyó ni se escribió nada. Vuelve a ejecutarlo cuando termine.
    .keyword = ocupado

elsewhere-busy-cannot-be-handed = { $who } tiene el directorio de este
    nodo. En este sistema todavía no se le puede pasar nada a un 333 en
    marcha desde otra terminal, así que aquí no se leyó ni se escribió
    nada. Escríbelo en su pantalla tras `:`, o detenlo y vuelve a
    ejecutar esto.
    .keyword = ocupado

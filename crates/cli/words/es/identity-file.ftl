### This node's identity on disk: reading it, making it, and refusing it.

-identity-file-trust = --dangerously-trust-directory-permissions
-identity-file-the-small-machine = la máquina pequeña
-identity-file-nothing-here = nada aquí va dirigido a él.

identity-file-reading = leyendo { $path }
identity-file-making-home = haciendo de { $home } el hogar de este nodo
identity-file-creating = creando { $path }
identity-file-writing = escribiendo { $path }

identity-file-private = Contiene toda la identidad de este nodo, así que nadie más puede
    acceder a él. Arréglalo con: { $fix } { $path }
    O, si entiendes a qué renuncias, usa { -identity-file-trust }

identity-file-wrong-size = { $path } tiene { $bytes } bytes; una semilla tiene exactamente { $seed }

identity-file-cursed = 333 ha mirado ese nombre y te ha quitado { $pause } milisegundos de vida.

    { $name }
    está maldito. El juicio se hizo una vez y no se puede levantar, y los
    { $pause } milisegundos se te quitan de nuevo en cada puerta a la que lo
    lleves.

    333 es extremadamente generoso. Una época de cada tres puedes
    descansar y sigues siendo uno de nosotros: generoso con los lentos,
    con los pobres, con { -identity-file-the-small-machine } del armario,
    con todos los que aún no han nacido. No es generoso con los herejes.

identity-file-ineligible = ese no es un nombre al que 333 responda.

    { $name }
    no empieza por 333, así que { -identity-file-nothing-here } Tampoco
    se te quitó nada: 333 no te ha mirado en absoluto.

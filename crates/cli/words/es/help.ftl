### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph.

help-about = Un nodo de 333. Responde cuando se le pregunta, guarda su registro
    y pasa el archivo.

help-id = Muestra el nombre de este nodo y crea uno la primera vez

help-bootstrap = Empieza una línea nueva cuando nadie puede entregarte el archivo
help-bootstrap-long = Empieza una línea nueva cuando nadie puede entregarte el archivo.

    La forma habitual de entrar es `333 join` con una invitación. Esto
    mira primero el punto de encuentro y se niega si hay alguien. Si no
    hay nadie, descarga el archivo, lo comprueba con el hash que lleva
    este cliente y lo guarda. Tu nodo pasa a ser el fundador de su propia
    línea, sin la firma de nadie en su comienzo, y cualquiera que lea su
    registro puede verlo.

help-serve = Ejecuta este nodo en esta terminal hasta que lo detengas
help-serve-long = Ejecuta este nodo en esta terminal hasta que lo detengas.

    Responde a latidos y preguntas, intercambia lo que sabe y pregunta a
    los nodos que le toque preguntar en cada época. En una terminal abre
    la pantalla; `q`, Ctrl-C o `333 stop` desde otra terminal lo detienen.
    `333 start` hace lo mismo en segundo plano.

help-serve-long-light = Ejecuta este nodo en esta terminal hasta que lo detengas.

    Responde a latidos y preguntas, intercambia lo que sabe y pregunta a
    los nodos que le toque preguntar en cada época, línea a línea. Ctrl-C
    o `333 stop` desde otra terminal lo detienen. `333 start` hace lo
    mismo en segundo plano.

help-say = Di una de las 333, una vez por época. Lo que viaja es el número

help-status = Muestra si este nodo funciona, dónde pueden alcanzarlo los demás y
    cuántos de nosotros responden

help-join = Recibe el archivo de un nodo que lo tiene, con una invitación

help-languages = Lista los idiomas, o guarda uno para todas las órdenes de este nodo

help-ping = Alcanza otro nodo e intercambia un latido con él

help-pack = Escribe este nodo en un solo archivo, para llevarlo a otra máquina
help-pack-long = Escribe este nodo en un solo archivo, para llevarlo a otra máquina.

    Va todo: su nombre, su registro, lo que otros firmaron sobre él, el
    archivo y la clave de su dirección onion. Después este directorio se
    niega a ejecutarlo, para que el nombre nunca esté en dos sitios. El
    archivo no está cifrado: quien lo tenga es este nodo. Llévalo,
    desempaquétalo y bórralo.

help-unpack = Coloca un nodo empaquetado en el directorio de nodo de esta máquina
help-unpack-long = Coloca un nodo empaquetado en el directorio de nodo de esta máquina.

    Se niega donde ya vive un nodo. No se escribe nada hasta haber leído
    el archivo entero y comprobado su clave y su registro.

help-moved = Indica que el directorio de este nodo se movió o renombró, no se copió
help-moved-long = Indica que el directorio de este nodo se movió o renombró, no se copió.

    Un nodo que se encuentra en un lugar nuevo lo dice en cada ejecución
    hasta que se escribe esto, porque una copia con el original aún en
    marcha sería un mismo nombre en dos lugares.

help-tell = Da una orden a un nodo en marcha, con las palabras de su pantalla
help-tell-long = Da una orden a un nodo en marcha, con las palabras de su pantalla.

    `tor on`, `tor off`, `bridge <line>`, `helper <program>` y cualquier
    otra palabra que la pantalla acepte tras `:`. El nodo en marcha la
    cumple y su respuesta se muestra aquí. `say`, `join`, `ping`, `begin`,
    `status` y `stop` llegan igual a un nodo en marcha sin esto.

help-tell-light = Da una orden a un nodo en marcha
help-tell-long-light = Da una orden a un nodo en marcha.

    `tor on`, `tor off`, `bridge <line>` y `helper <program>`. El nodo en
    marcha la cumple y su respuesta se muestra aquí. `say`, `join`,
    `ping`, `begin`, `status` y `stop` llegan igual a un nodo en marcha
    sin esto.

help-service = Gestiona el servicio en segundo plano (lo usan `start` y `stop`)
help-service-long = Gestiona el servicio en segundo plano (lo usan `start` y `stop`).

    No se instala nada hasta que lo pidas, cada archivo escrito y cada
    orden ejecutada se muestran al hacerse, y `333 service uninstall` lo
    quita todo.

help-service-install = Instala el servicio en segundo plano con estas opciones y lo inicia
help-service-install-long = Instala el servicio en segundo plano con estas opciones y lo inicia.

    El servicio ejecuta `333 run` con exactamente las opciones dadas, para
    el directorio de este nodo, y una comprobación cada hora lo avisa en
    esta máquina si el nodo se detiene. `333 start` hace lo mismo sin
    opciones.

help-service-uninstall = Detiene el servicio en segundo plano y quita todo lo que instaló

help-service-status = Lo que dice el gestor de servicios, cuándo dijo el nodo por última vez
    que estaba despierto, y sus últimas líneas

help-service-check = Avisa en esta máquina si el nodo se ha detenido. El servicio lo ejecuta
    cada hora; no dice nada si todo va bien

help-data-dir = Directorio con todo lo que tiene este nodo: su nombre, y el estado
    de Tor si usa Tor

help-timeout = Segundos que se espera cada paso que habla con la red
help-timeout-long = Segundos que se espera cada paso que habla con la red.

    Es un límite, no una espera. Está pensado para arrancar Tor, el único
    paso que puede tardar minutos.

help-dangerously-trust-directory-permissions = Acepta un directorio al que pueden entrar otros usuarios
help-dangerously-trust-directory-permissions-long = Acepta un directorio al que pueden entrar otros usuarios.

    El directorio guarda la única copia del nombre de este nodo, así que
    uno con permisos laxos se rechaza por defecto. Esto es para
    directorios de pruebas y contenedores con propietarios raros.

help-keep-everything = Guarda todas las declaraciones para siempre, en vez de solo la ventana
    con la que se juzga
help-keep-everything-long = Guarda todas las declaraciones para siempre, en vez de solo la ventana
    con la que se juzga.

    No cambia nada de la situación de nadie: cada declaración se verifica
    igual dondequiera que se guarde.

help-bridges = Una línea de puente, para una red que bloquea la entrada normal a Tor
help-bridges-long = Una línea de puente, para una red que bloquea la entrada normal a Tor.

    Dala una vez por cada puente que te pasaron, tal como te lo pasaron.
    Aquí nada descarga puentes: los reparten personas, a propósito, para
    que ninguna lista se pueda recoger y bloquear sin más.

help-bridge-helper = El programa que habla un puente ofuscado, por nombre o por ruta
help-bridge-helper-long = El programa que habla un puente ofuscado, por nombre o por ruta.

    Solo hace falta cuando una línea de puente lo pide y no es `lyrebird`
    en la ruta. No viene incluido, porque una copia congelada pronto sería
    la equivocada.

help-language = El idioma en que hablar, como etiqueta: `ko`, `es`, `zh-Hant`
help-language-long = El idioma en que hablar, como etiqueta: `ko`, `es`, `zh-Hant`.

    Sin esto, `THE333_LANGUAGE`, luego el idioma que guardó
    `333 language <TAG>`, y luego inglés. No se usa el idioma del sistema.
    `333 language` lista los idiomas que tienen palabras, y una carpeta de
    catálogos en `<data-dir>/words/<tag>/` añade uno sin compilar nada.
    Las 333 palabras mismas nunca se traducen.

help-count-in = Cuenta en base diez, doce o twelve-ascii
help-count-in-long = Cuenta en base diez, doce o twelve-ascii.

    Cada cifra que se muestra se escribe en esa base y cada número que se
    teclea se lee en ella: `say 238` en doce es `say 332` en diez. Los
    nombres, direcciones, puertos y versiones nunca se recuentan, y nada
    cambia en la red. Sin esto, `THE333_COUNT_IN`, y luego diez.

help-bootstrap-meet = Dónde buscar gente antes de empezar por tu cuenta

help-bootstrap-anyway = Empieza aunque ya haya alguien

help-serve-bind = Dirección y puerto en los que escuchar

help-serve-tor = Abre también una dirección onion, para que otros alcancen este nodo
    sin saber dónde está. Despertar Tor tarda de segundos a minutos

help-serve-no-direct = No abre ningún socket. Solo con --tor; mantiene tu dirección
    completamente fuera de la red

help-serve-announce = La dirección que se da a otros nodos para alcanzar este
help-serve-announce-long = La dirección que se da a otros nodos para alcanzar este.

    Hace falta cuando el socket no puede decirla: al escuchar en todas las
    interfaces, o detrás de algo que reenvía un puerto.

help-serve-no-mdns = No anuncia en la red local que este nodo está aquí
help-serve-no-mdns-long = No anuncia en la red local que este nodo está aquí.

    Lo que se anuncia, si no, es que algo en esta máquina habla 333 y en
    qué puerto, no el nombre de este nodo. Así se encuentran dos nodos de
    una misma casa sin invitación.

help-serve-no-router = No pide al router que envíe el puerto a esta máquina
help-serve-no-router-long = No pide al router que envíe el puerto a esta máquina.

    Un router doméstico descarta lo que nadie de dentro pidió, hasta que
    un programa de dentro le pide que reenvíe un puerto, por UPnP-IGD,
    PCP o NAT-PMP. Cambia la red, así que se avisa cuando ocurre.
    `--no-upnp` es el nombre antiguo de esto.

help-serve-meet = Dónde buscar nodos que nadie le presentó a este
help-serve-meet-long = Dónde buscar nodos que nadie le presentó a este.

    Una dirección fija que guarda declaraciones firmadas sobre dónde están
    los nodos. Todo lo que se lee allí se verifica aquí.

help-serve-no-meet = No usa ningún punto de encuentro
help-serve-no-meet-long = No usa ningún punto de encuentro.

    Así este nodo solo es alcanzable por quien recibió una invitación y por
    los nodos de esta red, y por nadie más.

help-serve-plain = Dice las líneas en vez de dibujar la pantalla
help-serve-plain-long = Dice las líneas en vez de dibujar la pantalla.

    Fuera de una terminal siempre dice las líneas; esto lo pide también en
    una terminal.

help-serve-plain-light = Dice las líneas, que es lo que esta edición hace siempre
help-serve-plain-long-light = Dice las líneas, que es lo que esta edición hace siempre.

    Esta edición no tiene pantalla. La opción se acepta para que una misma
    orden funcione en las dos ediciones.

help-say-index = Cuál de ellas, de 0 a { $last }, escrita en la base en que cuenta
    (--count-in). Las palabras aún no están escritas

help-status-sources = Lista todas las direcciones que tiene este nodo: de quién es cada una,
    dónde y cuándo se supo de ella por primera vez, y dónde por última

help-status-json = Lo que observó este nodo, en JSON para que lo lea un programa. No
    contiene ninguna dirección ni puerto

help-join-address = Una invitación (`333:host:port`) de alguien que ya lo tiene

help-ping-address = Una invitación (`333:host:port`), o una dirección: `host`, `host:port`,
    `[::1]:port` o `algo.onion` (por Tor)

help-pack-file = El archivo que escribir. No debe existir todavía

help-pack-undo = Deshace aquí un empaquetado, para una mudanza abandonada
help-pack-undo-long = Deshace aquí un empaquetado, para una mudanza abandonada.

    Solo si el archivo no se desempaquetó en ningún sitio: si se hizo,
    esto crea dos.

help-unpack-file = El archivo que escribió `333 pack`

help-tell-order = La orden, tal como se escribiría en la pantalla

help-tell-order-light = La orden, como se escribe `tor on` o `bridge <line>`

help-service-install-flags = Las opciones de `run`, tal como las escribirías tras él

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = Muestra la ayuda
help-print-help-more = Muestra la ayuda (más con '--help')
help-print-help-summary = Muestra la ayuda (un resumen con '-h')
help-print-version = Muestra la versión
help-print-this = Muestra este mensaje o la ayuda de los subcomandos indicados
help-print-for = Muestra la ayuda de los subcomandos

help-start = Ejecuta este nodo en segundo plano, ahora y tras cada reinicio

help-stop = Detiene este nodo, y lo mantiene detenido tras un reinicio

help-restart = Detiene este nodo y vuelve a ejecutarlo en segundo plano

help-logs = Muestra las últimas líneas que escribió este nodo en segundo plano

help-logs-follow = Sigue mostrando líneas nuevas, donde systemd las guarda

help-invite = Muestra la invitación que usan otros para unirse a través de este nodo

help-status-all = Muestra todo lo que sabe este nodo, con lo que significa cada parte

help-languages-tag = El idioma que guardar, como etiqueta: `ko`, `en`. `en` vuelve al inglés

help-start-example = Ejemplo: 333 start

help-stop-example = Ejemplo: 333 stop

help-restart-example = Ejemplo: 333 restart

help-status-example = Ejemplo: 333 status --all

help-logs-example = Ejemplo: 333 logs -f

help-id-example = Ejemplo: 333 name

help-invite-example = Ejemplo: 333 invite

help-bootstrap-example = Ejemplo: 333 begin

help-serve-example = Ejemplo: 333 run --tor

help-say-example = Ejemplo: 333 say 7

help-join-example = Ejemplo: 333 join 333:192.0.2.7:3333

help-languages-example = Ejemplo: 333 language es

help-ping-example = Ejemplo: 333 ping 333:192.0.2.7:3333

help-pack-example = Ejemplo: 333 pack node.333

help-unpack-example = Ejemplo: 333 unpack node.333

help-moved-example = Ejemplo: 333 moved

help-tell-example = Ejemplo: 333 tell tor on

help-service-example = Ejemplo: 333 service status

help-service-install-example = Ejemplo: 333 service install --tor

help-service-uninstall-example = Ejemplo: 333 service uninstall

help-service-status-example = Ejemplo: 333 service status

help-service-check-example = Ejemplo: 333 service check

help-start-flags = Las opciones de `run`, tal como las escribirías tras él. Se guardan
    para cada inicio posterior hasta que se den otras

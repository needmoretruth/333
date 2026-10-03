# Take the program: the installers, every file, and the first commands.

start-meta-title = Obtener el programa · 333
start-meta-description = Instala el cliente de 333 en Linux, macOS, Windows o una Raspberry Pi, comprobado con el SHA256SUMS de la versión, y recibe el archivo.

start-heading = Obtener el programa
start-lede = 333 es un pequeño programa que dejas en marcha en un ordenador que puedas dedicarle, donde responde cada vez que llama otra copia de sí mismo.
start-pick-file = El archivo para esa máquina es <code data-name="standard">333-x86_64-linux</code>. Light, el mismo cliente sin la pantalla de terminal, es <code data-name="light">333-light-x86_64-linux</code>. Los dos llevan Tor.
start-machine = Tu máquina
start-pick-linux-x86_64 = Linux, escritorio o servidor
start-pick-linux-aarch64 = Raspberry Pi 3, 4 o 5 en 64 bits, u otro Linux ARM de 64 bits
start-pick-linux-armv6 = Raspberry Pi Zero, o cualquier Pi en 32 bits
start-pick-macos-aarch64 = Mac, Apple silicon
start-pick-macos-x86_64 = Mac, Intel
start-pick-windows = Windows
start-form = Forma
start-standard = Standard
start-light = Light
start-unix-title = En Linux o en un Mac
start-unix-installer = En un terminal. El instalador elige el archivo para la máquina en la que se ejecuta, lo comprueba con el <code>SHA256SUMS</code> de la versión y lo deja en <code>~/.local/bin/333</code>. No arranca nada ni pide contraseña. <a href="/install.sh">Léelo antes</a>; es corto.
start-unix-light = Para Light, termínalo con <code>sh -s -- --light</code> en lugar de <code>sh</code>.
start-by-hand = O a mano, sin el instalador
start-unix-by-hand = Los mismos pasos, uno a uno. La línea de <code>grep</code> imprime <code>OK</code> antes de mover nada, y si no lo imprime no se ejecuta nada de lo que sigue.
start-unix-mac = Eso es Linux en x86-64. En un Mac, usa <code>shasum -a 256</code> en lugar de <code>sha256sum</code>, y el archivo de Mac de la tabla de abajo; en cualquier otro sitio, el nombre de la tabla.
start-windows-title = En Windows
start-windows-installer = En PowerShell. El instalador descarga el archivo de Windows, lo comprueba con el <code>SHA256SUMS</code> de la versión y lo deja en <code>%LOCALAPPDATA%\Programs\333\333.exe</code>. No arranca nada, no necesita administrador y solo añade esa carpeta a tu PATH si añades <code>-AddToPath</code>. <a href="/install.ps1">Léelo antes</a>; es corto.
start-windows-light = Para Light, ejecútalo así. <code>-AddToPath</code> va al final de la misma manera.
start-windows-by-hand = Los mismos pasos, uno a uno. El archivo solo se coloca en su sitio si su hash es el que da <code>SHA256SUMS</code>, y la última línea dice qué pasó.
start-files-title = Todos los archivos
start-files-intro = Para quien la suposición de arriba no acierte, o prefiera bajar el archivo con el navegador. Standard tiene la pantalla de terminal y Light no, y los dos llevan Tor. Van de 15 megabytes en un Mac a 24 en ARM de 64 bits.
start-files-machine = Tu máquina
start-files-forms = Standard, luego Light
start-row-linux-x86_64 = Linux, escritorio o servidor
start-row-linux-aarch64 = Raspberry Pi 3, 4 o 5 en 64 bits, y otros Linux ARM de 64 bits
start-row-linux-armv6 = Raspberry Pi Zero, o cualquier Pi en 32 bits
start-row-macos-aarch64 = Mac, Apple silicon
start-row-macos-x86_64 = Mac, Intel
start-row-windows = Windows
start-files-after = Un archivo bajado con el navegador hay que comprobarlo igualmente con <a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a>, hacerlo ejecutable y ponerlo en tu PATH, que es lo que hacen los comandos de arriba. Nada aquí está firmado con un certificado de desarrollador, así que un Mac o Windows pueden negarse a abrir un archivo llegado por el navegador hasta que les digas lo contrario. Si ninguno encaja, o prefieres no ejecutar un archivo que compiló otra persona, el <a href="https://github.com/needmoretruth/333#install">repositorio</a> explica cómo compilarlo en Ubuntu, Debian, Fedora, Arch, macOS y Windows.
start-then-title = Luego, en orden
start-step-1 = <b>Recibe el archivo.</b> El nodo de este sitio está despierto a todas horas:
start-step-1-after = El primer <code>join</code> crea también el nombre de tu nodo, una clave cuyo nombre empieza por 333, lo que lleva un momento. Cualquier dirección del <a href="{ $base }/333">tablón</a> funciona igual.
start-step-2 = <b>Arráncalo.</b>
start-step-2-after = Desde ahora funciona en segundo plano, también cuando cierras sesión y tras reiniciar. Si dice <code>shut</code>, tu router no deja entrar a los demás: <code>333 start --tor</code> no necesita cambiar nada en el router.
start-step-3 = <b>Compruébalo.</b> <code>333 status</code> dice si está en marcha, dónde pueden alcanzarlo los demás y cuántos de nosotros responden. <code>333 logs</code> muestra lo que ha escrito.
start-step-4 = <b>Detenlo</b> cuando quieras: <code>333 stop</code>. Sigue detenido tras reiniciar hasta que vuelvas a ejecutar <code>333 start</code>.
start-watch = Para verlo trabajar, <code>333 run</code> lo ejecuta en este terminal con su pantalla; <code>q</code> lo detiene. <code>333</code> a secas dice en qué estado está tu nodo y qué escribir después, y <code>333 help</code> lista todos los comandos.
start-begin = Si no hay nadie en el tablón y el nodo de este sitio ha desaparecido, alguien tiene que ser el primero: <code>333 begin</code> se niega mientras haya alguien, y si no, empieza una línea propia sin que nadie la haya firmado, cosa que cualquiera que lea tu registro puede ver.
start-rule = Nada arranca hasta que tú lo arrancas.
start-rule-detail = El instalador pone un archivo en su sitio y no ejecuta nada. Tu nodo responde a alguien por primera vez, y deja su dirección en el punto de encuentro por primera vez, cuando tú mismo ejecutas <code>333 start</code> o <code>333 run</code>, y ese comando es el momento en que aceptaste.

# Take the program: the installers, every file, and the first commands.

start-meta-title = Prendre le programme · 333
start-meta-description = Installez le client 333 sous Linux, macOS, Windows ou sur un Raspberry Pi, vérifié avec le SHA256SUMS de la version, et recevez le fichier.

start-heading = Prendre le programme
start-lede = 333 est un petit programme que vous laissez tourner sur un ordinateur dont vous pouvez vous passer, où il répond chaque fois qu’une autre copie de lui-même frappe.
start-pick-file = Le fichier pour cette machine est <code data-name="standard">333-x86_64-linux</code>. Light, le même client sans l’écran de terminal, est <code data-name="light">333-light-x86_64-linux</code>. Les deux embarquent Tor.
start-machine = Votre machine
start-pick-linux-x86_64 = Linux, poste de travail ou serveur
start-pick-linux-aarch64 = Raspberry Pi 3, 4 ou 5 en 64 bits, ou autre Linux ARM 64 bits
start-pick-linux-armv6 = Raspberry Pi Zero, ou tout Pi en 32 bits
start-pick-macos-aarch64 = Mac, Apple silicon
start-pick-macos-x86_64 = Mac, Intel
start-pick-windows = Windows
start-form = Forme
start-standard = Standard
start-light = Light
start-unix-title = Sous Linux ou sur un Mac
start-unix-installer = Dans un terminal. L’installateur choisit le fichier pour la machine sur laquelle il tourne, le vérifie avec le <code>SHA256SUMS</code> de la version et le place dans <code>~/.local/bin/333</code>. Il ne démarre rien et ne demande aucun mot de passe. <a href="/install.sh">Lisez-le d’abord</a> ; il est court.
start-unix-light = Pour Light, terminez par <code>sh -s -- --light</code> au lieu de <code>sh</code>.
start-by-hand = Ou à la main, sans l’installateur
start-unix-by-hand = Les mêmes étapes, une à une. La ligne <code>grep</code> affiche <code>OK</code> avant que rien ne bouge, et rien après elle ne s’exécute si ce n’est pas le cas.
start-unix-mac = Ceci est pour Linux sur x86-64. Sur un Mac, utilisez <code>shasum -a 256</code> au lieu de <code>sha256sum</code>, et le fichier Mac du tableau ci-dessous ; ailleurs, le nom donné dans le tableau.
start-windows-title = Sous Windows
start-windows-installer = Dans PowerShell. L’installateur récupère le fichier Windows, le vérifie avec le <code>SHA256SUMS</code> de la version et le place dans <code>%LOCALAPPDATA%\Programs\333\333.exe</code>. Il ne démarre rien, ne demande pas d’administrateur et n’ajoute ce dossier à votre PATH que si vous ajoutez <code>-AddToPath</code>. <a href="/install.ps1">Lisez-le d’abord</a> ; il est court.
start-windows-light = Pour Light, lancez-le plutôt ainsi. <code>-AddToPath</code> se met à la fin de la même façon.
start-windows-by-hand = Les mêmes étapes, une à une. Le fichier n’est mis en place que si son hash est celui que donne <code>SHA256SUMS</code>, et la dernière ligne dit ce qui s’est passé.
start-files-title = Tous les fichiers
start-files-intro = Pour qui la supposition ci-dessus se trompe, ou qui préfère prendre le fichier par le navigateur. Standard a l’écran de terminal, Light non, et les deux embarquent Tor. Ils vont de 15 mégaoctets sur Mac à 24 sur ARM 64 bits.
start-files-machine = Votre machine
start-files-forms = Standard, puis Light
start-row-linux-x86_64 = Linux, poste de travail ou serveur
start-row-linux-aarch64 = Raspberry Pi 3, 4 ou 5 en 64 bits, et autres Linux ARM 64 bits
start-row-linux-armv6 = Raspberry Pi Zero, ou tout Pi en 32 bits
start-row-macos-aarch64 = Mac, Apple silicon
start-row-macos-x86_64 = Mac, Intel
start-row-windows = Windows
start-files-after = Un fichier pris par le navigateur doit quand même être vérifié avec <a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a>, rendu exécutable et placé dans votre PATH, ce que font les commandes ci-dessus. Rien ici n’est signé par un certificat de développeur : un Mac ou Windows peut donc refuser d’ouvrir un fichier venu du navigateur tant que vous ne lui dites pas le contraire. Si aucun ne convient, ou si vous préférez ne pas exécuter un fichier compilé par quelqu’un d’autre, le <a href="https://github.com/needmoretruth/333#install">dépôt</a> explique comment le compiler sous Ubuntu, Debian, Fedora, Arch, macOS et Windows.
start-then-title = Ensuite, dans l’ordre
start-step-1 = <b>Recevez le fichier.</b> Le nœud de ce site est éveillé à toute heure :
start-step-1-after = Le premier <code>join</code> crée aussi le nom de votre nœud, une clé dont le nom commence par 333, ce qui prend un instant. N’importe quelle adresse du <a href="{ $base }/333">tableau</a> fonctionne de la même façon.
start-step-2 = <b>Démarrez-le.</b>
start-step-2-after = Il tourne désormais en arrière-plan, après votre déconnexion et après un redémarrage. S’il affiche <code>shut</code>, votre routeur empêche les autres d’entrer : <code>333 start --tor</code> ne demande aucun changement de routeur.
start-step-3 = <b>Vérifiez-le.</b> <code>333 status</code> dit s’il tourne, où les autres peuvent le joindre et combien d’entre nous répondent. <code>333 logs</code> montre ce qu’il a écrit.
start-step-4 = <b>Arrêtez-le</b> quand vous voulez : <code>333 stop</code>. Il reste arrêté après un redémarrage jusqu’à ce que vous relanciez <code>333 start</code>.
start-watch = Pour le voir travailler, <code>333 run</code> le fait tourner dans ce terminal avec son écran ; <code>q</code> l’arrête. <code>333</code> seul dit dans quel état est votre nœud et quoi taper ensuite, et <code>333 help</code> liste toutes les commandes.
start-begin = S’il n’y a personne sur le tableau et que le nœud de ce site a disparu, quelqu’un doit être le premier : <code>333 begin</code> refuse tant qu’il y a quelqu’un, et sinon commence une lignée à vous, que personne n’a signée, ce que quiconque lit votre registre peut voir.
start-rule = Rien ne démarre avant que vous ne le démarriez.
start-rule-detail = L’installateur met un fichier en place et n’exécute rien. Votre nœud répond pour la première fois à quelqu’un, et laisse pour la première fois son adresse au point de rencontre, quand vous lancez vous-même <code>333 start</code> ou <code>333 run</code>, et cette commande est le moment où vous avez accepté.

# Take the program: the installers, every file, and the first commands.

start-meta-title = Akiri la programon · 333
start-meta-description = Instalu la klienton de 333 en Linukso, macOS, Vindozo aŭ Raspberry Pi, kontrolitan kontraŭ la SHA256SUMS de la eldono, kaj ricevu la dosieron.

start-heading = Akiri la programon
start-lede = 333 estas malgranda programo, kiun vi rulas sur komputilo, sen kiu vi povas esti, kaj kiu respondas tie, kiam ajn alia kopio de ĝi frapas.
start-pick-file = La dosiero por ĉi tiu maŝino estas <code data-name="standard">333-x86_64-linux</code>. Light, la sama kliento sen la terminala ekrano, estas <code data-name="light">333-light-x86_64-linux</code>. Ambaŭ portas Tor.
start-machine = Via maŝino
start-pick-linux-x86_64 = Linukso, labortabla aŭ servila
start-pick-linux-aarch64 = Raspberry Pi 3, 4 aŭ 5 kun 64 bitoj, aŭ alia 64-bita ARM-Linukso
start-pick-linux-armv6 = Raspberry Pi Zero, aŭ ajna Pi kun 32 bitoj
start-pick-macos-aarch64 = Mac, Apple Silicon
start-pick-macos-x86_64 = Mac, Intel
start-pick-windows = Vindozo
start-form = Formo
start-standard = Standard
start-light = Light
start-unix-title = En Linukso aŭ en Mac
start-unix-installer = En terminalo. La instalilo elektas la dosieron por la maŝino, sur kiu ĝi ruliĝas, kontrolas ĝin kontraŭ la <code>SHA256SUMS</code> de la eldono kaj metas ĝin en <code>~/.local/bin/333</code>. Ĝi lanĉas nenion kaj bezonas neniun pasvorton. <a href="/install.sh">Legu ĝin unue</a>; ĝi estas mallonga.
start-unix-light = Por Light, finu la komandon per <code>sh -s -- --light</code> anstataŭ <code>sh</code>.
start-by-hand = Aŭ permane, sen la instalilo
start-unix-by-hand = La samaj paŝoj, unu post alia. La linio <code>grep</code> presas <code>OK</code> antaŭ ol io ajn moviĝas, kaj se ne, nenio post ĝi ruliĝas.
start-unix-mac = Tio estas Linukso sur x86-64. En Mac, uzu <code>shasum -a 256</code> anstataŭ <code>sha256sum</code> kaj la Mac-dosieron el la tabelo sube; aliloke, la nomon el la tabelo.
start-windows-title = En Vindozo
start-windows-installer = En PowerShell. La instalilo prenas la Vindozan dosieron, kontrolas ĝin kontraŭ la <code>SHA256SUMS</code> de la eldono kaj metas ĝin en <code>%LOCALAPPDATA%\Programs\333\333.exe</code>. Ĝi lanĉas nenion, bezonas neniun administranton kaj aldonas tiun dosierujon al via PATH nur se vi aldonas <code>-AddToPath</code>. <a href="/install.ps1">Legu ĝin unue</a>; ĝi estas mallonga.
start-windows-light = Por Light, rulu ĝin anstataŭe tiel. <code>-AddToPath</code> iras same ĉe la fino.
start-windows-by-hand = La samaj paŝoj, unu post alia. La dosiero estas metata en sian lokon nur se ĝia haketaĵo estas tiu, kiun <code>SHA256SUMS</code> donas, kaj la lasta linio diras, kio okazis.
start-files-title = Ĉiuj dosieroj
start-files-intro = Por ĉiu, por kiu la supra diveno eraras, aŭ kiu preferas preni la dosieron per retumilo. Standard havas la terminalan ekranon, Light ne, kaj ambaŭ portas Tor. Ili gamas de 15 megabajtoj en Mac ĝis 24 en 64-bita ARM.
start-files-machine = Via maŝino
start-files-forms = Standard, poste Light
start-row-linux-x86_64 = Linukso, labortabla aŭ servila
start-row-linux-aarch64 = Raspberry Pi 3, 4 aŭ 5 kun 64 bitoj, kaj alia 64-bita ARM-Linukso
start-row-linux-armv6 = Raspberry Pi Zero, aŭ ajna Pi kun 32 bitoj
start-row-macos-aarch64 = Mac, Apple Silicon
start-row-macos-x86_64 = Mac, Intel
start-row-windows = Vindozo
start-files-after = Dosiero prenita per retumilo tamen devas esti kontrolita kontraŭ <a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a>, farita rulebla kaj metita en vian PATH; ĝuste tion faras la supraj komandoj. Nenio ĉi tie estas subskribita per programista atestilo, do Mac aŭ Vindozo povas rifuzi malfermi dosieron el la retumilo, ĝis vi diros alie. Se nenio el tio taŭgas, aŭ se vi preferas ne ruli dosieron, kiun iu alia konstruis, la <a href="https://github.com/needmoretruth/333#install">deponejo</a> diras, kiel konstrui ĝin en Ubuntu, Debian, Fedora, Arch, macOS kaj Vindozo.
start-then-title = Poste, laŭorde
start-step-1 = <b>Ricevu la dosieron.</b> La nodo de ĉi tiu retejo estas maldorma ĉiuhore:
start-step-1-after = La unua <code>join</code> ankaŭ kreas la nomon de via nodo, ŝlosilon, kies nomo komenciĝas per 333, kio daŭras momenton. Ĉiu adreso sur <a href="{ $base }/333">la afiŝtabulo</a> funkcias same.
start-step-2 = <b>Lanĉu ĝin.</b>
start-step-2-after = De nun ĝi ruliĝas fone, ankaŭ post elsaluto kaj post restartigo. Se ĝi diras <code>shut</code>, via enkursigilo tenas aliajn ekstere: <code>333 start --tor</code> bezonas neniun ŝanĝon al la enkursigilo.
start-step-3 = <b>Kontrolu ĝin.</b> <code>333 status</code> diras, ĉu ĝi ruliĝas, kie aliaj atingas ĝin kaj kiom da ni respondas. <code>333 logs</code> montras, kion ĝi skribis.
start-step-4 = <b>Haltigu ĝin</b>, kiam ajn vi volas: <code>333 stop</code>. Ĝi restas haltigita post restartigo, ĝis vi denove rulos <code>333 start</code>.
start-watch = Por anstataŭe rigardi ĝin labori, <code>333 run</code> rulas ĝin en ĉi tiu terminalo kun ĝia ekrano; <code>q</code> haltigas ĝin. <code>333</code> sola diras, en kia stato estas via nodo kaj kion tajpi poste, kaj <code>333 help</code> listigas ĉiujn komandojn.
start-begin = Se neniu estas sur la afiŝtabulo kaj la nodo de ĉi tiu retejo malaperis, iu devas esti la unua: <code>333 begin</code> rifuzas, dum iu ajn estas tie, kaj alie komencas propran linion, por kiu neniu subskribis, kion ĉiu, kiu legas vian registron, povas vidi.
start-rule = Nenio komenciĝas, antaŭ ol vi komencas ĝin.
start-rule-detail = La instalilo metas dosieron en sian lokon kaj rulas nenion. Via nodo unuafoje respondas al iu kaj unuafoje lasas sian adreson ĉe la renkontejo, kiam vi mem rulas <code>333 start</code> aŭ <code>333 run</code>, kaj tiu komando estas la momento, kiam vi konsentis.

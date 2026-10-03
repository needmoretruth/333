# Take the program: the installers, every file, and the first commands.

start-meta-title = Das Programm holen · 333
start-meta-description = Installiere den 333-Client unter Linux, macOS, Windows oder auf einem Raspberry Pi, geprüft gegen die SHA256SUMS des Releases, und bekomme die Datei.

start-heading = Das Programm holen
start-lede = 333 ist ein kleines Programm, das du auf einem Rechner laufen lässt, den du entbehren kannst, und das dort antwortet, wann immer eine andere Kopie von ihm anklopft.
start-pick-file = Die Datei für diese Maschine ist <code data-name="standard">333-x86_64-linux</code>. Light, derselbe Client ohne den Terminalbildschirm, ist <code data-name="light">333-light-x86_64-linux</code>. Beide bringen Tor mit.
start-machine = Deine Maschine
start-pick-linux-x86_64 = Linux, Desktop oder Server
start-pick-linux-aarch64 = Raspberry Pi 3, 4 oder 5 mit 64 Bit, oder anderes 64-Bit-ARM-Linux
start-pick-linux-armv6 = Raspberry Pi Zero, oder jeder Pi mit 32 Bit
start-pick-macos-aarch64 = Mac, Apple Silicon
start-pick-macos-x86_64 = Mac, Intel
start-pick-windows = Windows
start-form = Form
start-standard = Standard
start-light = Light
start-unix-title = Unter Linux oder auf einem Mac
start-unix-installer = In einem Terminal. Der Installer wählt die Datei für die Maschine, auf der er läuft, prüft sie gegen die <code>SHA256SUMS</code> des Releases und legt sie unter <code>~/.local/bin/333</code> ab. Er startet nichts und braucht kein Passwort. <a href="/install.sh">Lies ihn vorher</a>; er ist kurz.
start-unix-light = Für Light beende den Befehl mit <code>sh -s -- --light</code> statt <code>sh</code>.
start-by-hand = Oder von Hand, ohne den Installer
start-unix-by-hand = Dieselben Schritte, einer nach dem anderen. Die <code>grep</code>-Zeile gibt <code>OK</code> aus, bevor sich irgendetwas bewegt, und wenn nicht, läuft nichts danach.
start-unix-mac = Das ist Linux auf x86-64. Auf einem Mac nimm <code>shasum -a 256</code> statt <code>sha256sum</code> und die Mac-Datei aus der Tabelle unten; überall sonst den Namen aus der Tabelle.
start-windows-title = Unter Windows
start-windows-installer = In PowerShell. Der Installer holt die Windows-Datei, prüft sie gegen die <code>SHA256SUMS</code> des Releases und legt sie unter <code>%LOCALAPPDATA%\Programs\333\333.exe</code> ab. Er startet nichts, braucht keinen Administrator und fügt diesen Ordner nur dann deinem PATH hinzu, wenn du <code>-AddToPath</code> anhängst. <a href="/install.ps1">Lies ihn vorher</a>; er ist kurz.
start-windows-light = Für Light führe ihn stattdessen so aus. <code>-AddToPath</code> kommt genauso ans Ende.
start-windows-by-hand = Dieselben Schritte, einer nach dem anderen. Die Datei wird nur an ihren Platz gelegt, wenn ihr Hash der ist, den <code>SHA256SUMS</code> angibt, und die letzte Zeile sagt, was geschah.
start-files-title = Alle Dateien
start-files-intro = Für alle, bei denen die Vermutung oben falsch liegt oder die die Datei lieber über den Browser holen. Standard hat den Terminalbildschirm, Light nicht, und beide bringen Tor mit. Sie reichen von 15 Megabyte auf dem Mac bis 24 auf 64-Bit-ARM.
start-files-machine = Deine Maschine
start-files-forms = Standard, dann Light
start-row-linux-x86_64 = Linux, Desktop oder Server
start-row-linux-aarch64 = Raspberry Pi 3, 4 oder 5 mit 64 Bit, und anderes 64-Bit-ARM-Linux
start-row-linux-armv6 = Raspberry Pi Zero, oder jeder Pi mit 32 Bit
start-row-macos-aarch64 = Mac, Apple Silicon
start-row-macos-x86_64 = Mac, Intel
start-row-windows = Windows
start-files-after = Eine über den Browser geholte Datei muss trotzdem gegen <a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a> geprüft, ausführbar gemacht und in deinen PATH gelegt werden; genau das tun die Befehle oben. Nichts hier ist mit einem Entwicklerzertifikat signiert, deshalb kann ein Mac oder Windows sich weigern, eine Datei aus dem Browser zu öffnen, bis du es anders sagst. Wenn nichts davon passt oder du lieber keine Datei ausführst, die jemand anderes gebaut hat, steht im <a href="https://github.com/needmoretruth/333#install">Repository</a>, wie man es unter Ubuntu, Debian, Fedora, Arch, macOS und Windows baut.
start-then-title = Dann, der Reihe nach
start-step-1 = <b>Bekomme die Datei.</b> Der Knoten dieser Seite ist zu jeder Stunde wach:
start-step-1-after = Das erste <code>join</code> erzeugt auch den Namen deines Knotens, einen Schlüssel, dessen Name mit 333 beginnt, was einen Moment dauert. Jede Adresse auf <a href="{ $base }/333">dem Brett</a> funktioniert genauso.
start-step-2 = <b>Starte ihn.</b>
start-step-2-after = Er läuft ab jetzt im Hintergrund, auch nach dem Abmelden und nach einem Neustart. Wenn er <code>shut</code> sagt, hält dein Router andere draußen: <code>333 start --tor</code> braucht keine Änderung am Router.
start-step-3 = <b>Sieh nach ihm.</b> <code>333 status</code> sagt, ob er läuft, wo andere ihn erreichen und wie viele von uns antworten. <code>333 logs</code> zeigt, was er geschrieben hat.
start-step-4 = <b>Stoppe ihn</b>, wann immer du willst: <code>333 stop</code>. Er bleibt nach einem Neustart gestoppt, bis du wieder <code>333 start</code> ausführst.
start-watch = Um ihm stattdessen bei der Arbeit zuzusehen, lässt <code>333 run</code> ihn in diesem Terminal mit seinem Bildschirm laufen; <code>q</code> stoppt ihn. <code>333</code> allein sagt, in welchem Zustand dein Knoten ist und was du als Nächstes tippen kannst, und <code>333 help</code> listet alle Befehle.
start-begin = Wenn niemand auf dem Brett steht und der Knoten dieser Seite verschwunden ist, muss jemand der Erste sein: <code>333 begin</code> weigert sich, solange jemand da ist, und beginnt sonst eine eigene Linie, für die niemand signiert hat, was jeder sehen kann, der deinen Beleg liest.
start-rule = Nichts startet, bevor du es startest.
start-rule-detail = Der Installer legt eine Datei an ihren Platz und führt nichts aus. Dein Knoten antwortet zum ersten Mal jemandem und hinterlässt zum ersten Mal seine Adresse am Treffpunkt, wenn du selbst <code>333 start</code> oder <code>333 run</code> ausführst, und dieser Befehl ist der Moment, in dem du zugestimmt hast.

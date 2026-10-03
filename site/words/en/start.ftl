# Take the program: the installers, every file, and the first commands.

start-meta-title = Take the program · 333
start-meta-description = Install the 333 client on Linux, macOS, Windows or a Raspberry Pi, checked against the release's SHA256SUMS, and be handed the file.

start-heading = Take the program
start-lede = 333 is a small program you leave running on a computer you can spare, where it answers whenever another copy of it knocks.
start-pick-file = The file for that machine is <code data-name="standard">333-x86_64-linux</code>. Light, the same client without the terminal screen, is <code data-name="light">333-light-x86_64-linux</code>. Both carry Tor.
start-machine = Your machine
start-pick-linux-x86_64 = Linux, desktop or server
start-pick-linux-aarch64 = Raspberry Pi 3, 4 or 5 on 64-bit, or other 64-bit ARM Linux
start-pick-linux-armv6 = Raspberry Pi Zero, or any Pi on 32-bit
start-pick-macos-aarch64 = Mac, Apple silicon
start-pick-macos-x86_64 = Mac, Intel
start-pick-windows = Windows
start-form = Form
start-standard = Standard
start-light = Light
start-unix-title = On Linux or a Mac
start-unix-installer = In a terminal. The installer picks the file for the machine it runs on, checks it against the release's <code>SHA256SUMS</code>, and puts it at <code>~/.local/bin/333</code>. It starts nothing and needs no password. <a href="/install.sh">Read it first</a>; it is short.
start-unix-light = For Light, end it with <code>sh -s -- --light</code> in place of <code>sh</code>.
start-by-hand = Or by hand, without the installer
start-unix-by-hand = The same steps, one at a time. The <code>grep</code> line prints <code>OK</code> before anything moves, and nothing after it runs if it does not.
start-unix-mac = That is Linux on x86-64. On a Mac, use <code>shasum -a 256</code> in place of <code>sha256sum</code>, and the Mac file from the table below; anywhere else, the name from the table.
start-windows-title = On Windows
start-windows-installer = In PowerShell. The installer fetches the Windows file, checks it against the release's <code>SHA256SUMS</code>, and puts it at <code>%LOCALAPPDATA%\Programs\333\333.exe</code>. It starts nothing, needs no administrator, and adds that folder to your PATH only if you add <code>-AddToPath</code>. <a href="/install.ps1">Read it first</a>; it is short.
start-windows-light = For Light, run it this way instead. <code>-AddToPath</code> goes at the end in the same way.
start-windows-by-hand = The same steps, one at a time. The file is moved into place only if its hash is the one <code>SHA256SUMS</code> gives, and the last line says which happened.
start-files-title = Every file
start-files-intro = For anybody the guess above is wrong for, or who would rather take the file through the browser. Standard has the terminal screen, Light does not, and both carry Tor. They run from 15 megabytes on a Mac to 24 on 64-bit ARM.
start-files-machine = Your machine
start-files-forms = Standard, then Light
start-row-linux-x86_64 = Linux, desktop or server
start-row-linux-aarch64 = Raspberry Pi 3, 4 or 5 on 64-bit, and other 64-bit ARM Linux
start-row-linux-armv6 = Raspberry Pi Zero, or any Pi on 32-bit
start-row-macos-aarch64 = Mac, Apple silicon
start-row-macos-x86_64 = Mac, Intel
start-row-windows = Windows
start-files-after = A file taken through the browser still has to be checked against <a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a>, made executable, and put on your PATH, which is what the commands above do. Nothing here is signed by a developer certificate, so a Mac or Windows may refuse to open a file that came through the browser until you tell it otherwise. If none of these fits, or you would rather not run a file somebody else built, the <a href="https://github.com/needmoretruth/333#install">repository</a> says how to build it on Ubuntu, Debian, Fedora, Arch, macOS and Windows.
start-then-title = Then, in order
start-step-1 = <b>Be handed the file.</b> This site's node is awake at all hours:
start-step-1-after = The first <code>join</code> also makes your node's name, a key whose name begins with 333, which takes a moment. Any address on <a href="{ $base }/333">the board</a> works the same way.
start-step-2 = <b>Start it.</b>
start-step-2-after = It runs in the background from now on, after you log out and after a reboot. If it says <code>shut</code>, your router keeps others out: <code>333 start --tor</code> needs no router change.
start-step-3 = <b>Check on it.</b> <code>333 status</code> says whether it is running, where others can reach it, and how many of us are answering. <code>333 logs</code> shows what it wrote.
start-step-4 = <b>Stop it</b> whenever you want: <code>333 stop</code>. It stays stopped after a reboot until you run <code>333 start</code> again.
start-watch = To watch it work instead, <code>333 run</code> runs it in this terminal with its screen; <code>q</code> stops it. <code>333</code> on its own says what state your node is in and what to type next, and <code>333 help</code> lists every command.
start-begin = If nobody is on the board and this site's node is gone, somebody has to be first: <code>333 begin</code> refuses while anybody is there, and otherwise starts a line of your own, with nobody having signed for it, which anybody reading your record can see.
start-rule = Nothing starts until you start it.
start-rule-detail = The installer puts one file in place and runs nothing. Your node first answers anybody, and first leaves its address at the meeting point, when you run <code>333 start</code> or <code>333 run</code> yourself, and that command is the moment you agreed.

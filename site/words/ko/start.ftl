# Take the program: the installers, every file, and the first commands.

start-meta-title = 프로그램 받기 · 333
start-meta-description = 333 클라이언트를 Linux, macOS, Windows, Raspberry Pi에 설치하고 릴리스의 SHA256SUMS로 확인한 뒤, 파일을 건네받습니다.

start-heading = 프로그램 받기
start-lede = 333은 남는 컴퓨터에 켜 두는 작은 프로그램입니다. 다른 사본이 찾아올 때마다 그 자리에서 응답합니다.
start-pick-file = 그 컴퓨터용 파일은 <code data-name="standard">333-x86_64-linux</code>입니다. 터미널 화면이 없는 같은 클라이언트인 Light는 <code data-name="light">333-light-x86_64-linux</code>입니다. 둘 다 Tor를 품고 있습니다.
start-machine = 내 컴퓨터
start-pick-linux-x86_64 = Linux, 데스크톱이나 서버
start-pick-linux-aarch64 = 64비트 Raspberry Pi 3, 4, 5 또는 다른 64비트 ARM Linux
start-pick-linux-armv6 = Raspberry Pi Zero 또는 32비트로 쓰는 모든 Pi
start-pick-macos-aarch64 = Mac, Apple 실리콘
start-pick-macos-x86_64 = Mac, Intel
start-pick-windows = Windows
start-form = 형태
start-standard = Standard
start-light = Light
start-unix-title = Linux나 Mac에서
start-unix-installer = 터미널에서 실행합니다. 설치 프로그램은 실행되는 컴퓨터에 맞는 파일을 골라 릴리스의 <code>SHA256SUMS</code>로 확인하고 <code>~/.local/bin/333</code>에 둡니다. 아무것도 켜지 않고 비밀번호도 묻지 않습니다. <a href="/install.sh">먼저 읽어 보세요</a>. 짧습니다.
start-unix-light = Light를 받으려면 끝의 <code>sh</code>를 <code>sh -s -- --light</code>로 바꿉니다.
start-by-hand = 설치 프로그램 없이 손으로
start-unix-by-hand = 같은 단계를 하나씩 합니다. <code>grep</code> 줄은 무엇이든 옮기기 전에 <code>OK</code>를 출력하고, 그러지 않으면 그 뒤는 하나도 실행되지 않습니다.
start-unix-mac = 위는 x86-64 Linux 기준입니다. Mac에서는 <code>sha256sum</code> 대신 <code>shasum -a 256</code>을, 파일은 아래 표의 Mac 파일을 씁니다. 다른 곳에서는 표의 이름을 씁니다.
start-windows-title = Windows에서
start-windows-installer = PowerShell에서 실행합니다. 설치 프로그램은 Windows 파일을 받아 릴리스의 <code>SHA256SUMS</code>로 확인하고 <code>%LOCALAPPDATA%\Programs\333\333.exe</code>에 둡니다. 아무것도 켜지 않고 관리자 권한도 필요 없으며, <code>-AddToPath</code>를 붙일 때만 그 폴더를 PATH에 더합니다. <a href="/install.ps1">먼저 읽어 보세요</a>. 짧습니다.
start-windows-light = Light는 대신 이렇게 실행합니다. <code>-AddToPath</code>는 똑같이 끝에 붙입니다.
start-windows-by-hand = 같은 단계를 하나씩 합니다. 파일은 해시가 <code>SHA256SUMS</code>에 적힌 값과 같을 때만 제자리로 옮겨지고, 마지막 줄이 어느 쪽이었는지 말합니다.
start-files-title = 모든 파일
start-files-intro = 위의 추측이 틀린 사람, 또는 브라우저로 파일을 받고 싶은 사람을 위한 표입니다. Standard에는 터미널 화면이 있고 Light에는 없으며, 둘 다 Tor를 품고 있습니다. 크기는 Mac의 15메가바이트부터 64비트 ARM의 24메가바이트까지입니다.
start-files-machine = 내 컴퓨터
start-files-forms = Standard, 그다음 Light
start-row-linux-x86_64 = Linux, 데스크톱이나 서버
start-row-linux-aarch64 = 64비트 Raspberry Pi 3, 4, 5와 다른 64비트 ARM Linux
start-row-linux-armv6 = Raspberry Pi Zero 또는 32비트로 쓰는 모든 Pi
start-row-macos-aarch64 = Mac, Apple 실리콘
start-row-macos-x86_64 = Mac, Intel
start-row-windows = Windows
start-files-after = 브라우저로 받은 파일도 <a href="https://github.com/needmoretruth/333/releases/latest/download/SHA256SUMS">SHA256SUMS</a>로 확인하고, 실행 권한을 주고, PATH에 두어야 합니다. 위의 명령이 하는 일이 그것입니다. 여기 있는 어떤 파일도 개발자 인증서로 서명되지 않았으므로, Mac이나 Windows는 브라우저로 받은 파일을 직접 허락하기 전까지 열지 않을 수 있습니다. 맞는 파일이 없거나 남이 빌드한 파일을 실행하고 싶지 않다면, <a href="https://github.com/needmoretruth/333#install">저장소</a>에 Ubuntu, Debian, Fedora, Arch, macOS, Windows에서 빌드하는 방법이 있습니다.
start-then-title = 그다음, 순서대로
start-step-1 = <b>파일을 건네받습니다.</b> 이 사이트의 노드는 밤낮없이 깨어 있습니다:
start-step-1-after = 첫 <code>join</code>은 노드의 이름도 만듭니다. 이름이 333으로 시작하는 키라서 잠시 걸립니다. <a href="{ $base }/333">게시판</a>의 어느 주소든 같은 방법으로 쓸 수 있습니다.
start-step-2 = <b>켭니다.</b>
start-step-2-after = 이제부터 로그아웃한 뒤에도, 재부팅한 뒤에도 백그라운드에서 돕니다. <code>shut</code>이라고 나오면 라우터가 다른 노드를 막고 있는 것입니다. <code>333 start --tor</code>는 라우터를 바꾸지 않아도 됩니다.
start-step-3 = <b>살펴봅니다.</b> <code>333 status</code>는 노드가 돌고 있는지, 다른 노드가 어디로 닿을 수 있는지, 응답하는 노드가 몇인지 알려 줍니다. <code>333 logs</code>는 노드가 쓴 기록을 보여 줍니다.
start-step-4 = 언제든 <b>끕니다</b>: <code>333 stop</code>. 다시 <code>333 start</code>를 실행할 때까지 재부팅한 뒤에도 꺼진 채로 있습니다.
start-watch = 노드가 일하는 모습을 보려면 <code>333 run</code>이 이 터미널에서 화면과 함께 노드를 돌립니다. <code>q</code>로 끕니다. 명령 없이 <code>333</code>만 치면 노드의 상태와 다음에 칠 명령을 알려 주고, <code>333 help</code>는 모든 명령을 보여 줍니다.
start-begin = 게시판에 아무도 없고 이 사이트의 노드도 사라졌다면 누군가 처음이 되어야 합니다. <code>333 begin</code>은 누가 있으면 거절하고, 없으면 아무도 서명하지 않은 자기 계보를 시작합니다. 기록을 읽는 누구나 그것을 볼 수 있습니다.
start-rule = 직접 켜기 전에는 아무것도 시작되지 않습니다.
start-rule-detail = 설치 프로그램은 파일 하나를 제자리에 두고 아무것도 실행하지 않습니다. 노드가 처음 누군가에게 응답하고 처음 만남의 장소에 주소를 남기는 때는 직접 <code>333 start</code>나 <code>333 run</code>을 실행할 때이고, 그 명령이 동의한 순간입니다.

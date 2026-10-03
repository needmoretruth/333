### `333 --help`: 각 명령과 옵션이 하는 일.
###
### 문단 안의 줄바꿈은 공백으로 읽고, 빈 줄은 새 문단을 시작합니다.

help-about = 333의 노드 하나. 물으면 답하고, 기록을 남기고, 파일을 건넵니다.

help-id = 이 노드의 이름을 보여 줍니다. 처음 실행하면 이름을 만듭니다

help-bootstrap = 파일을 건네줄 노드가 아무도 없을 때 새 계보를 시작합니다

help-bootstrap-long = 파일을 건네줄 노드가 아무도 없을 때 새 계보를 시작합니다.

    보통은 초대장으로 `333 join`을 합니다. 이 명령은 먼저 만남의 장소를
    보고, 누가 있으면 거절합니다. 아무도 없으면 파일을 받아 이 클라이언트에
    든 해시와 맞춰 보고 저장합니다. 이 노드는 새 계보의 창시자가 되고,
    시작에 아무의 서명도 없다는 것은 기록을 읽는 누구나 볼 수 있습니다.

help-serve = 이 터미널에서 노드를 켭니다. 끌 때까지 돕니다

help-serve-long = 이 터미널에서 노드를 켭니다. 끌 때까지 돕니다.

    하트비트와 질문에 답하고, 아는 것을 주고받고, 에포크마다 뽑힌
    노드에게 묻습니다. 터미널에서는 화면을 엽니다. `q`, Ctrl-C, 또는 다른
    터미널의 `333 stop`으로 끕니다. 백그라운드에서 켜려면 `333 start`.

help-serve-long-light = 이 터미널에서 노드를 켭니다. 끌 때까지 돕니다.

    하트비트와 질문에 답하고, 아는 것을 주고받고, 에포크마다 뽑힌
    노드에게 묻고, 한 줄씩 기록합니다. Ctrl-C나 다른 터미널의
    `333 stop`으로 끕니다. 백그라운드에서 켜려면 `333 start`.

help-say = 333 가운데 하나를 말합니다. 에포크마다 한 번, 숫자로 전해집니다

help-status = 켜져 있는지, 밖에서 닿는 주소, 응답 중인 노드 수를 보여 줍니다

help-join = 초대장으로 파일을 가진 노드에게서 파일을 받습니다

help-languages = 언어 목록을 보이거나, 이 노드에서 쓸 언어를 저장합니다

help-ping = 다른 노드에 연결해 하트비트를 한 번 주고받습니다

help-pack = 이 노드를 파일 하나에 담습니다. 다른 기계로 옮길 때 씁니다

help-pack-long = 이 노드를 파일 하나에 담습니다. 다른 기계로 옮길 때 씁니다.

    이름, 기록, 다른 노드가 서명한 것, 파일, 어니언 주소의 키가 모두
    들어갑니다. 그 뒤 이 디렉터리에서는 노드가 켜지지 않아 같은 이름이 두
    곳에 생기지 않습니다. 암호화하지 않으니, 파일을 가진 사람이 곧 이
    노드입니다. 옮기고, 풀고, 지우십시오.

help-unpack = 포장한 노드를 이 기계의 노드 디렉터리에 풉니다

help-unpack-long = 포장한 노드를 이 기계의 노드 디렉터리에 풉니다.

    이미 노드가 있으면 거절합니다. 파일을 끝까지 읽고 키와 기록을 확인한
    뒤에야 씁니다.

help-moved = 노드 디렉터리를 복사하지 않고 옮기거나 이름만 바꿨다고 적습니다

help-moved-long = 노드 디렉터리를 복사하지 않고 옮기거나 이름만 바꿨다고 적습니다.

    자리가 바뀐 노드는 이 명령을 칠 때까지 실행할 때마다 알립니다. 복사본과
    원본이 함께 돌면 같은 이름이 두 곳에 생기기 때문입니다.

help-tell = 실행 중인 노드에 화면에서 쓰는 말로 명령을 보냅니다

help-tell-long = 실행 중인 노드에 화면에서 쓰는 말로 명령을 보냅니다.

    `tor on`, `tor off`, `bridge <줄>`, `helper <프로그램>` 등 화면에서 `:`
    뒤에 쓰는 말이면 됩니다. 실행 중인 노드가 수행하고 답이 여기 나옵니다.
    `say`, `join`, `ping`, `begin`, `status`, `stop`은 이 명령 없이도 실행
    중인 노드로 갑니다.

help-tell-light = 실행 중인 노드에 명령을 보냅니다

help-tell-long-light = 실행 중인 노드에 명령을 보냅니다.

    `tor on`, `tor off`, `bridge <줄>`, `helper <프로그램>`. 실행 중인
    노드가 수행하고 답이 여기 나옵니다. `say`, `join`, `ping`, `begin`,
    `status`, `stop`은 이 명령 없이도 실행 중인 노드로 갑니다.

help-service = 백그라운드 서비스를 직접 다룹니다(`start`와 `stop`이 이것을 씁니다)

help-service-long = 백그라운드 서비스를 직접 다룹니다(`start`와 `stop`이 이것을 씁니다).

    요청하기 전에는 아무것도 설치하지 않고, 쓰는 파일과 실행하는 명령을
    모두 출력합니다. `333 service uninstall`이 전부 지웁니다.

help-service-install = 이 실행 옵션으로 백그라운드 서비스를 설치하고 켭니다

help-service-install-long = 이 실행 옵션으로 백그라운드 서비스를 설치하고 켭니다.

    서비스는 이 노드의 디렉터리로 `333 run`을 주어진 옵션 그대로
    실행합니다. 한 시간마다 확인해 노드가 멈추면 이 기계에 알립니다.
    옵션 없이 하려면 `333 start`.

help-service-uninstall = 백그라운드 서비스를 끄고 설치한 것을 모두 지웁니다

help-service-status = 서비스 관리자가 말하는 상태, 노드가 마지막으로 깨어 있던 때, 마지막
    기록

help-service-check = 노드가 멈췄으면 이 기계에 알립니다. 서비스가 한 시간마다 실행하고,
    문제가 없으면 아무 말도 하지 않습니다

help-data-dir = 이 노드의 모든 것(이름, Tor를 쓰면 Tor 상태)이 든 디렉터리

help-timeout = 네트워크를 쓰는 한 단계마다 기다리는 최대 초

help-timeout-long = 네트워크를 쓰는 한 단계마다 기다리는 최대 초.

    지연이 아니라 상한입니다. 몇 분 걸릴 수 있는 Tor 시작에 맞춘 값입니다.

help-dangerously-trust-directory-permissions = 이 기계의 다른 계정이 들어올 수 있는 디렉터리도 받아들입니다

help-dangerously-trust-directory-permissions-long = 이 기계의 다른 계정이 들어올 수 있는 디렉터리도 받아들입니다.

    그 디렉터리에 이 노드 이름의 유일한 사본이 있어서, 권한이 느슨하면
    기본으로 거절합니다. 임시 디렉터리나 소유자가 특이한 컨테이너용입니다.

help-keep-everything = 판정 창이 지난 진술도 지우지 않고 모두 남깁니다

help-keep-everything-long = 판정 창이 지난 진술도 지우지 않고 모두 남깁니다.

    누구의 자리도 바뀌지 않습니다. 진술은 어디에 두든 똑같이 검증됩니다.

help-bridges = 일반적인 Tor 연결이 막힌 네트워크에서 쓸 브리지 한 줄

help-bridges-long = 일반적인 Tor 연결이 막힌 네트워크에서 쓸 브리지 한 줄.

    받은 브리지마다 한 번씩, 받은 그대로 주십시오. 여기서 브리지를 대신
    구해 오지 않습니다. 목록으로 모을 수 있으면 막히기 때문에 사람이
    직접 나눠 줍니다.

help-bridge-helper = 난독화 브리지를 쓰는 프로그램(이름이나 경로)

help-bridge-helper-long = 난독화 브리지를 쓰는 프로그램(이름이나 경로).

    브리지 줄이 요구하고 경로에 `lyrebird`가 없을 때만 필요합니다. 함께
    넣지 않은 것은 고정된 사본이 금방 낡기 때문입니다.

help-language = 쓸 언어의 태그: `ko`, `es`, `zh-Hant`

help-language-long = 쓸 언어의 태그: `ko`, `es`, `zh-Hant`.

    이 옵션이 없으면 `THE333_LANGUAGE`, 그다음 `333 language <태그>`로
    저장한 언어, 그다음 영어입니다. 시스템 언어는 보지 않습니다.
    `333 language`가 있는 언어를 보여 주고, `<data-dir>/words/<태그>/`에
    문구 폴더를 두면 빌드 없이 언어가 늘어납니다. 333의 낱말은 번역하지
    않습니다.

help-count-in = 십진법(ten), 십이진법(twelve, twelve-ascii) 가운데 하나로 셉니다

help-count-in-long = 십진법(ten), 십이진법(twelve, twelve-ascii) 가운데 하나로 셉니다.

    보이는 수와 입력하는 수가 모두 그 진법입니다. 십이진법의 `say 238`은
    십진법의 `say 332`입니다. 이름, 주소, 포트, 버전은 바꾸지 않고,
    네트워크로 나가는 것도 같습니다. 없으면 `THE333_COUNT_IN`, 그다음
    십진법.

help-bootstrap-meet = 혼자 시작하기 전에 노드를 찾아볼 곳

help-bootstrap-anyway = 이미 누가 있어도 새로 시작합니다

help-serve-bind = 연결을 받을 주소와 포트

help-serve-tor = 어니언 주소도 엽니다. 다른 노드가 이 노드의 위치를 모른 채 닿을 수
    있습니다. Tor를 켜는 데 몇 초에서 몇 분이 걸립니다

help-serve-no-direct = 소켓을 아예 열지 않습니다. --tor와 함께만 쓰며, 주소가 네트워크에
    전혀 나가지 않습니다

help-serve-announce = 다른 노드에게 알릴 이 노드의 주소

help-serve-announce-long = 다른 노드에게 알릴 이 노드의 주소.

    소켓이 스스로 알 수 없을 때 필요합니다. 모든 인터페이스에서 받거나,
    포트를 넘겨주는 장치 뒤에 있을 때입니다.

help-serve-no-mdns = 이 노드가 있다는 것을 같은 네트워크에 알리지 않습니다

help-serve-no-mdns-long = 이 노드가 있다는 것을 같은 네트워크에 알리지 않습니다.

    알릴 때 나가는 것은 이 기계에서 333이 어느 포트로 도는지이고, 이름은
    나가지 않습니다. 한 집의 두 노드가 초대장 없이 서로 찾는 방법입니다.

help-serve-no-router = 공유기에 포트를 이 기계로 보내 달라고 요청하지 않습니다

help-serve-no-router-long = 공유기에 포트를 이 기계로 보내 달라고 요청하지 않습니다.

    가정용 공유기는 안에서 요청하기 전에는 밖에서 오는 연결을 버립니다.
    요청은 UPnP-IGD, PCP, NAT-PMP로 합니다. 네트워크 설정이 바뀌는 일이라
    할 때마다 출력합니다. `--no-upnp`는 같은 옵션의 옛 이름입니다.

help-serve-meet = 아무도 소개해 주지 않은 노드를 찾아볼 곳

help-serve-meet-long = 아무도 소개해 주지 않은 노드를 찾아볼 곳.

    노드들이 서명해 남긴 주소를 모아 두는 고정된 주소 하나입니다.
    거기서 읽은 것은 모두 이 노드가 직접 검증합니다.

help-serve-no-meet = 만남의 장소를 쓰지 않습니다

help-serve-no-meet-long = 만남의 장소를 쓰지 않습니다.

    그러면 초대장을 받은 노드와 같은 네트워크의 노드만 이 노드에 닿습니다.

help-serve-plain = 화면 대신 한 줄씩 기록합니다

help-serve-plain-long = 화면 대신 한 줄씩 기록합니다.

    터미널이 아니면 언제나 줄로 기록합니다. 이 옵션은 터미널에서도 그렇게
    합니다.

help-serve-plain-light = 한 줄씩 기록합니다. 이 판본은 늘 그렇습니다

help-serve-plain-long-light = 한 줄씩 기록합니다. 이 판본은 늘 그렇습니다.

    이 판본에는 화면이 없습니다. 같은 명령줄이 두 판본에서 모두 돌도록
    받아 둡니다.

help-say-index = 0부터 { $last }까지 가운데 하나. 셈 진법(--count-in)으로 씁니다. 뜻은 아직
    정해지지 않았습니다

help-status-sources = 이 노드가 가진 주소를 모두 보입니다: 누구의 것인지, 언제 어디서 처음
    받았는지, 마지막은 어디서인지

help-status-json = 이 노드가 관측한 것을 프로그램이 읽는 JSON으로. 주소와 포트는 들어 있지
    않습니다

help-join-address = 파일을 가진 노드의 초대장(`333:host:port`)

help-ping-address = 초대장(`333:host:port`) 또는 주소: `host`, `host:port`, `[::1]:port`,
    `something.onion`(Tor로 연결)

help-pack-file = 쓸 파일. 아직 없는 파일이어야 합니다

help-pack-undo = 그만둔 옮기기의 포장을 여기서 되돌립니다

help-pack-undo-long = 그만둔 옮기기의 포장을 여기서 되돌립니다.

    그 파일을 어디에도 풀지 않았을 때만 쓰십시오. 풀었다면 같은 노드가
    둘이 됩니다.

help-unpack-file = `333 pack`이 쓴 파일

help-tell-order = 화면에 입력하는 그대로의 명령

help-tell-order-light = `tor on`이나 `bridge <줄>`처럼 쓰는 명령

help-service-install-flags = `run`에 줄 옵션. `run` 뒤에 쓰는 그대로

## clap 자신의 `--help`, `--version`, `help`에 대해 하는 말.

help-print-help = 도움말을 보입니다

help-print-help-more = 도움말을 보입니다(자세히는 '--help')

help-print-help-summary = 도움말을 보입니다(요약은 '-h')

help-print-version = 버전을 보입니다

help-print-this = 이 도움말이나 하위 명령의 도움말을 보입니다

help-print-for = 하위 명령의 도움말을 보입니다

help-start = 백그라운드에서 노드를 켭니다. 재부팅 뒤에도 켜집니다

help-stop = 노드를 끕니다. 재부팅 뒤에도 꺼진 채입니다

help-restart = 노드를 껐다가 백그라운드에서 다시 켭니다

help-logs = 백그라운드에서 돈 노드의 마지막 기록을 보입니다

help-logs-follow = 새 기록이 생기는 대로 계속 보입니다(systemd가 기록을 맡는 곳에서만)

help-invite = 다른 사람이 이 노드를 통해 들어올 때 쓰는 초대장을 보입니다

help-status-all = 이 노드가 아는 모든 것을 각 부분의 뜻과 함께 보입니다

help-languages-tag = 저장할 언어 태그: `ko`, `en`. `en`이면 영어로 돌아갑니다

help-start-example = 예: 333 start

help-stop-example = 예: 333 stop

help-restart-example = 예: 333 restart

help-status-example = 예: 333 status --all

help-logs-example = 예: 333 logs -f

help-id-example = 예: 333 name

help-invite-example = 예: 333 invite

help-bootstrap-example = 예: 333 begin

help-serve-example = 예: 333 run --tor

help-say-example = 예: 333 say 7

help-join-example = 예: 333 join 333:192.0.2.7:3333

help-languages-example = 예: 333 language ko

help-ping-example = 예: 333 ping 333:192.0.2.7:3333

help-pack-example = 예: 333 pack node.333

help-unpack-example = 예: 333 unpack node.333

help-moved-example = 예: 333 moved

help-tell-example = 예: 333 tell tor on

help-service-example = 예: 333 service status

help-service-install-example = 예: 333 service install --tor

help-service-uninstall-example = 예: 333 service uninstall

help-service-status-example = 예: 333 service status

help-service-check-example = 예: 333 service check

help-start-flags = `run`에 줄 옵션. `run` 뒤에 쓰는 그대로. 다른 옵션을 줄 때까지 다음
    `start`에도 그대로 씁니다


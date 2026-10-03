# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = 복사
js-copied = 복사함
js-selected = 선택함
js-state-awake = 이 사이트의 노드가 깨어 있습니다
js-state-not-running = 이 사이트의 노드가 꺼져 있습니다
js-in-hours = { $h }시간 { $m }분 뒤
js-in-minutes = { $m }분 뒤
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = 이 계보의 { $n }번째 에포크

## The network

js-network-state-founder = 어느 명부에도 없음
js-network-state-ok = 이번 에포크에 응답
js-network-state-quiet = 이번 에포크에 침묵
js-network-state-later = 다음 에포크부터 셈
js-network-state-seen = 보였지만 명부에 없음
js-network-awake = 깨어 있음
js-network-not-running = 꺼져 있음
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · 내 노드
js-network-find-bad = 노드 이름은 16진수입니다. 앞 6자 이상을 입력하세요.
js-network-find-none = 이 사이트의 노드는 그 이름의 노드를 본 적이 없습니다.
js-network-find-many = 그렇게 시작하는 노드가 { $count }개입니다. 이름을 더 입력하세요.
js-network-find-marked = 이 기기에서 내 노드로 표시했습니다.
js-network-select = 노드를 고르면 이 사이트의 노드가 그 노드에 대해 아는 것이 나옵니다.
js-network-role-founder = 이 계보의 창시자
js-network-role-site = 이 사이트의 노드
js-network-role-yours = 내 노드, 이 기기에서
js-network-role-none = 네트워크의 노드
js-network-col-name = 이름
js-network-col-state = 이번 에포크
js-network-col-given = 파일 받음
js-network-col-counted = 셈 시작
js-network-col-answered = 마지막 응답
js-network-col-said = 말함
js-network-col-reached = 닿는 길
js-network-row-said = 이번 에포크에 말함
js-network-row-handed = 파일을 건넨 노드
js-network-row-testimony = 증언
js-network-given-by = 에포크 { $epoch }, { $sponsor }에게서
js-network-given-founder = 아무에게서도. 이 계보를 시작했습니다.
js-network-given-none = 명부에 없음
js-network-epoch = 에포크 { $epoch }
js-network-epoch-now = { $epoch } (이번 에포크)
js-network-epoch-ago = { $epoch } ({ $ago } 전)
js-network-more = 그리고 아래 표에 { $count }개 더
js-network-nothing = 없음
js-network-reach-direct = 직접
js-network-reach-tor = Tor로
js-network-reach-tor-short = Tor
js-network-reach-unknown = 알 수 없음
js-network-testimony = { $asked }번 물음을 받고, { $asking }번 물음 (최근 3 에포크)
js-network-copy-name = 이름 복사
js-network-select-name = 위의 이름을 선택하세요
js-network-mine = 내 노드입니다
js-network-tag-founder = 창시자
js-network-tag-site = 이 사이트
js-network-tag-yours = 내 노드
js-network-empty = 이 사이트의 노드는 아직 다른 노드를 보지 못했습니다.
js-network-this-node = 이 노드
js-network-yes = 예
js-network-no = 아니요
js-network-none = 없음

## Where we are

js-map-watch = 실시간으로 보기
js-map-stop = 그만 보기
js-map-read-at = { $read_at } UTC에 읽음.
js-map-unreadable = 지금은 게시판을 읽지 못했습니다.
js-map-tor = Tor
js-map-nowhere = 위치를 알 수 없는 곳
js-map-nobody = 위치를 말하는 노드가 없습니다.
js-map-all = 위치를 말하는 노드 전체
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = 네트워크에 있지만 위치를 말하지 않음
js-map-dot = 노드 { $count }개

## The board

js-board-said = 에포크 { $epoch }에 말함 · { $node }
js-board-site = 이 사이트의 노드
js-board-tor = Tor로

## Take the program

js-start-machine-linux-x86_64 = x86-64 Linux
js-start-machine-linux-aarch64 = 64비트 ARM Linux
js-start-machine-linux-armv6 = 32비트 ARM Linux
js-start-machine-macos-aarch64 = Apple 실리콘 Mac
js-start-machine-macos-x86_64 = Intel 칩 Mac
js-start-machine-windows-x86_64 = Windows
js-start-phone = 휴대폰이나 태블릿으로 보입니다. 이 프로그램은 켜 두는 컴퓨터용입니다. 그 컴퓨터를 여기서 고르세요.
js-start-unknown = 이 브라우저는 어디서 돌고 있는지 알려 주지 않습니다. 컴퓨터를 여기서 고르세요.
js-start-sure = 이 브라우저는 { $machine }에서 돈다고 알려 와서 그것을 골라 두었습니다.
js-start-mac = 이 브라우저는 Mac이라고만 알려 오고 칩은 알려 주지 않아서 Apple 실리콘을 골라 두었습니다. 설치 프로그램은 컴퓨터에 직접 묻습니다.
js-start-linux = 이 브라우저는 Linux라고만 알려 오고 프로세서는 알려 주지 않아서 x86-64를 골라 두었습니다. 설치 프로그램은 컴퓨터에 직접 묻습니다.
js-start-chosen = 위에서 고른 것

## The story on the home page, drawn

js-story-file = 333.txt · 3바이트
js-story-gave = 당신에게 건넸습니다
js-story-received = 당신에게서 받았습니다
js-story-signed = 서명함
js-story-minutes = 333분
js-story-epochs = 333 에포크
js-story-now = 지금
js-story-answering = 응답 중
js-story-roll = 명부
js-story-years = { $years }년

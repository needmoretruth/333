### `333 service`: 요청을 받았을 때, 로그아웃과 재부팅 너머까지 철야를 지키기.
##
## 한 줄이 `{`로 끝나고 다음 줄이 `""}`로 시작하는 곳은, 출력되는 줄이 이 파일의
## 한 줄보다 넓은 곳입니다. 두 줄에 걸친 자리표시는 줄바꿈을 출력하지 않습니다.

service-mind = { $node }은(는) 이 시스템이 비우는 곳에 있고, 이 노드의 이름은
    다른 어디에도 없습니다. 서비스는 그곳이 비워질 때까지 그곳에서
    철야를 지킵니다.
    .keyword = 주의

service-runs = { $command }
    .keyword = 철야

service-undo-partial = `333 service uninstall`이 여기까지 한 것을 무엇이든 지웁니다.
    .keyword = 되돌림

service-no-receipt-directory = 이 시스템에는 설치 기록을 둘 설정 디렉터리가 없습니다

service-wrote-receipt = { $path }. `333 service uninstall`은 이것을 보고 무엇을 되돌릴지
    압니다.
    .keyword = 작성

service-undo = `333 service uninstall`은 철야를 멈추고 위의 일을 모두 되돌립니다.
    노드 자신의 디렉터리는 어느 쪽도 건드리지 않습니다.
    .keyword = 되돌림

service-uninstalled = 이제 서비스가 철야를 지키지 않습니다. { $node }은(는) 철야가 남긴
    그대로 둡니다. `333 serve`로 직접 철야를 지킬 수 있고,
    `333 service install`로 서비스를 다시 설정합니다.
    .keyword = 철야

service-none-installed = 이 사용자에게 `333 service install`로 설치한 서비스가 없습니다.
    .keyword = 서비스

service-state = { $state }
    .keyword = 서비스

service-node = { $node }
    .keyword = 노드

service-last-awake = 마지막으로 { $at }에, { $ago } 전에 깨어 있다고 말했습니다
    .keyword = 깨어있음

service-never-awake = 이 디렉터리에서는 깨어 있다고 말한 적이 없습니다
    .keyword = 깨어있음

service-said-nothing = 보관된 말이 없습니다
    .keyword = 말함

service-said-last = 마지막 { $lines }줄:
    .keyword = 말함

service-no-manager = 이 시스템에는 `333 service`가 물을 줄 아는 서비스 관리자가 {
    ""}없습니다. 여기서 프로그램을 계속 돌리는 것이 무엇이든, 그 아래에서 {
    ""}`333 serve --plain`이 철야를 지킵니다.

service-not-installed-here = 설치되지 않음: 여기에는 이것이 아는 서비스 관리자가 없습니다

# 서비스 관리자마다 자기 파일에서 말합니다.

service-creating = { $path } 만들기
service-writing = { $path } 쓰기
service-removing = { $path } 지우기

service-wrote = { $path }
    .keyword = 작성

service-removed = { $path }
    .keyword = 삭제

service-left = { $path }. `333 service install`이 쓴 것이 아닙니다.
    .keyword = 남김

service-failed = { $why }
    .keyword = 실패

service-not-installed = 설치되지 않음
service-running = 도는 중
service-starting = 시작하는 중

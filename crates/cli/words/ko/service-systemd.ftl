### Linux의 `333 service`: systemd.
##
## 한 줄이 `{`로 끝나고 다음 줄이 `""}`로 시작하는 곳은, 출력되는 줄이 이 파일의
## 한 줄보다 넓은 곳입니다.

service-systemd-no-configuration = 이 시스템에는 이 사용자의 설정 디렉터리가 없습니다

service-systemd-no-session = 여기서 systemd가 이 사용자의 세션을 지키고 있지 않습니다 {
    ""}({ $why }). 이 사용자가 콘솔이나 ssh로 로그인하면 세션이 시작되고, su나 {
    ""}sudo로는 시작되지 않습니다. 이 사용자로 로그인해서 다시 실행하십시오.

service-systemd-wrote-over = { $path }, 원래 있던 것 대신
    .keyword = 작성

service-systemd-linger-already = { $user }에게 이미 켜져 있습니다. 로그아웃한 뒤에도 철야를
    돌게 하고, 아무도 로그인하지 않아도 부팅할 때 철야를 시작하는 것이
    이것입니다.
    .keyword = linger

service-systemd-linger-on = { $user }에게 켰습니다. 로그아웃한 뒤에도 철야를 돌게 하고,
    아무도 로그인하지 않아도 부팅할 때 철야를 시작하는 것이 이것입니다.
    .keyword = linger

service-systemd-linger-not-on = 켜지 못했습니다: { $why }. 이것이 없으면 철야는 로그아웃할 때
    멈추고, 재부팅한 뒤 다시 로그인할 때까지 기다립니다.
    `sudo loginctl enable-linger { $user }`로 켭니다.
    .keyword = linger

service-systemd-linger-off = 설치 전처럼 다시 껐습니다.
    .keyword = linger

service-systemd-linger-left = 그대로 두었습니다. 설치가 켠 것이 아닙니다.
    .keyword = linger

service-systemd-not-answering = 알 수 없음: systemd가 이 사용자에 대해 답하지 않습니다
service-systemd-restarting = 멈췄고, 멈춘 지 333초 뒤에 다시 시작합니다
service-systemd-stopping = 멈추는 중
service-systemd-failed = 실패 ({ $result })
service-systemd-stopped = 멈춤
service-systemd-at-every-boot = { $state }, 부팅할 때마다 시작
service-systemd-not-again = { $state }, { $file_state }: 스스로 다시 시작하지 않습니다

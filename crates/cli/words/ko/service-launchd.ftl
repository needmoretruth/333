### macOS의 `333 service`: launchd.

service-launchd-no-home = 이 시스템에는 이 사용자의 홈 디렉터리가 없습니다
service-launchd-asking-who = `id -u`에게 어느 사용자인지 묻기

service-launchd-login = launchd는 로그인한 순간부터 로그아웃할 때까지 노드를 돌리고,
    재시작 뒤에는 로그인하면 다시 켭니다. 기록은 { $log }에 있습니다.
    .keyword = 로그인

service-launchd-ended-with = { $state }, 그리고 마지막에는 { $code }(으)로 끝났습니다
service-launchd-loaded = 올라와 있지만, launchd가 무엇을 하는 중인지 말하지 않았습니다

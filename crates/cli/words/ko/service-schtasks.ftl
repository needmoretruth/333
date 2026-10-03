### Windows의 `333 service`: 작업 스케줄러.

service-schtasks-cannot-pass = `{ $word }`에 " 또는 %가 있어서, cmd.exe가 그대로 넘길 수 없습니다
service-schtasks-no-user = Windows가 이 사용자가 누구인지 말하지 않았습니다 {
    ""}(%USERDOMAIN%, %USERNAME%)

service-schtasks-logon = Windows는 로그온한 동안 노드를 돌립니다. 부팅부터 도는 서비스는 {
    ""}별도 계정이
    필요한데, 노드는 이 계정의 디렉터리에 있기 때문입니다.
    기록은 { $log }에 있습니다.
    .keyword = 로그온

service-schtasks-ready = 멈췄고, 333초 안에 다시 시작합니다
service-schtasks-disabled = 꺼져 있음: 스스로 다시 시작하지 않습니다

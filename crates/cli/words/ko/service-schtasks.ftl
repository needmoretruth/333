### Windows의 `333 service`: 작업 스케줄러.

service-schtasks-cannot-pass = `{ $word }`에 " 또는 %가 있어서, cmd.exe가 그대로 넘길 수 없습니다
service-schtasks-no-user = Windows가 이 사용자가 누구인지 말하지 않았습니다 {
    ""}(%USERDOMAIN%, %USERNAME%)

service-schtasks-logon = Windows는 로그온한 동안, 로그온하는 순간부터 철야를 지킵니다.
    부팅부터 지키는 서비스는 자기 계정으로 돌 텐데, 노드는 당신의
    디렉터리에 살고 그 계정은 그곳에 볼일이 없습니다.
    철야가 하는 말은 { $log }에 있습니다.
    .keyword = 로그온

service-schtasks-ready = 멈췄고, 333초 안에 다시 시작합니다
service-schtasks-disabled = 꺼져 있음: 스스로 다시 시작하지 않습니다

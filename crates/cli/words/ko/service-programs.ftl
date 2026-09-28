### `333 service`: 시스템 자신의 프로그램을 실행하고, 그렇게 했다고 말하기.

service-programs-ran = { $command }
    .keyword = 실행

service-programs-did-not-succeed = `{ $command }`이(가) 성공하지 못했습니다: { $why }
service-programs-ended-with = { $status }(으)로 끝났습니다
service-programs-no-such = 이 시스템에는 { $program }이(가) 없습니다
service-programs-not-started = { $program }을(를) 시작할 수 없었습니다: { $why }

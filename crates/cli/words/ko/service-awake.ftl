### `333 service`: 도는 철야가 아직 깨어 있다고 적는 줄.

service-awake-failed = 노드가 깨어 있다고 { $root }에 쓰는 중: { $why }. 이것이 다시 될
    때까지 이 기계는 노드가 돌고 있는지 알 수 없습니다.
    .keyword = 실패

service-awake-never-kept = 돌지 않습니다. 서비스는 설치됐지만 노드가 깨어 있다고 한 적이
    없습니다. 이유는 `333 service status`가 보여 줍니다.
    .keyword = 노드

service-awake-not-kept-since = { $at }({ $ago } 전)부터 돌지 않습니다. 이유는 `333 service status`가 보여 줍니다.
    .keyword = 노드

service-awake-under-a-minute = 1분 미만
service-awake-minutes = { $minutes }분
service-awake-epochs = { $epochs }에포크

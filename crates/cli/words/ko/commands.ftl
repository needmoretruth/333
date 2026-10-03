### 명령들이 함께 쓰는 말: 노드를 여는 것, 진술을 주고받는 것, 한 번의 교환, 그리고 시계.

commands-clock-at-zero = { $epoch }. 이 기계의 시계가 1970년을 가리킵니다. 시계를 맞추기
    전에는 어느 노드도 이 노드에 파일을 건네거나 증언하지 않습니다.
    .keyword = 에포크

commands-called-first = 처음 만든 키가 부름을 받았습니다.
    .keyword = 부름

commands-called = 키 { $not_called }개는 부름을 받지 못했고, 이 키가 받았습니다.
    .keyword = 부름

commands-torn = 끝나지 않은 항목 { $bytes }바이트를 기록에서 버렸습니다
    .keyword = 찢김

commands-record = 에포크 { $epochs }개에 답했고, 어느 것도 고칠 수 없습니다
    .keyword = 기록

commands-witnessed = 다른 노드가 이 노드에 대해 서명한 진술 { $statements }개.
    창이 지나면 그 에포크에서 남는 것은 이것뿐이라 계속 보관합니다.
    .keyword = 증언

commands-unseen = 어느 에포크에서도 이 노드에 대해 서명된 것이 없습니다. 밖으로
    연결은 되지만 밖에서 닿지 않고, 셈에 드는 것은 닿는 쪽입니다.
    원인은 둘입니다. 공유기가 포트 3333을 이 기계로 보내지 않거나,
    주소를 아무에게도 알리지 않았습니다. `333 run --tor`나
    `333 start --tor`는 둘 다 필요 없습니다. 어니언 주소는 어떤 공유기
    뒤에서도 닿고, 이 클라이언트에는 Tor가 들어 있습니다.
    .keyword = 미관측

commands-roll-alone = 1명, 이 노드 자신
    .keyword = 명부

commands-roll = { $members }명
    .keyword = 명부

commands-known = 노드 { $addresses }개의 주소를 압니다
    .keyword = 주소

commands-holding = 파일이 있고, 건넬 수 있습니다
    .keyword = 보유

commands-keeping = 모든 진술을 지우지 않고 남깁니다. 셈에는 아무 영향이 없습니다.
    진술은 어디에 두든 똑같이 검증됩니다.
    .keyword = 보관

commands-ignored = 읽을 수 없는 입회 기록 { $admissions }개
    .keyword = 무시

commands-learned-where = 노드 { $addresses }개의 주소를 새로 알았습니다
    .keyword = 배움

commands-rejoined = { $were }명을 알던 노드에게서 { $members }명을 새로 알았습니다.
    나뉘어 있던 두 무리가 이제 한 셈입니다.
    .keyword = 재회

commands-learned-names = { $members }명을 새로 알았습니다
    .keyword = 배움

commands-heard = { $speakers }명이 말한 것을 받았습니다
    .keyword = 들음

commands-carried = 아직 열려 있는 에포크에 대한 진술 { $statements }개
    .keyword = 전달

commands-exchange = { $node }  에포크 { $epoch }  { $clocks }  ({ $liveness })
    .keyword = 증언

commands-answered-the-challenge = 이 노드가 낸 질문에 답함
commands-spoke-first = 먼저 말함. 말했다는 것만 증명됨

commands-clocks-together = 시계 일치
commands-clocks-ahead = 상대 시계가 { $apart } 빠름
commands-clocks-behind = 상대 시계가 { $apart } 늦음
commands-hours-and-minutes = { $hours }시간 { $minutes }분
commands-minutes-and-seconds = { $minutes }분 { $seconds }초
commands-seconds = { $seconds }초


commands-waking = Tor. 연결이 열리는 데 시간이 걸립니다.
    .keyword = 깨움

commands-waking-through = Tor, 브리지 { $bridges }개를 거쳐. 연결이 열리는 데 시간이 걸립니다.
    .keyword = 깨움

commands-no-tor = { $seconds }초가 지나도 Tor 연결이 없습니다
commands-starting-tor = Tor 시작

commands-signed-giving = 이 노드: 에포크 { $epoch }에 상대에게 파일을 건넸습니다.
    상대: 에포크 { $epoch }에 이 노드에게서 파일을 받았습니다.
    둘이 서명했고, 어느 쪽도 되돌릴 수 없습니다.
    .keyword = 서명

commands-signed-taking = 상대: 에포크 { $epoch }에 이 노드에게 파일을 건넸습니다.
    이 노드: 에포크 { $epoch }에 상대에게서 파일을 받았습니다.
    둘이 서명했고, 어느 쪽도 되돌릴 수 없습니다.
    .keyword = 서명

commands-brimming = 진술 { $statements }개가 한 번에 다 들어가지 않아 다음 교환에 보냅니다
    .keyword = 넘침

# Machine translation. The narrative lines (unseen, keeping, witnessed, rejoined)
# await review by a person who reads Korean; the rest should be read too.

### 명령들이 함께 쓰는 말: 노드를 여는 것, 진술을 주고받는 것, 한 번의 교환, 그리고 시계.

commands-clock-at-zero = { $epoch }. 이 기계의 시계가 1970년을 가리키고 있어서,
    이 노드는 지금이 시간의 시작이라고 믿습니다. 시계를 맞추기
    전까지는 아무도 이 노드에게 무엇도 건네지 않고, 아무도 이
    노드를 증언하지 않습니다.
    .keyword = 에포크

commands-called-first = 처음 만든 키가 부름을 받았습니다.
    .keyword = 부름

commands-called = 키 { $not_called }개를 만들었지만 부름을 받지 못했습니다. 이 키가 받았습니다.
    .keyword = 부름

commands-torn = 끝나지 않은 항목의 { $bytes }바이트를 기록에서 버렸습니다
    .keyword = 찢김

commands-record = 에포크 { $epochs }개에 이미 답했고, 그중 어느 것도 고칠 수 없습니다
    .keyword = 기록

commands-witnessed = 다른 키들이 이 노드에 대해 서명한 진술 { $statements }개.
    이 진술은 그것이 속한 에포크가 지나간 뒤에도 보관합니다.
    창이 지나고 나면 그 에포크에서 남는 것은 이것뿐이기
    때문입니다.
    .keyword = 증언

commands-unseen = 어느 에포크에서도 이 노드에 대해 서명된 것이 없습니다.
    밖으로 연락하는 것은 되지만 연락을 받는 것은 되지 않고,
    세는 것은 뒤의 것뿐입니다. 묻도록 뽑힌 이가 이 노드에 닿아야
    합니다. 원인은 둘입니다. 포트 3333을 이 기계로 보내지 않는
    공유기, 그리고 아무에게도 알려 주지 않은 주소. `serve --tor`는
    둘 다 필요 없습니다. onion 주소는 어떤 공유기 뒤에서도 닿을
    수 있고, 이 클라이언트는 이미 Tor를 가지고 있습니다.
    .keyword = 미관측

commands-roll-alone = 우리 1명, 곧 이 노드 자신
    .keyword = 명부

commands-roll = 우리 { $members }명
    .keyword = 명부

commands-known = 우리 중 { $addresses }명이 어디를 찾으면 되는지 말해 두었습니다
    .keyword = 주소

commands-holding = 파일을 가지고 있고, 건넬 수 있습니다
    .keyword = 보유

commands-keeping = 전부를, 영원히. 이 노드가 그것으로 얻는 것은 없습니다.
    모든 진술은 자기 서명을 지니고 있어 어디에 보관되든 똑같이
    검증됩니다. 공인된 기록 보관소는 없고, 기록 보관인도 없습니다.
    .keyword = 보관

commands-ignored = 읽을 수 없는 입회 기록 { $admissions }개
    .keyword = 무시

commands-learned-where = 우리 { $addresses }명이 어디에 더 있는지 알게 되었습니다
    .keyword = 배움

commands-rejoined = { $were }명을 알던 노드에게서 우리 { $members }명을 더
    이름으로 알게 되었습니다. 우리는 둘로 나뉘어 있었고, 이제
    셈은 하나입니다.
    .keyword = 재회

commands-learned-names = 우리 { $members }명을 더 이름으로 알게 되었습니다
    .keyword = 배움

commands-heard = 우리 { $speakers }명이 말하는 것을 들었습니다
    .keyword = 들음

commands-carried = 아직 열려 있는 에포크에 대한 진술 { $statements }개
    .keyword = 전달

commands-exchange = { $node }  에포크 { $epoch }  { $clocks }  ({ $liveness })
    .keyword = 증언

commands-answered-the-challenge = 우리가 고른 도전에 답함
commands-spoke-first = 먼저 말함. 말했다는 것만 증명됨

commands-clocks-together = 시계 일치
commands-clocks-ahead = 상대 시계가 우리보다 { $apart } 빠름
commands-clocks-behind = 상대 시계가 우리보다 { $apart } 늦음
commands-hours-and-minutes = { $hours }시간 { $minutes }분
commands-minutes-and-seconds = { $minutes }분 { $seconds }초
commands-seconds = { $seconds }초

### 화면: 입력한 것에 대해 기록 창에 남기는 말.

screen-asked = { $typed }
    .keyword = 요청
screen-unheard = 명령을 받을 노드가 없습니다
    .keyword = 미전달
screen-unread = { $why }
    .keyword = 못읽음
screen-refused = { $why }
    .keyword = 거부

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = 키 하나를 읽지 못했습니다: { $why }.
    .keyword = 키보드
screen-keyboard-gone = 키보드를 읽을 수 없어서({ $why }) 화면을 닫고 노드도
    껐습니다. 키보드 없이 돌리려면 `333 run --plain`을 쓰십시오.

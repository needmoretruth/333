### 화면: 화면에 입력한 것에 대해 철야 창에 남기는 말.

screen-asked = { $typed }
    .keyword = 요청
screen-unheard = 이제 명령을 수행하는 곳이 없습니다
    .keyword = 미전달
screen-unread = { $why }
    .keyword = 못읽음
screen-refused = { $why }
    .keyword = 거부

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = 키 하나를 읽지 못했습니다: { $why }. 다음 키는 읽힐 수 있습니다.
    .keyword = 키보드
screen-keyboard-gone = 키보드를 읽을 수 없어서({ $why }) 화면을 닫았고, 철야도 함께
    끝났습니다. `333 serve --plain`은 키보드 없이 철야를 지킵니다.

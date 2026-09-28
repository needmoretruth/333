# Machine translation, awaiting review by a person who reads Korean.

### `333 join`: 파일을 가진 노드에게 건네 달라고 청하기.

join-name = { $name }
    .keyword = 이름

join-knocking = { $address }
    .keyword = 노크

join-silence = { $address }에서 아무에게도 닿지 못했습니다. 333이 끝났다는
    증거는 아닙니다. 이 클라이언트는 파일이 아니라 파일의 해시를
    지닙니다. 파일을 가진 누군가에게서 받는 것 말고는 들어올 길이 없습니다.
    .keyword = 침묵

join-knocking-on = { $address }에 노크
join-exchanging = 하트비트 교환
join-asking = 파일 요청
join-no-answer = { $address }에서 { $seconds }초 동안 답이 없었습니다

join-given = { $giver }에게서
    .keyword = 받음

join-joined = 에포크 { $epoch }
    .keyword = 합류

join-holding = 파일을 가지고 있고, 건넬 수 있습니다
    .keyword = 보유

join-roll = 우리 { $members }명
    .keyword = 명부

join-counted = 에포크 { $epoch }부터이고, 한 에포크도 더 이르지 않습니다. 경계 두 개
    뒤로, 이 에포크의 어디쯤 왔는지에 따라 { $least }분에서 { $most }분 뒤입니다.
    그때까지 묻는 것에는 전부 답하십시오. 그동안 증언되는 것이
    당신이 여기 있었다는 증명의 전부입니다.
    .keyword = 집계

join-vigil = `333 serve`를 실행하고 깨어 있으십시오. 아무도 닿을 수 없는 노드는
    증언될 수 없고, 이 구간은 한 번 증언되거나 끝내 증언되지 않습니다.
    .keyword = 철야

join-already-given = 이 노드는 이미 파일을 가지고 있습니다. 에포크 { $epoch }에 { $giver }이(가)
    건넸습니다. 청할 것이 없고, 아무것도 청하지 않았습니다.
join-same-handover = 이 노드와 { $peer }는 에포크 { $epoch }에 이미 파일을 주고받았습니다.
    같은 에포크에 되건네면 그 건넴을 반대편에서 읽은 것일 뿐이어서 아무도
    명부에 오르지 않으므로, 아무것도 청하지 않았습니다.

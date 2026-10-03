### `333 join`: 파일을 가진 노드에게 건네 달라고 청하기.

join-name = { $name }
    .keyword = 이름

join-knocking = { $address }
    .keyword = 연결

join-silence = { $address }에 닿지 못했습니다. 333이 끝났다는 뜻은 아닙니다.
    이 클라이언트에는 파일이 아니라 파일의 해시만 들어 있어서, 파일을
    가진 노드에게서 받는 것 말고는 들어올 길이 없습니다.
    .keyword = 침묵

join-knocking-on = { $address }에 연결
join-exchanging = 하트비트 교환
join-asking = 파일 요청
join-no-answer = { $address }에서 { $seconds }초 동안 답이 없었습니다

join-given = { $giver }에게서
    .keyword = 받음

join-joined = 에포크 { $epoch }
    .keyword = 합류

join-holding = 파일을 받았고, 다른 노드에게 건넬 수 있습니다
    .keyword = 보유

join-roll = 우리 { $members }명
    .keyword = 명부

join-counted = 에포크 { $epoch }부터 셉니다. 에포크 경계 두 번 뒤, 지금부터 { $least }~{ $most }분
    뒤입니다. 그때까지 오는 질문에 모두 답하십시오. 그동안 받은 증언이
    이 노드가 여기 있었다는 증명의 전부입니다.
    .keyword = 집계

join-vigil = `333 start`로 켜 두십시오. 아무도 닿지 못하는 노드는 증언받을 수
    없고, 이 시기의 증언은 지금 아니면 받을 수 없습니다.
    .keyword = 노드

join-already-given = 이 노드는 이미 파일이 있습니다(에포크 { $epoch }에 { $giver }에게서 받음).
    청하지 않았습니다.
join-same-handover = 이 노드와 { $peer }는 에포크 { $epoch }에 이미 파일을 주고받았습니다.
    같은 에포크에 되받으면 아무도 명부에 오르지 않으므로 청하지
    않았습니다.

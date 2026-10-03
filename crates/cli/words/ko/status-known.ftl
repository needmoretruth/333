### `333 status --all`: 아는 주소와 같은 이름의 다른 노드.
status-known-another-copy = 같은 이름의 노드가 하나 더 있습니다
status-known-sighting = 이 노드의 키로 서명됐지만 이 노드가 만들지 않은 진술이, 이 노드가
    { $address }에 있다고 말합니다(에포크 { $said_in }).
    받은 곳: { $arrived }, 에포크 { $epoch }.
status-known-either = 이 디렉터리를 복사해 켰거나, 다른 사람이 키를 가지고 있습니다. 같은
    이름의 두 노드는 에포크마다 서로 어긋나니 하나를 끄십시오. 노드를
    옮길 때는 `333 pack`을 씁니다. 이 노드는 어느 쪽도 스스로 끄지
    않습니다. 옛 진술은 누구나 다시 보낼 수 있어서, 그것을 보고 꺼지면
    키 사본을 가진 사람이 이 노드를 끌 수 있게 됩니다.
status-known-nowhere = 아직 연결할 곳이 없습니다. `333 ping`이나 `333 join`에 준 주소는
    기억해 두고, 그때부터 그곳에 연결합니다.
    .keyword = 주소
status-known-held = 주소 { $held }개, 처음 받은 곳별로
    .keyword = 주소
status-known-by-hand = 직접 입력
status-known-this-network = 이 네트워크
status-known-meeting-point = 만남의 장소
status-known-from-us = 노드 { $peers }개에게서
status-known-not-noted = 기록 없음
status-known-where-heard = 어디서 받았는지는 그 주소에서 누가 답하는지와 상관없습니다.
status-known-sources-lists = `333 status --sources`가 목록을 보여 줍니다.
status-known-sources = 받은 곳
status-known-nobody-answered = 아직 답한 노드가 없습니다
status-known-at = 주소
status-known-first = 처음
status-known-last = 마지막
status-known-from-before = 주소 출처를 적기 전부터 가지고 있던 것
status-known-when = { $from }, 에포크 { $epoch }

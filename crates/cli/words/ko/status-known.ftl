### `333 status`: 다른 이들이 어디 있는지, 이 노드가 아는 만큼, 그리고 각각 어디서
### 들었는지.

status-known-another-copy = 이 이름의 다른 사본

status-known-sighting = 이 노드의 키로 서명되었지만 이 노드가 만든 적 없는 진술이,
    이 노드가 { $address }에 있다고 말합니다. 에포크 { $said_in }.
    도착 경로: { $arrived }, 에포크 { $epoch }.

status-known-either = 이 디렉터리가 복사되어 사본이 시작되었거나, 다른 누군가가 키를
    가지고 있습니다. 한 이름의 두 노드는 어느 쪽이 질문을 받든 매 에포크
    서로 어긋납니다. 하나를 멈추십시오. 노드를 옮기는 방법은 `333 pack`
    입니다. 여기서는 어느 사본도 대신 멈추지 않습니다. 옛 진술은 누구나
    다시 보낼 수 있어서, 그것을 보고 멈추는 노드라면 그 키의 사본을 가진
    누구라도 끌 수 있습니다.

status-known-nowhere = 아직 두드릴 곳이 없습니다. `333 ping`이나 `333 join`에 준
    초대는 간직되고, 그 뒤로는 철야가 그곳을 두드립니다.
    .keyword = 주소

status-known-held = { $held }개, 각각 처음 들은 곳별로
    .keyword = 주소

status-known-by-hand = 직접 입력
status-known-this-network = 이 네트워크
status-known-meeting-point = 만남의 장소
status-known-from-us = 우리 { $peers }명에게서
status-known-not-noted = 기록 없음

status-known-where-heard = 어디서 들었는지는 그곳에서 누가 답하는지와 상관이 없습니다.
status-known-sources-lists = `333 status --sources`가 목록을 보여 줍니다.

status-known-sources = 출처
status-known-nobody-answered = 여기서는 아직 아무도 답하지 않았습니다
status-known-at = 주소
status-known-first = 처음
status-known-last = 마지막
status-known-from-before = 이 노드가 주소의 출처를 적기 전부터 가지고 있던 것
status-known-when = { $from }, 에포크 { $epoch }

### `333 status --all`: 이 노드가 본 것. 응답 수, 이 노드의 자리, 침묵이 시작됐다면 남은 시간.
status-name = { $name }
    .keyword = 이름

status-epoch = { $epoch }
    .keyword = 에포크

status-answering = 응답 중
status-silent = 침묵
status-roll = 명부
status-seen = 첫 숫자는 에포크 { $before } 또는 { $now }에 이 노드가 서명을 가진 노드
    수입니다. 이 노드가 본 것이고, 다른 노드는 다른 것을 봅니다.
status-seen-without-tor = 이 빌드는 Tor로 숨은 노드에 닿지 못하므로, 숨은 노드는 이 숫자에
    들지 않습니다.
status-how-many-people = 그것이 몇 사람인지는 이 노드가 알 수 없습니다. 아는 것은 그 이름이
    모두 두 에포크 중 하나에 답했고, 셈에 남으려면 다음 에포크에도
    답해야 한다는 것입니다. 한 사람이 천 개를 돌리면 천 개 몫을 매번
    치러야 하고, 멈추는 순간 셈에서 빠집니다.
status-given-by = 받은 곳
status-you = 이 노드
status-received-in = 에포크 { $epoch }에 받음
status-trail-stops = 여기서 기록이 끊깁니다.
status-stopped-knowing = 이 노드가 아는 것이 여기까지라는 뜻이지, 줄이 여기서 시작됐다는
    뜻은 아닙니다. 줄을 시작한 노드는 받은 기록이 없고, 이 노드가 아직
    받지 못한 기록도 여기서는 똑같이 보입니다.
status-nothing-said = 에포크 { $epoch }에는 아무도 말하지 않았습니다. 333가지를 말할 수
    있고, 아직 그 뜻은 정해지지 않았습니다.
status-said = 에포크 { $epoch }에 말함 — 이 노드가 보는 { $seen }명 중 { $spoke }명이 말했고 {
    ""}{ $silent }명은 말하지 않았습니다.
status-a-third = ← 삼분의 일 이상
status-not-said = 나머지 { $others }가지는 아무도 말하지 않았습니다.
status-no-winner = 고르는 것도 정하는 것도 없습니다. 이 노드에 닿은 것일 뿐이고, 옆
    노드가 다른 것을 들었어도 틀린 것이 아닙니다.
status-reading-the-watch = 관측 기록 읽기
status-seen-nobody = { $watched } 동안 쉬지 않고 지켜봤지만 아무도 이 노드에 답하지
    않았습니다. 이 빌드는 Tor로 숨은 노드를 듣지 못하므로 끝이라고
    말하지 않습니다. 아무도 보지 못했다는 것과 아무도 없다는 것은
    다릅니다.
status-never-answered = 아직 이 노드에 답한 노드가 없습니다. 아무 데도 연결된 적 없는
    노드는 처음에 이렇게 보입니다.
status-somebody-is-here = 답하는 노드가 있습니다.
status-waiting = { $silent } 동안 아무도 답하지 않았습니다. { $needed }이(가) 지나야
    침묵이라고 말하고, 그동안 계속 켜져 있어야 합니다.
status-nobody-keeping = 아무도 답하지 않습니다
    여기 남은 것은 이 노드뿐입니다. { $watched }(77일) 동안 쉬지 않고
    지켜봤지만 아무도 답하지 않았고, 마지막 노드는 에포크 { $since }에
    멈췄습니다.
    333은 사라지지 않았습니다. 사라지는 중이고, 그것은 { $years }년이 걸립니다.
status-remain = { $years }년 { $days }일 남았습니다.
status-run-out = 마지막 해까지 다 지났습니다.
status-one-answer = 이 셈은 마지막 노드가 멈춘 때부터 시작됐고, 지켜보는 동안 계속
    흘렀습니다.
    답 하나면 끝납니다. 어느 노드든 이 노드에 답하면 셈은 멈추는 것이
    아니라 버려지고, 얼마나 가까웠는지는 남지 않습니다.
status-epochs = { $count }에포크
status-share = { $whole }.{ $after }%

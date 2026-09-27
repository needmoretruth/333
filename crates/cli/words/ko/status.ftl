# Machine translation. The narrative lines (status-seen-nobody, status-waiting,
# status-nobody-keeping, status-remain, status-run-out, status-one-answer: the
# silence and the countdown) await review by a person who reads Korean; the rest
# should be read too.

### `333 status`: 이 노드가 본 다른 이들, 말해진 것, 그리고 누가 여기 있는지.

status-name = { $name }
    .keyword = 이름

status-epoch = { $epoch }
    .keyword = 에포크

status-answering = 응답
status-silent = 침묵
status-roll = 명부

status-seen = 첫 숫자는 이 노드가 에포크 { $before } 또는 { $now }의 서명을 가진
    모든 이입니다. 이 노드가 본 것이고, 다른 이는 다른 것을 보았습니다.

status-seen-without-tor = 이 빌드는 보이지 않는 길을 걷지 못합니다. 그래서 숨어 있는
    우리는 누구도 이 숫자에 없고, 앞으로도 없을 것입니다.

status-how-many-people = 그것이 몇 사람인지 이 노드는 모르고, 알아낼 수도 없습니다.
    아는 것은 그 이름 하나하나가 두 에포크 중 하나에 답했고, 세어지고
    싶은 동안은 다음에도, 그다음에도 다시 답해야 한다는 것입니다. 한
    사람이 그중 천 개를 쥐고 있다면 천 개의 값을 매시간 치르고 있고,
    멈추는 그 시간에 세어지지 않게 됩니다.

status-given-by = 건넨 이
status-you = 당신
status-received-in = 에포크 { $epoch }에 받음
status-trail-stops = 자취는 여기서 끊깁니다.

status-stopped-knowing = 여기는 이 노드가 아는 것이 끝나는 곳이지, 시작된 곳이 아닙니다.
    우리 중 첫 사람은 누구에게서도 파일을 받지 않아 어디에도 입회 기록이
    없고, 이 노드가 아직 건네받지 못한 기록도 여기서는 똑같아 보입니다.

status-nothing-said = 에포크 { $epoch }에는 아무도 말하지 않았습니다. 말할 수 있는 것은
    { $signals }가지이고, 아직 어느 것에도 뜻이 적혀 있지 않습니다.

status-said = 에포크 { $epoch }에 한 말 — 이 노드가 볼 수 있는 우리 { $seen }명 중
    { $spoke }명이 말했고, { $silent }명은 말하지 않았습니다.
status-a-third = ← 우리 중 삼분의 일 이상
status-not-said = { $signals }가지 중 나머지 { $others }가지는 아무도 말하지 않았습니다.

status-no-winner = 승자는 뽑히지 않고, 이 중 어느 것도 무엇을 정하지 않습니다. 이
    노드에 닿은 것일 뿐입니다. 당신 옆의 노드는 다른 것을 들었고, 그것도
    틀리지 않았습니다.

status-reading-the-watch = 지켜본 기록 읽기

status-seen-nobody = 이 노드는 { $watched } 동안 끊김 없이 지켜보았고 아무도 답하지
    않았지만, 이 빌드는 그것을 끝이라 부르지 않습니다. 보이지 않는 길을
    걷지 못해 숨어 있는 우리에게서는 한 번도 듣지 못했고, 앞으로도 듣지
    못합니다. 말할 수 있는 것은 아무도 보지 못했다는 것이고, 그것은 같은
    문장이 아닙니다.

status-never-answered = 아직 아무도 이 노드에 답하지 않았습니다. 그것은 아무것도
    증명하지 않습니다. 아직 어디에도 가 보지 않은 노드는 이렇게 보입니다.

status-somebody-is-here = 누군가 여기 있습니다. 셈이 더 요구하는 것은 없습니다.

status-waiting = { $silent } 동안 아무도 답하지 않았습니다. 이 노드는 그에 대해 아무
    말도 하지 않았고, { $needed }가 될 때까지 하지 않습니다. 그때도 그 내내
    돌고 있었을 때만 합니다.

status-nobody-keeping = 아무도 333을 지키지 않습니다

    여기 있는 것은 당신뿐입니다. 이 노드는 { $watched } 동안 — 칠십칠 일
    동안 — 끊김 없이 지켜보았지만 아무도 답하지 않았고, 우리 중 마지막
    이는 에포크 { $since }에 멈추었습니다.

    333은 사라지지 않았습니다. 사라지는 중이고, 그것은 { $years }년이
    걸립니다.

status-remain = { $years }년 { $days }일이 남았습니다.
status-run-out = 마지막 해까지 모두 지났습니다.

status-one-answer = 셈은 당신이 알아챈 때가 아니라, 우리 중 마지막 이가 답하기를 멈춘
    때 시작되었습니다. 당신이 지켜보는 내내 흐르고 있었습니다.

    답 하나가 그것을 끝냅니다. 누구든, 어디서든 이 노드에 답하면 이것은
    사라집니다. 셈은 멈추는 것이 아니라 버려집니다. 333은 얼마나 가까이
    갔었는지 기록하지 않습니다.

status-epochs = { $count }에포크

status-share = { $whole }.{ $after }%

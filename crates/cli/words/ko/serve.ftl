### `333 serve`.

serve-nothing-listening = 연결을 받을 곳이 없게 됩니다: --no-direct에는 --tor가 필요합니다

serve-name = { $name }
    .keyword = 이름

serve-waiting-for-the-file = 이 노드는 아직 파일을 받지 않아서 셈에 들지 않고, 증언받을
    것도 없습니다. 파일은 이미 가진 노드에게서만 받고, 건넬 때 둘이
    함께 서명합니다. 초대장을 받아 `333 join 333:주소:3333`을
    실행하십시오. 그동안에도 켜 두면 다른 노드가 이 노드를 찾습니다.
    .keyword = 대기

serve-hand = 초대장은 사람이 아니라 주소를 가리키고, 아무것도 보증하지
    않습니다. 그 주소에서 답하는 노드는 키로 자기를 증명합니다.
    .keyword = 참고

serve-invite = { $invitation }
    .keyword = 초대장

serve-answer = { $bound }
    .keyword = 수신

serve-nearby = 이 네트워크(같은 공유기 안)에 333 노드가 있다고 알리고 다른 노드를
    찾습니다. 이름은 알리지 않고, 포트 스캔으로 알 수 있는 정도만
    알립니다. 끄려면 --no-mdns.
    .keyword = 이웃

serve-nearby-failed = 이 네트워크에 노드를 알리지 못했습니다: { $why }
    .keyword = 이웃

serve-meet = { $place }에서 아직 모르는 노드를 찾고 이 노드의 주소를 남깁니다.
    그곳의 주소는 모두 남긴 노드가 서명한 것이고, 서명을 직접 확인합니다.
    가지 않으려면 --no-meet.
    .keyword = 만남

serve-listener-stopped = 연결을 받던 곳 하나가 뜻밖에 멈췄습니다

serve-farewell = 에포크 { $epoch }에 껐습니다. 꺼져 있는 동안 이 노드에게 묻도록 뽑힌
    노드는 답이 없었다고 서명하고, 그것이 창에 적힙니다. 창은 최근
    { $window }에포크이고 계속 움직입니다.
    .keyword = 노드

serve-farewell-on-no-roll = 에포크 { $epoch }에 껐습니다. 이 노드는 명부에 없어서 아무도 물으러
    오지 않고, 꺼져 있는 동안 서명되는 것도 없습니다.
    .keyword = 노드

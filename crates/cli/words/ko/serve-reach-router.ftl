### `333 run`: 공유기에 포트를 청하고, 빌린 것을 유지하고, 돌려주기.

serve-reach-router-opened-upnp = 공유기가 { $on } 포트 { $port }번을 이 기계로 보냅니다. 공유기 설정에
    `333`이라는 이름으로 있으니 지우면 닫힙니다. 실제로 들어오는지는
    다음 줄에 나옵니다.
    .keyword = 개방
serve-reach-router-opened-upnp-for = 공유기가 { $on } 포트 { $port }번을 { $time } 동안 이 기계로 보냅니다.
    공유기 설정에 `333`이라는 이름으로 있으니 지우면 닫힙니다. 실제로
    들어오는지는 다음 줄에 나옵니다.
    .keyword = 개방
serve-reach-router-opened-lease = { $router } 공유기에 { $way }로 포트 { $port }번을 { $asked_for } 동안 청했고,
    공유기가 { $on } 포트 { $granted_port }번을 { $granted } 동안 이 기계로 보냅니다. 기한
    전에 다시 청하고, 노드를 끄면 돌려줍니다. 노드가 강제로 멈추면
    기한이 지나 공유기가 거둡니다. 실제로 들어오는지는 다음 줄에
    나옵니다.
    .keyword = 개방
serve-reach-router-nobody-answered = 포트를 열어 달라는 요청(UPnP-IGD · PCP · NAT-PMP)에 답한 공유기가
    없습니다. 셋 다 꺼 둔 공유기가 많고, 공인 주소를 가진 기계는 물을
    필요가 없습니다. `--no-router`를 주면 묻지 않습니다.
    .keyword = 닫힘
serve-reach-router-refused = 공유기가 포트 { $port }번을 열지 않았습니다: { $why }
    .keyword = 닫힘
serve-reach-router-let-go = 제때 다시 청하지 못해 공유기가 포트 { $port }번을 닫았습니다. 지금은
    밖에서 이 포트로 들어올 수 없습니다. 노드를 다시 켜면 다시 청합니다.
    .keyword = 닫힘
serve-reach-router-given-back = 포트 { $port }번을 { $way }로 공유기에 돌려주었습니다. 이제 이
    기계로 오지 않습니다.
    .keyword = 닫힘
serve-reach-router-not-taken-back = 공유기가 포트 { $port }번을 돌려받지 않았습니다({ $why }).
    { $time } 안에 공유기가 스스로 닫습니다.
    .keyword = 닫힘
serve-reach-router-moved = 공유기가 포트를 바꿨습니다. 이제 { $before_on } 포트 { $before_port }번
    대신 { $on } 포트 { $port }번이 이 기계로 옵니다. 옛 주소가 적힌
    초대장은 더 이상 닿지 않습니다.
    .keyword = 개방
serve-reach-router-not-kept = 다시 청했을 때 공유기가 포트 { $port }번을 이어 주지 않았습니다({ $why }).
    아직 { $time } 남았고, 그 전에 다시 청합니다.
    .keyword = 대기
serve-reach-router-on-its-outside-address = 바깥 주소의
serve-reach-router-on = { $address }의
serve-reach-router-one-second = 1초
serve-reach-router-two-seconds = 2초
serve-reach-router-seconds = { $count }초
serve-reach-router-one-minute = 1분
serve-reach-router-two-minutes = 2분
serve-reach-router-minutes = { $count }분
serve-reach-router-one-hour = 1시간
serve-reach-router-two-hours = 2시간
serve-reach-router-hours = { $count }시간

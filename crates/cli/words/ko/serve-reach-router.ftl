# Machine translation, awaiting review by a person who reads Korean.
# The 1 and 2 below are written the same in every base this client counts in.

### `333 serve`: 공유기에 묻고, 빌려준 것을 지키고, 돌려주기.

serve-reach-router-opened-upnp = 공유기에 따르면 { $on } 포트 { $port }번이 이제 이 기계로
    옵니다. 공유기에 `333`이라는 이름으로 올라 있으니, 없애려면 그것을
    지우면 됩니다. 무언가 도착하는지는 다음 줄에 나옵니다.
    .keyword = 개방

serve-reach-router-opened-upnp-for = 공유기에 따르면 { $on } 포트 { $port }번이 이제 { $time } 동안
    이 기계로 옵니다. 공유기에 `333`이라는 이름으로 올라 있으니,
    없애려면 그것을 지우면 됩니다. 무언가 도착하는지는 다음 줄에
    나옵니다.
    .keyword = 개방

serve-reach-router-opened-lease = { $router }의 공유기에 { $way }로 포트 { $port }번을 { $asked_for } 동안
    청했습니다. 공유기에 따르면 { $on } 포트 { $granted_port }번이 이제 { $granted }
    동안 여기로 옵니다. 이 노드는 그 시간이 다하기 전에 다시 청하고,
    철야가 끝나면 돌려줍니다. 다른 식으로 멈추면 공유기가 시간이
    다했을 때 스스로 거둡니다. 무언가 도착하는지는 다음 줄에 나옵니다.
    .keyword = 개방

serve-reach-router-nobody-answered = 여기서는 UPnP-IGD, PCP, NAT-PMP 어느 쪽으로도 포트를 열어
    달라는 요청에 답한 공유기가 없습니다. 흔한 일입니다. 셋 다 꺼
    둔 공유기가 많고, 자기 주소를 가진 기계는 물을 것이 없습니다.
    `--no-router`를 주면 이 노드는 아예 묻지 않습니다.
    .keyword = 닫힘

serve-reach-router-refused = 공유기가 포트 { $port }번을 열어 주지 않았습니다: { $why }
    .keyword = 닫힘

serve-reach-router-let-go = 공유기가 포트 { $port }번을 놓았습니다. 제때 다시 청하지 못해서,
    이제 밖에서는 그 포트로 이 노드에 닿을 수 없습니다. 철야를
    다시 시작하면 다시 청합니다.
    .keyword = 닫힘

serve-reach-router-given-back = 포트 { $port }번을 { $way }로 공유기에 돌려주었습니다. 이제 그
    포트는 이 기계로 오지 않습니다.
    .keyword = 닫힘

serve-reach-router-not-taken-back = 공유기가 포트 { $port }번을 돌려받지 않았습니다({ $why }).
    공유기는 { $time } 안에 스스로 그것을 거둡니다.
    .keyword = 닫힘

serve-reach-router-moved = 공유기가 이 노드를 옮겼습니다. 이제 { $before_on } 포트
    { $before_port }번 대신 { $on } 포트 { $port }번이 여기로 옵니다.
    옛 주소를 적은 초대장으로는 더 이상 도착하지 않습니다.
    .keyword = 개방

serve-reach-router-not-kept = 청했을 때 공유기가 포트 { $port }번을 이어 주지 않았습니다({ $why }).
    공유기는 아직 { $time } 동안 그것을 가지고 있고, 그 전에 다시 청합니다.
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

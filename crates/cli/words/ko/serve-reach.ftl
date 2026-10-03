### `333 run`: 밖에서 이 노드에 실제로 들어올 수 있는지.

serve-reach-unanswered-at-the-end = 노드를 끌 때까지 공유기가 답하지 않았습니다. 공유기가
    열어 준 포트는 { $time } 안에 저절로 닫힙니다.
    .keyword = 닫힘
serve-reach-shut-behind-another = 공유기의 바깥 주소가 { $seen }인데, 인터넷에서 바로 닿는 주소가
    아닙니다. 공유기가 하나 더 있거나 통신사가 주소를 나눠 쓰게 하고
    있어서, 여기서는 포트를 열 수 없습니다. `333 run --tor`는
    공유기를 바꾸지 않아도 됩니다.
    .keyword = 막힘
serve-reach-open = 밖에서 포트 { $port }번으로 이 기계에 들어올 수 있습니다. 이 노드가
    { $outside }에 연결해 스스로 답했으니, 이 주소를 누구에게나 줄 수
    있습니다.
    .keyword = 열림
serve-reach-invite = { $invitation }
    .keyword = 초대
serve-reach-shut-somebody-else = { $outside }에서 이 노드가 아닌 다른 것이 답했습니다. 그 포트는
    다른 프로그램이 쓰고 있어서, 이 주소가 적힌 초대장은 엉뚱한 곳으로
    갑니다.
    .keyword = 막힘
serve-reach-shut-unfinished = { $outside }의 무언가가 연결을 받았지만 하트비트를 끝내지
    않았습니다: { $why }. 이 주소가 적힌 초대장은 쓸 수 없습니다.
    .keyword = 막힘
serve-reach-shut-nothing = { $outside }에서 아무것도 답하지 않았습니다. 밖에서는 이 노드에 들어올
    수 없습니다. 공유기가 포트 { $port }번을 이 기계로 보내도록 설정되지
    않았거나, 안쪽 기계가 자기 바깥 주소로 접속하는 것을 공유기가
    막습니다. `333 run --tor`는 공유기를 바꾸지 않아도 되고 어느
    네트워크에서나 됩니다.
    .keyword = 막힘

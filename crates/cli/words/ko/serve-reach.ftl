### `333 serve`: 밖에서 누가 정말 들어올 수 있는지.

serve-reach-unanswered-at-the-end = 철야가 끝날 때까지 공유기가 답하지 않았습니다. 공유기가
    허락한 것은 { $time } 안에 저절로 끝납니다.
    .keyword = 닫힘

serve-reach-shut-behind-another = 공유기는 이 집의 주소가 { $seen }라고 하지만, 그것은 열린
    인터넷의 주소가 아닙니다. 다른 공유기나 통신사의 공유 주소가
    이 집과 다른 모든 이 사이에 있고, 여기서는 그쪽에 물을 수
    없습니다. `333 serve --tor`는 공유기를 전혀 바꾸지 않아도 됩니다.
    .keyword = 막힘

serve-reach-open = 포트 { $port }가 밖에서 이 기계에 닿습니다. 이 노드가 { $outside }를
    두드렸고 스스로 답했으니, 그 주소는 누구에게나 건넬 수 있습니다.
    .keyword = 열림

serve-reach-invite = { $invitation }
    .keyword = 초대

serve-reach-shut-somebody-else = { $outside }에서 무언가 답했지만 이 노드가 아니었습니다. 당신
    주소의 그 포트는 다른 것의 것이므로, 그 주소를 적은 초대장은
    사람들을 엉뚱한 기계로 보냅니다.
    .keyword = 막힘

serve-reach-shut-unfinished = { $outside }의 무언가가 연결을 받았지만 하트비트를 마치지
    않았습니다: { $why }. 그 주소를 적은 초대장은 건넬 것이 못 됩니다.
    .keyword = 막힘

serve-reach-shut-nothing = { $outside }에서 아무것도 답하지 않았으니, 바깥에서 보기에 이
    노드는 듣고 있지 않습니다. 앞의 공유기가 포트 { $port }를 여기로
    보내라는 말을 들은 적이 없거나, 안쪽 기계가 자기 바깥 주소로
    접속하는 것을 허락하지 않습니다. `333 serve --tor`는 공유기를
    전혀 바꾸지 않아도 되고 어느 네트워크에서나 됩니다. 닿을 수
    있는 주소를 아예 주지 않는 네트워크에서도 됩니다.
    .keyword = 막힘

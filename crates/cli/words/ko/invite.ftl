### `333 invite`: 다른 사람이 이 노드를 통해 들어올 때 쓰는 초대장.
invite-line = { $invitation }
    .keyword = 초대장
invite-how = 이 줄을 상대에게 주십시오. 상대는 자기 기계에서
    `333 join { $invitation }`을 실행합니다.
    .keyword = 사용법
invite-none = 아직 없습니다. 밖에서 닿는 주소를 이 노드가 찾지 못했습니다.
    .keyword = 초대장
invite-none-next = `333 start`로 켜고 몇 분 뒤 다시 확인하십시오. 공유기 때문에
    밖에서 닿지 않으면 `333 service install --tor`로 어니언 주소를 씁니다.
    공유기는 바꾸지 않아도 됩니다.
    .keyword = 다음

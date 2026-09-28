### `333 pack`: 다른 기계로 옮기려고 이 노드를 파일 하나에 담기.

pack-name-the-file = 이 노드를 담을 파일을 지정하십시오: 333 pack <FILE>

pack-no-node = { $root }에는 포장할 노드가 없습니다. 아무것도 쓰지 않았습니다.

pack-already-exists = { $file } 파일이 이미 있습니다. 포장은 새 파일에만 쓰고
    옛 파일 위에는 쓰지 않습니다. 다른 이름을 주십시오.

pack-not-marked = { $file } 파일은 썼지만, 이 디렉터리에 포장했다는 표시를 하지
    못했습니다. 표시하기 전까지 이 노드는 두 곳에 삽니다. 여기서 무엇이든
    실행하기 전에 그 파일을 지우십시오.

pack-creating = { $file } 만들기

pack-name = { $name }
    .keyword = 이름

pack-record-none = 아직 없음
    .keyword = 기록

pack-record = 에포크 { $epochs }개, 함께 갑니다
    .keyword = 기록

pack-witnessed = 다른 키들이 이 노드에 대해 서명한 진술 { $statements }개, 함께 갑니다
    .keyword = 증언

pack-holding = 파일, 함께 갑니다
    .keyword = 보유

pack-onion-key = 어니언 주소의 키. 그래서 주소도 함께 갑니다
    .keyword = 미관측

pack-carrying = 이 노드를 { $file }에 담습니다.
    그 파일이 곧 이 노드입니다. 가진 사람은 누구나 이 이름으로 답할 수
    있습니다. 옮기고, 풀고, 지우십시오. 간직할 백업이 아닙니다.
    암호화하지 않았습니다. 비밀번호는 잃어버릴 것을 하나 더 만들고,
    그것을 잃으면 파일을 잃은 것만큼 확실하게 이름을 잃습니다.
    이 디렉터리처럼 당신만 읽을 수 있습니다.
    .keyword = 운반

pack-packed = { $bytes }바이트: 파일들 그대로에, 파일마다 반 킬로바이트씩.
    이제 { $root }에서는 아무것도 이 노드로서 움직이지 않습니다.
    .keyword = 포장

pack-next = 다른 기계에서: 333 unpack { $carried }
    옮기기를 그만두면: { $undo }
    .keyword = 다음

pack-not-packed = 이 노드는 포장되지 않았으므로 { $root }에서 되돌릴 것이 없습니다
    .keyword = 여기

pack-restored = 이 노드는 다시 { $root }에 삽니다.
    포장했던 파일은 여전히 이 이름입니다. 어디서든 풀었다면 둘 중 하나가
    사라져야 어느 쪽이든 실행할 수 있습니다. 풀지 않았다면
    { $file } 파일을 지우십시오.
    .keyword = 복귀

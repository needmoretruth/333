### `333 pack`: 다른 기계로 옮기려고 이 노드를 파일 하나에 담기.

pack-name-the-file = 이 노드를 담을 파일을 지정하십시오: 333 pack <FILE>

pack-no-node = { $root }에는 포장할 노드가 없습니다. 아무것도 쓰지 않았습니다.

pack-already-exists = { $file } 파일이 이미 있습니다. 덮어쓰지 않으니 다른 이름을
    주십시오.

pack-not-marked = { $file } 파일은 썼지만 이 디렉터리에 「포장함」 표시를 하지
    못했습니다. 지금은 같은 노드가 두 곳에 있습니다. 여기서 다른 명령을
    실행하기 전에 그 파일을 지우십시오.

pack-creating = { $file } 만들기

pack-name = { $name }
    .keyword = 이름

pack-record-none = 아직 없음
    .keyword = 기록

pack-record = 기록: 에포크 { $epochs }개
    .keyword = 기록

pack-witnessed = 다른 노드가 이 노드에 대해 서명한 진술 { $statements }개
    .keyword = 증언

pack-holding = 파일
    .keyword = 보유

pack-onion-key = 어니언 주소의 키(주소도 그대로 옮겨집니다)
    .keyword = 미관측

pack-carrying = 이 노드를 { $file }에 담습니다.
    이 파일이 곧 이 노드입니다. 가진 사람은 누구나 이 이름으로 답할 수
    있습니다. 옮겨서 풀고 나면 지우십시오. 보관용 백업이 아닙니다.
    비밀번호는 걸지 않았습니다. 비밀번호를 잊으면 이름도 잃기 때문입니다.
    이 디렉터리처럼 이 계정만 읽을 수 있습니다.
    .keyword = 운반

pack-packed = { $bytes }바이트.
    이제 { $root }에서는 이 노드가 실행되지 않습니다.
    .keyword = 포장

pack-next = 다른 기계에서: 333 unpack { $carried }
    옮기기를 그만두면: { $undo }
    .keyword = 다음

pack-not-packed = 이 노드는 포장되지 않았으므로 { $root }에서 되돌릴 것이 없습니다
    .keyword = 여기

pack-restored = 이 노드가 다시 { $root }에서 실행됩니다.
    포장한 파일도 같은 이름입니다. 다른 곳에 풀었다면 둘 중 하나를 지운
    뒤에 실행하십시오. 풀지 않았다면 { $file } 파일을 지우십시오.
    .keyword = 복귀

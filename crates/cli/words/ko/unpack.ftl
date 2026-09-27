# Machine translation, awaiting review by a person who reads Korean.

### `333 unpack`: 포장된 노드를 이 기계의 노드 디렉터리에 넣기.

unpack-kept = 이 디렉터리에는 이미 노드가 삽니다. 그 곁에 풀려면
    --data-dir로 자기 디렉터리를 주십시오.

unpack-seed-in = { $file } 안의 { $seed }

unpack-not-one-node = 그 파일이 말하는 이름은 { $claimed },
    안에 든 키는 { $name }입니다.
    한 노드가 아니므로 아무것도 풀지 않았습니다.

unpack-taken = 다른 333이 풀어 넣던 디렉터리를 가져갔습니다. 아무것도 풀지 않았습니다.

unpack-elsewhere = 333 --data-dir <다른 디렉터리> unpack { $file }

unpack-could-not-open = { $target }에 이미 노드가 사는데, 무엇을 가졌는지 말하려고
    열 수가 없었습니다. 아무것도 풀지 않았습니다. 그 곁에 풀려면:
    { $elsewhere }

unpack-no-record = 아직 기록 없음
unpack-epochs-of-record = 기록 { $epochs }에포크
unpack-holding = 파일 보유
unpack-not-holding = 파일 없음

unpack-occupied = { $target }에 이미 노드가 삽니다:
    { $name }, { $epochs }, { $holding }.
    그 위에 풀면 그 전부를 영영 잃으므로 아무것도 풀지 않았습니다.
    그 곁에 풀려면 자기 디렉터리를 주십시오:
    { $elsewhere }

unpack-holds-files = { $target }에는 노드 없이 파일만 있습니다. 노드는 자기
    디렉터리에 풀므로 아무것도 풀지 않았습니다. 다른 곳에 풀려면:
    { $elsewhere }

unpack-opening-the-record = 기록 열기
unpack-torn = 그 파일의 기록이 찢겨 있어 온전한 노드가 아닙니다.
    아무것도 풀지 않았습니다.
unpack-reading-the-record = 기록 읽기
unpack-does-not-verify = 그 파일의 기록이 검증되지 않습니다. 아무것도 풀지 않았습니다.
unpack-another-key = 그 파일의 기록은 다른 키가 썼습니다. 아무것도 풀지 않았습니다.

unpack-not-a-place = { $target }: 노드를 넣을 수 있는 디렉터리가 아닙니다
unpack-making-room = { $target }에 자리 만들기
unpack-putting = { $target }에 노드 넣기

unpack-name = { $name }
    .keyword = 이름

unpack-record-none = 아직 없음
    .keyword = 기록

unpack-record = 에포크 { $epochs }개, 검증함
    .keyword = 기록

unpack-holding-the-file = 파일
    .keyword = 보유

unpack-onion-key = 어니언 주소의 키. 그래서 주소도 함께 왔습니다
    .keyword = 미관측

unpack-unpacked = { $target }에 풀었습니다.
    { $packed }에 포장한 파일에서 왔습니다.
    이제 이것이 그 노드이고, 그 파일도 그렇습니다: { $file } 파일을 지우십시오
    .keyword = 풀기

unpack-next = { $serve }
    .keyword = 다음

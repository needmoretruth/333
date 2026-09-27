# Machine translation, awaiting review by a person who reads Korean.

### 다른 333이 이미 이 노드의 디렉터리를 가지고 있을 때 명령이 하는 말.

elsewhere-the-vigil = 이 디렉터리에서 지키는 철야
elsewhere-the-vigil-by-number = 이 디렉터리에서 지키는 철야(프로세스 { $pid })
elsewhere-another = 다른 333
elsewhere-another-by-number = 다른 333(프로세스 { $pid })

elsewhere-done = { $vigil }에서 수행했습니다.
    .keyword = 완료

elsewhere-failed = { $vigil }에서 그 일을 하지 못했습니다.
    .keyword = 실패

elsewhere-finding-its-name = { $who }이 아직 이 노드의 이름을 찾고 있습니다.
    이름이 생기면 다시 실행하십시오.
    .keyword = 점유

elsewhere-already-keeping = { $who }이 이미 여기서 철야를 지키고 있고,
    한 디렉터리는 한 노드입니다. 여기서 그것에게 말할 수 있습니다:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`. 두 번째
    노드에는 --data-dir로 준 자기 디렉터리가 필요합니다.
    .keyword = 점유

elsewhere-keeping = { $who }이 여기서 철야를 지키고 있고,
    { $why }
    .keyword = 점유

elsewhere-nobody-to-tell = 이 디렉터리에서 철야를 지키는 이가 없어서 말할 상대가
    없습니다. `333 serve`가 철야를 지키고, 그러면 이것이 됩니다.
    .keyword = 부재

elsewhere-busy = { $who }이 이 노드의 디렉터리를 가지고 있고, 이것을
    넘겨받을 수 있는 철야가 아닙니다. 여기서는 아무것도 읽거나 쓰지
    않았습니다. 그것이 끝나면 다시 실행하십시오.
    .keyword = 점유

elsewhere-busy-cannot-be-handed = { $who }이 이 노드의 디렉터리를 가지고 있습니다.
    이 시스템에서는 실행 중인 333에게 아직 다른 터미널에서 무엇도
    넘길 수 없어서, 여기서는 아무것도 읽거나 쓰지 않았습니다. 그쪽
    화면에서 `:` 뒤에 입력하거나, 그것을 멈추고 다시 실행하십시오.
    .keyword = 점유

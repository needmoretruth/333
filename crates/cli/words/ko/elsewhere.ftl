### 다른 333이 이미 이 노드의 디렉터리를 가지고 있을 때 명령이 하는 말.

elsewhere-the-vigil = 이 디렉터리에서 실행 중인 노드
elsewhere-the-vigil-by-number = 이 디렉터리에서 실행 중인 노드(프로세스 { $pid })
elsewhere-another = 다른 333
elsewhere-another-by-number = 다른 333(프로세스 { $pid })

elsewhere-done = { $vigil }이(가) 수행했습니다.
    .keyword = 완료

elsewhere-failed = { $vigil }이(가) 그 일을 하지 못했습니다.
    .keyword = 실패

elsewhere-finding-its-name = { $who }이(가) 아직 노드 이름을 찾는 중입니다.
    이름이 생긴 뒤 다시 실행하십시오.
    .keyword = 점유

elsewhere-already-keeping = { $who }이(가) 이미 여기서 실행 중이고, 한 디렉터리는 한
    노드입니다. 여기서 명령을 보낼 수 있습니다: `333 say 7`,
    `333 join <초대장>`, `333 tell 'tor on'`, `333 stop`. 노드를 하나 더
    돌리려면 --data-dir로 다른 디렉터리를 주십시오.
    .keyword = 점유

elsewhere-keeping = { $who }이(가) 여기서 실행 중이고,
    { $why }
    .keyword = 점유

elsewhere-nobody-to-tell = 이 디렉터리에서 실행 중인 노드가 없어서 보낼 곳이 없습니다.
    `333 start`나 `333 run`으로 켜면 됩니다.
    .keyword = 부재

elsewhere-busy = { $who }이(가) 이 노드의 디렉터리를 쓰고 있고, 명령을
    받을 수 있는 실행 중 노드가 아닙니다. 아무것도 읽거나 쓰지
    않았습니다. 그것이 끝난 뒤 다시 실행하십시오.
    .keyword = 점유

elsewhere-busy-cannot-be-handed = { $who }이(가) 이 노드의 디렉터리를 쓰고 있습니다.
    이 시스템에서는 다른 터미널에서 실행 중인 333에 명령을 보낼 수
    없어서 아무것도 읽거나 쓰지 않았습니다. 그쪽 화면에서 `:` 뒤에
    입력하거나, 그것을 끄고 다시 실행하십시오.
    .keyword = 점유

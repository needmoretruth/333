### `333 start` and `333 restart`: running this node in the background.

start-no-node = { $home }에 아직 노드가 없습니다

start-no-node-next = 초대장이 있으면 `333 join <초대장>`으로 파일을 받습니다. 아무도
    없을 때만 `333 begin`으로 새 줄을 시작합니다.

start-in-a-terminal = 이미 터미널에서 돌고 있습니다. 그곳에서 끄거나 `333 stop`으로 끈
    뒤 `333 start`를 실행하면 백그라운드에서 켜집니다.
    .keyword = 실행

start-already = 이미 백그라운드에서 돌고 있습니다.
    .keyword = 실행

start-started = 백그라운드에서 켰습니다. 재부팅 뒤에도 켜집니다. 상태는
    `333 status`, 끄려면 `333 stop`.
    .keyword = 켜짐

start-elsewhere = 이 기계의 백그라운드 서비스가 { $other }의 노드를 돌리고 있습니다.
    `333 service uninstall`로 지운 뒤 다시 `333 start`를 실행하십시오.

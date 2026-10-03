### `333 serve`: 이 기계의 다른 터미널에서 오는 명령.

serve-told-not-here = 이 시스템에서는 명령을 이 노드의 화면으로만 받습니다. 옆에서
    켠 두 번째 333은 거절합니다.
    .keyword = 명령

serve-told-cannot = 다른 터미널에서 명령을 받을 수 없습니다: { $why }
    .keyword = 명령

serve-told-not-private = 소켓을 이 계정 전용으로 만들 수 없었습니다

serve-told-taking = 이 기계의 어느 터미널에서든 화면과 같은 말로 명령할 수 있습니다:
    `333 say 7`, `333 join <초대장>`, `333 tell 'tor on'`, `333 stop`.
    이 노드가 수행하고 그 터미널에 답합니다.
    .keyword = 명령

serve-told-taking-light = 이 기계의 어느 터미널에서든 명령할 수 있습니다:
    `333 say 7`, `333 join <초대장>`, `333 tell 'tor on'`, `333 stop`.
    이 노드가 수행하고 그 터미널에 답합니다.
    .keyword = 명령

serve-told-old-socket = 옛 소켓이 남아 있어 그대로 둡니다: { $why }
    .keyword = 명령

serve-told-not-a-socket = { $path }이(가) 소켓이 아니어서 건드리지 않습니다. 그것을 옮기기
    전에는 다른 터미널에서 이 노드에 명령할 수 없습니다.
    .keyword = 명령

serve-told-no-longer = 이제 다른 터미널에서 명령을 받지 않습니다: { $why }
    .keyword = 명령

serve-told-not-the-owner = 이 노드 디렉터리의 주인 계정만 명령할 수 있습니다
    .keyword = 거부

serve-told-too-long = 명령으로 보기에 너무 깁니다. 가장 긴 명령은 { $bytes }바이트입니다.
    .keyword = 못읽음

serve-told-other-version = 이 노드는 { $ours }이고 요청은 { $theirs }(으)로 왔습니다. 버전이
    다르니 실행 중인 것과 같은 버전의 333으로 다시 실행하십시오.
    .keyword = 거부

serve-told-unread = { $why }
    .keyword = 못읽음

serve-told-asked = { $order }, 다른 터미널에서
    .keyword = 요청

serve-told-unheard = 명령을 수행할 노드가 없습니다
    .keyword = 못들음

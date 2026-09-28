### `333 serve`: 이 기계의 다른 터미널에서 오는 명령.

serve-told-not-here = 이 시스템에서는 명령이 이 철야의 화면으로만 들어옵니다. 옆에서
    시작한 두 번째 333은 거부합니다.
    .keyword = 명령

serve-told-cannot = 다른 터미널에서 받을 수 없습니다: { $why }
    .keyword = 명령

serve-told-not-private = 소켓을 비공개로 만들 수 없었습니다

serve-told-taking = 이 기계의 어느 터미널에서든, 화면에서 쓰는 말 그대로:
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`.
    이 철야가 그것을 수행하고 그 터미널에 답합니다.
    .keyword = 명령

serve-told-old-socket = 옛 소켓이 자리를 막고 있고 그대로 남습니다: { $why }
    .keyword = 명령

serve-told-not-a-socket = { $path }이(가) 있지만 소켓이 아니어서 그대로 둡니다. 그것을
    옮기기 전까지는 다른 터미널에서 이 철야에 아무것도 건넬 수
    없습니다.
    .keyword = 명령

serve-told-no-longer = 이제 다른 터미널에서 받지 않습니다: { $why }
    .keyword = 명령

serve-told-not-the-owner = 이 노드의 디렉터리 주인만 이 노드에 무언가를 말할 수 있습니다
    .keyword = 거부

serve-told-too-long = 어떤 명령보다도 깁니다. 가장 긴 명령은 { $bytes }바이트입니다.
    .keyword = 못읽음

serve-told-other-version = 이 철야는 { $ours }을(를) 말하는데 { $theirs }(으)로 요청받았습니다.
    요청한 333은 철야를 지키는 것과 다른 버전입니다. 그쪽을 실행하십시오.
    .keyword = 거부

serve-told-unread = { $why }
    .keyword = 못읽음

serve-told-asked = { $order }, 다른 터미널에서
    .keyword = 요청

serve-told-unheard = 이제 명령을 수행하는 것이 없습니다
    .keyword = 못들음

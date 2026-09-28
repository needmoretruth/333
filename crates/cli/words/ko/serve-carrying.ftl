### `333 serve`: 화면이나 다른 터미널에서 철야가 받은 명령을 수행하기.

serve-carrying-not-written-down = 주소를 적는 중: { $why }
    .keyword = 실패

serve-carrying-not-written-who = 누가 답했는지 적는 중: { $why }
    .keyword = 실패

serve-carrying-unheard = { $why }
    .keyword = 못들음

serve-carrying-unbegun = { $why }
    .keyword = 미시작

serve-carrying-refused = { $why }
    .keyword = 거부

serve-carrying-holding-the-file = 명부에 우리 { $roll }명이 있고, 파일은 여기 있습니다
    .keyword = 보유

serve-carrying-holding-no-file = 명부에 우리 { $roll }명이 있고, 이 노드는 아직 파일을 받지 못했습니다
    .keyword = 보유

serve-carrying-unread-holding = 이 노드가 가진 것: { $why }
    .keyword = 못읽음

serve-carrying-already-up = 보이지 않는 주소가 이미 올라 있습니다. `tor off`로 내립니다.
    .keyword = 그대로

serve-carrying-unraised = { $why }
    .keyword = 못올림

serve-carrying-tor-off = 어니언 주소가 이제 답하지 않습니다. 그 주소에 대해 이미 한 말은
    잊힐 때까지, 곧 말한 때로부터 두 에포크 동안 남습니다.
    .keyword = 어니언

serve-carrying-none-up = 내릴 보이지 않는 주소가 없습니다.
    .keyword = 그대로

serve-carrying-too-late = Tor가 이미 돌고 있어서, 지금 더한 브리지는 이미 맺은 연결에
    아무것도 바꾸지 않습니다. 대신 그 브리지를 주고 노드를 다시
    시작하십시오.
    .keyword = 늦음

serve-carrying-bridged = 다음에 Tor가 시작할 때 브리지 { $bridges }개를 씁니다.
    .keyword = 브리지

serve-carrying-helper = 난독화된 브리지에는 { $program }을(를) 실행합니다.
    .keyword = 브리지

serve-carrying-not-an-address = { $typed }은(는) 주소가 아닙니다: { $why }
    .keyword = 못읽음

serve-carrying-cannot-end = 다른 터미널은 이 철야를 끝낼 수 없습니다. 철야는 시작한 곳에서
    끝납니다. 화면에서 `q`, Ctrl-C, 또는 철야를 지키는 서비스 관리자로.
    .keyword = 거부

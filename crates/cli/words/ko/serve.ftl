### `333 serve`.

serve-nothing-listening = 아무것도 듣고 있지 않게 됩니다: --no-direct에는 --tor가 필요합니다

serve-name = { $name }
    .keyword = 이름

serve-waiting-for-the-file = 이 노드는 아직 파일을 받지 못해서 아무것도 세지 않고, 아직
    누구도 증언할 것이 없습니다. 파일은 만들 수 없습니다. 이미
    가진 사람에게서만 오고, 건넬 때 두 사람이 함께 서명합니다.
    초대장을 부탁한 뒤 `333 join 333:their.address:3333`을
    실행하십시오. 그동안 응답하는 데는 아무 비용이 들지 않고,
    사람들은 그렇게 당신을 찾습니다.
    .keyword = 대기

serve-hand = 초대장은 사람이 아니라 장소를 가리킵니다. 무엇도 보증하지
    않습니다. 그곳에서 답하는 이는 키를 가진 것으로 자신을 증명합니다.
    .keyword = 초대장

serve-invite = { $invitation }
    .keyword = 초대

serve-answer = { $bound }
    .keyword = 응답

serve-nearby = 이 네트워크에 333을 말하는 무언가가 여기 있다고 알리고, 다른
    노드를 듣고 있습니다. 이 노드의 이름은 나가지 않습니다. 나가는
    것은 같은 네트워크를 포트 스캔하면 찾을 수 있는 것입니다.
    --no-mdns를 주면 이 노드는 여기에 나서지 않습니다.
    .keyword = 이웃

serve-nearby-failed = 이 노드가 여기 있다고 이 네트워크에 알리기 시작하지 못했습니다: { $why }
    .keyword = 이웃

serve-meet = { $place }에서 이 노드는 아무도 소개해 주지 않은 사람들을 찾습니다.
    그곳에서 읽는 것은 모두 말한 이가 서명한 것이고, 그곳의 어떤
    것도 믿지 않습니다. --no-meet를 주면 이 노드는 그곳에 가지 않습니다.
    .keyword = 만남

serve-listener-stopped = 듣던 것 하나가 뜻밖에 멈췄습니다

serve-farewell = 에포크 { $epoch }에 끝났습니다. 이것이 돌지 않는 동안 당신에게
    묻도록 뽑힌 이는 물었지만 아무것도 듣지 못했다고 서명하고,
    당신의 창에는 그것이 적힙니다. 창의 길이는 { $window }에포크이고,
    창은 움직입니다.
    .keyword = 철야

serve-farewell-on-no-roll = 에포크 { $epoch }에 끝났습니다. 당신은 어느 명부에도 없어서 아무도
    당신을 찾아가 묻지 않고, 이것이 돌지 않는 동안 당신에 대해 서명되는
    것도 없습니다.
    .keyword = 철야

### `333 serve`: 만남의 장소에 이 노드의 주소를 남기고, 다른 이들의 주소를 읽기.

hours-meeting-unreadable = { $place }을(를) 읽을 수 없습니다: { $why }
    .keyword = 만남

hours-meeting-read-failed-inside = { $place }을(를) 읽는 일이 이 노드 안에서 실패했습니다: { $why }
    .keyword = 만남

hours-meeting-left = { $place }에 이 노드의 주소를 남겼습니다
    .keyword = 만남

hours-meeting-stopped = { $place }이(가) 답하기 전에 이 노드가 멈췄습니다
    .keyword = 만남

hours-meeting-leaving-failed-inside = { $place }에 주소를 남기는 일이 이 노드 안에서 실패했습니다: { $why }
    .keyword = 만남

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place }은(는) 인터넷 주소마다 분당 진술 하나를 받는데,
    이 주소에서는 채 일 분이 지나기 전에 하나를 받았습니다.{ $holding }
    이 노드는 { $when } 주소를 다시 남깁니다.
    .keyword = 만남

hours-meeting-full = { $place }은(는) 하루에 받는 진술을 다 받았고 UTC 자정이
    지나면 더 받습니다. 읽는 것은 여전히 됩니다.{ $holding }
    이 노드는 { $next_epoch } 주소를 다시 남깁니다.
    .keyword = 만남

hours-meeting-full-until = { $place }은(는) 하루에 받는 진술을 다 받았고 UTC 자정이
    지나면, 곧 { $midnight } 뒤에 더 받습니다. 읽는 것은 여전히 됩니다.{ $holding }
    이 노드는 { $next_epoch } 주소를 다시 남깁니다.
    .keyword = 만남

hours-meeting-holds-from = 그곳에는 아직 에포크 { $epoch }의 이 노드 주소가 있습니다.
hours-meeting-holds-nothing = 그곳에는 이 노드에게서 받은 것이 없습니다.

hours-meeting-at-the-next-epoch = 다음 에포크에, { $wait } 뒤에
hours-meeting-in = { $wait } 뒤에

hours-meeting-did-not-reach = 이 노드의 주소가 { $place }에 닿지 않았습니다: { $why }
    .keyword = 만남

hours-meeting-not-taken = { $place }이(가) 이 노드의 주소를 받지 않았습니다: { $why }
    .keyword = 만남

hours-meeting-seconds = { $seconds }초

hours-meeting-nobody = { $place }에서 어디 있는지 말하는 이가 아무도 없습니다
    .keyword = 만남

hours-meeting-newer = { $place }에 더 새로운 주소 { $fresh }개
    .keyword = 만남

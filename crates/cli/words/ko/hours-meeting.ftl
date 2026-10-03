### `333 serve`: 만남의 장소에 이 노드의 주소를 남기고, 다른 이들의 주소를 읽기.

hours-meeting-unreadable = { $place }을(를) 읽을 수 없습니다: { $why }
    .keyword = 만남

hours-meeting-read-failed-inside = { $place }을(를) 읽다가 이 노드 안에서 실패했습니다: { $why }
    .keyword = 만남

hours-meeting-left = { $place }에 이 노드의 주소를 남겼습니다
    .keyword = 만남

hours-meeting-stopped = { $place }이(가) 답하기 전에 이 노드가 멈췄습니다
    .keyword = 만남

hours-meeting-leaving-failed-inside = { $place }에 주소를 남기다가 이 노드 안에서 실패했습니다: { $why }
    .keyword = 만남

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place }은(는) 인터넷 주소 하나에서 1분에 하나만 받습니다.
    이 주소에서 1분 안에 이미 하나를 받았습니다.{ $holding }
    { $when } 다시 남깁니다.
    .keyword = 만남

hours-meeting-full = { $place }이(가) 오늘 받을 수 있는 만큼 받았습니다. UTC 자정이
    지나면 다시 받습니다. 읽기는 됩니다.{ $holding }
    { $next_epoch } 다시 남깁니다.
    .keyword = 만남

hours-meeting-full-until = { $place }이(가) 오늘 받을 수 있는 만큼 받았습니다. { $midnight } 뒤
    UTC 자정에 다시 받습니다. 읽기는 됩니다.{ $holding }
    { $next_epoch } 다시 남깁니다.
    .keyword = 만남

hours-meeting-holds-from = 그곳에 에포크 { $epoch }에 남긴 주소가 아직 있습니다.
hours-meeting-holds-nothing = 그곳에 이 노드의 주소가 없습니다.

hours-meeting-at-the-next-epoch = 다음 에포크({ $wait } 뒤)에
hours-meeting-in = { $wait } 뒤에

hours-meeting-did-not-reach = 이 노드의 주소가 { $place }에 닿지 않았습니다: { $why }
    .keyword = 만남

hours-meeting-not-taken = { $place }이(가) 이 노드의 주소를 받지 않았습니다: { $why }
    .keyword = 만남

hours-meeting-seconds = { $seconds }초

hours-meeting-nobody = { $place }에 남은 주소가 없습니다
    .keyword = 만남

hours-meeting-newer = { $place }에서 새 주소 { $fresh }개를 받음
    .keyword = 만남

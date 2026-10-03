### `333 run`: leaving this node's address at a meeting point, and reading everyone else's.

hours-meeting-unreadable = { $place } を読めませんでした: { $why }
    .keyword = 集合

hours-meeting-read-failed-inside = { $place } の読み込みがこのノード内で失敗しました: { $why }
    .keyword = 集合

hours-meeting-left = このノードのアドレスを { $place } に残しました
    .keyword = 集合

hours-meeting-stopped = { $place } が応答する前にこのノードが止まりました
    .keyword = 集合

hours-meeting-leaving-failed-inside = このノードのアドレスを { $place } に残す処理が
    このノード内で失敗しました: { $why }
    .keyword = 集合

# { $holding } is empty, or one of the two lines below on a line of its own.
hours-meeting-not-yet = { $place } は各インターネットアドレスから一分に一つ言明を
    受け付け、このアドレスからは一分以内に受けています。{ $holding }
    このノードは { $when } にまたアドレスを残します。
    .keyword = 集合

hours-meeting-full = { $place } は一日分の言明を受け付け終え、UTC の真夜中を
    過ぎるとまた受け付けます。読むことはできます。{ $holding }
    このノードは { $next_epoch } にまたアドレスを残します。
    .keyword = 集合

hours-meeting-full-until = { $place } は一日分の言明を受け付け終え、UTC の真夜中、
    { $midnight } 後にまた受け付けます。読むことはできます。{ $holding }
    このノードは { $next_epoch } にまたアドレスを残します。
    .keyword = 集合

hours-meeting-holds-from = エポック { $epoch } のこのノードのアドレスをまだ保っています。
hours-meeting-holds-nothing = このノードのものは何も保っていません。

hours-meeting-at-the-next-epoch = 次のエポック、{ $wait } 後
hours-meeting-in = { $wait } 後

hours-meeting-did-not-reach = このノードのアドレスが { $place } に届きませんでした: { $why }
    .keyword = 集合

hours-meeting-not-taken = { $place } がこのノードのアドレスを受け付けませんでした: { $why }
    .keyword = 集合

hours-meeting-seconds = { $seconds ->
       *[other] { $seconds } 秒
    }

hours-meeting-nobody = { $place } で居場所を告げている人はいません
    .keyword = 集合

hours-meeting-newer = { $fresh ->
       *[other] { $place } に新しいアドレスが { $fresh } 件
    }
    .keyword = 集合

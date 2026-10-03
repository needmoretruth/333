### The screen: what it says in the log pane about what was typed into it.

screen-asked = { $typed }
    .keyword = 依頼
screen-unheard = もう指示を実行するものがありません
    .keyword = 未聴取
screen-unread = { $why }
    .keyword = 読めず
screen-refused = { $why }
    .keyword = 拒否

screen-at = { $hours }:{ $minutes }:{ $seconds }

screen-key-unreadable = キーを読めませんでした: { $why }。後のキーは読めるかもしれません。
    .keyword = キー
screen-keyboard-gone = キーボードを読めなかった ({ $why }) ので、画面を閉じ、
    ノードも止まりました。`333 run --plain` ならキーボードなしで
    ノードを動かします。

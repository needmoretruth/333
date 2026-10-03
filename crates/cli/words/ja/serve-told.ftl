### `333 run`: orders from other terminals on this machine.

serve-told-not-here = このシステムでは、指示は画面からしかこのノードに届きません。
    横で二つ目の 333 を起動すると拒否されます。
    .keyword = 指示

serve-told-cannot = 別の端末からは受け付けられません: { $why }
    .keyword = 指示

serve-told-not-private = ソケットを非公開にできませんでした

serve-told-taking = このマシンのどの端末からでも、画面の言葉で:
    `333 say 7`、`333 join <invitation>`、`333 tell 'tor on'`、`333 stop`。
    このノードが実行し、そこで答えます。
    .keyword = 指示

serve-told-taking-light = このマシンのどの端末からでも:
    `333 say 7`、`333 join <invitation>`、`333 tell 'tor on'`、`333 stop`。
    このノードが実行し、そこで答えます。
    .keyword = 指示

serve-told-old-socket = 古いソケットが邪魔をしていて、そのまま残ります: { $why }
    .keyword = 指示

serve-told-not-a-socket = { $path } はあるもののソケットではないので触れません。それを
    動かすまで、別の端末からこのノードに何も渡せません。
    .keyword = 指示

serve-told-no-longer = 別の端末からはもう受け付けません: { $why }
    .keyword = 指示

serve-told-not-the-owner = このノードのディレクトリの持ち主しか指示できません
    .keyword = 拒否

serve-told-too-long = どの指示よりも長すぎます。最長は { $bytes } バイトです。
    .keyword = 読めず

serve-told-other-version = このノードは { $ours } を話しますが、{ $theirs } で尋ねられました。
    尋ねた 333 は動作中のものと別のバージョンです。そちらを使ってください。
    .keyword = 拒否

serve-told-unread = { $why }
    .keyword = 読めず

serve-told-asked = { $order }、別の端末から
    .keyword = 依頼

serve-told-unheard = もう指示を実行するものがありません
    .keyword = 未聴取

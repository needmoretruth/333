### `333 run`.

serve-nothing-listening = 何も待ち受けないことになります: --no-direct には --tor が必要です

serve-name = { $name }
    .keyword = 名前

serve-waiting-for-the-file = このノードはファイルを受け取っていないので、まだ何も数えられず、
    誰かが証言できることもありません。ファイルは自分では作れません。
    すでに持っている人からしか届かず、受け渡しには二人とも署名します。
    招待状を求め、`333 join 333:相手のアドレス:3333` を実行してください。
    それまで応答しておくのは無料で、人に見つけてもらう方法です。
    .keyword = 待機

serve-hand = 招待状が指すのは人ではなく場所です。そこで応答する者は、
    鍵を持つことで自分が誰かを証明します。
    .keyword = 信頼

serve-invite = { $invitation }
    .keyword = 招待状

serve-answer = { $bound }
    .keyword = 応答

serve-nearby = このネットワークで、ここに 333 を話すものがいると告げ、ほかを
    聞いています。このノードの名前は出しません。出ていくのは、同じ
    ネットワークのポートスキャンで見つかる程度のことです。
    --no-mdns で外します。
    .keyword = 近く

serve-nearby-failed = このネットワークでノードの存在を告げ始められませんでした: { $why }
    .keyword = 近く

serve-meet = { $place } は、誰にも紹介されていない相手をこのノードが探す場所
    です。そこで読むものは言った本人が署名していて、そこにあるものは
    何も信じません。--no-meet で近づかないようにします。
    .keyword = 集合

serve-listener-stopped = 待ち受けが予期せず止まりました

serve-farewell = エポック { $epoch } に終了。これが動いていない間にあなたのことを
    尋ねるよう抽選された人は、尋ねて何も聞こえなかったと署名し、
    あなたの期間はそれを読みます。期間は { $window } エポックで、
    動いていきます。
    .keyword = ノード

serve-farewell-on-no-roll = エポック { $epoch } に終了。どの名簿にも載っていないので、
    あなたのことを尋ねに行く人はおらず、これが動いていない間に
    あなたについて署名されることもありません。
    .keyword = ノード

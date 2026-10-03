### `333 status`: what this node saw of everybody else, what was said, and whether
### anybody is here.

status-name = { $name }
    .keyword = 名前

status-epoch = { $epoch }
    .keyword = エポック

status-epoch-in-line = { $epoch }、{ $line }
    .keyword = エポック

status-the-line = この系譜の { $nth }{ $kind ->
       *[other] 番目
    }

status-answering = 応答中
status-silent = 沈黙
status-roll = 名簿

status-seen = 最初の数は、エポック { $before } か { $now } の署名をこのノードが
    持っている全員です。このノードが見たものであり、ほかの誰かは
    別のものを見ています。

status-seen-without-tor = このビルドは隠れた道を通れないので、隠れている仲間はこの数に
    入っておらず、これからも入りません。

status-how-many-people = それが何人なのか、このノードは知らず、知りようもありません。
    わかるのは、その名前のどれもがその二つのエポックのどちらかで
    応答し、数えられたい限り、次も、その次も応答し続けなければ
    ならないことです。一人が千の名前を持つなら、その人は千の分を
    時間ごとに払い続け、やめた時から数えられなくなります。

status-given-by = 渡した人
status-you = あなた
status-received-in = エポック { $epoch } に受領
status-trail-stops = 足取りはここで途切れます。

status-stopped-knowing = ここはこのノードが知るのをやめた所で、始まりではありません。
    最初の仲間は誰からもファイルを受け取らず、どこにも加入記録が
    ありません。このノードにまだ届いていない記録も、ここからは
    同じように見えます。

status-nothing-said = エポック { $epoch } には誰も何も言っていません。言えることは 333 あり、
    まだどれにも言葉はありません。

status-said = エポック { $epoch } の発言 — このノードから見える { $seen } 人のうち
    { $spoke } 人が話し、{ $silent } 人は話していません。
status-a-third = ← 仲間の三分の一以上
status-not-said = 333 のうち残りの { $others } は言われていません。

status-no-winner = 勝者は決めず、これは何も決めません。このノードに届いたもの
    です。隣のノードは別のことを聞いていて、それも間違いではありません。

status-reading-the-watch = 見張りの記録を読み込み中

status-seen-nobody = 途切れなく見張った { $watched } の間、このノードに誰も応答して
    いませんが、このビルドはそれを終わりとは呼びません。隠れた道を
    通れないので、隠れている仲間からは聞いたことがなく、これからも
    聞けません。言えるのは誰も見ていないということで、それは同じ
    文ではありません。

status-never-answered = このノードに応答した人はまだいません。何の証拠でもありません。
    まだどこにも行っていないノードはこう見えます。

status-somebody-is-here = 誰かがここにいます。計算にこれ以上負うものはありません。

status-waiting = { $silent } の間、誰も応答していません。このノードはそれについて
    何も言っておらず、{ $needed } までは言いません。その間ずっと
    動いていた場合に限ります。

status-nobody-keeping = 誰も応答していません

    ここにいるのはあなただけです。途切れなく見張った { $watched }
    (七十七日) の間、このノードに誰も応答せず、最後の仲間は
    エポック { $since } に止まりました。

    333 は消えたのではありません。消えていくところで、それには
    { $years } 年かかります。

status-remain = 残り { $years } 年 { $days } 日。
status-run-out = 最後の年も尽きました。

status-one-answer = この計時は、あなたが気づいた時ではなく、最後の仲間が応答を
    やめた時に始まりました。あなたが見ている間もずっと進んでいました。

    応答が一つあれば終わります。どこかの誰かがこのノードに応答すれば
    これは消え、計時は止まるのではなく捨てられます。333 はどこまで
    近づいたかを記録しません。

status-epochs = { $count ->
       *[other] { $count } エポック
    }

status-share = { $whole }.{ $after }%

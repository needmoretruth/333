### `333 bootstrap`: beginning a line of your own, when there is nobody to join.

bootstrap-name = { $name }
    .keyword = 名前

bootstrap-vigil = `333 start` で動かすと応答するようになります。
    .keyword = ノード

bootstrap-already-has-it = このノードはもうファイルを持っています。始めることはありません。

bootstrap-stop = { $already ->
       *[other] { $already } 人の仲間が
    }{ $meet } で居場所を伝えています。
    いま一人で始めると、理由もなくその系譜の横に二つ目の系譜が
    できます。ブラウザで { $board } を開き、招待状を一つ選んで、
    それで `333 join` を実行してください。
    これを読んだうえで始めるなら `--anyway` を付けてください。
    .keyword = 停止

bootstrap-not-the-file = 届いたものはファイルではありません

bootstrap-begun = ファイルはこのノードのディレクトリにあり、このノードは自分の
    系譜の始まりです。誰も渡していないので、受け渡しに署名した人は
    いません。このノードの記録を読めば誰でもそれがわかります。

    これは創始者の立場で、普通の立場ではありません。名簿に入るのは
    ファイルを受け取った者なので、誰からも受け取っていないノードは
    どの名簿にも載りません。誰も質問しに来ず、このノードが誰かに
    質問する抽選に当たることもありません。ただし、このノードに
    質問するよう抽選で選ばれた相手のもとへ行き、証言を得ることは
    できます。

    この後にファイルを渡した相手は、普通の手順で二人とも署名して
    名簿に入り、その時から数えられます。
    .keyword = 開始

bootstrap-reading-the-board = { $place } の掲示板を読み込み中

bootstrap-asking = { $meet } にファイルを求めています
    .keyword = 要求

bootstrap-asking-for-the-file = { $meet } にファイルを求めています

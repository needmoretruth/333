### `333 say`, and saying from the screen.

say-not-one = 全部で { $count } 個、0 から { $last } まで番号があります。
    「{ $typed }」はそのどれでもありません。

say-no-such = 全部で { $count } 個、0 から { $last } まで番号があります。{ $index } はありません。

say-not-joined = まだ誰からもファイルを受け取っていないので、何を言っても
    数えられません。先に受け取ってください: `333 join <invitation>`。

say-already = エポック { $epoch } ではすでに { $index } 番を言いました。一人一つで、
    言い直しても置き換わりません。ノードが最初に言ったことが、
    そのノードの言ったことです。

say-sealing = 言ったことを封印中

say-said = エポック { $epoch } に { $index } 番
    .keyword = 発言

# Said under the line before, in its column, with no keyword of its own.
say-goes-out = このノードが届く全員に伝わり、そこからさらに伝わります。
    333 分ごとに 333 のうち一つを言え、それは番号で言います。
    334 番目はなく、これからもありません。二度は言えず、大きな声でも
    言えず、生きている誰もあなたより多くは言えません。
    その意味も誰も教えてくれません。表はまだありません。できた時は
    私たち全員に同じ表で、翻訳されません。
    .keyword = {""}

say-given-by-nobody = ファイルを持っていますが、誰からも渡されていないので、
    どの名簿にも載っておらず、言ったことを数える人もいません。
    話せるのは誰かからファイルを渡されたノードだけです。渡せば、
    渡した相手が話せるようになります。

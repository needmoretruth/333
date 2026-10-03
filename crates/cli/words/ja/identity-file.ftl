### This node's identity on disk: reading it, making it, and refusing it.

-identity-file-trust = --dangerously-trust-directory-permissions
-identity-file-the-small-machine = 戸棚の小さなマシン
-identity-file-nothing-here = ここに宛てたものは何もありません。

identity-file-reading = { $path } を読み込み中
identity-file-making-home = { $home } をこのノードの住処にしています
identity-file-creating = { $path } を作成中
identity-file-writing = { $path } を書き込み中

identity-file-private = このノードの身元すべてが入っているので、他人が触れては
    いけません。直すには: { $fix } { $path }
    手放すものを理解しているなら、{ -identity-file-trust } を付けます

identity-file-wrong-size = { $path } は { $bytes } バイトです。シードはちょうど { $seed } バイトです

identity-file-cursed = 333 はその名前を見て、あなたの命から { $pause } ミリ秒を奪いました。

    { $name }
    は呪われています。裁きは一度下され、解かれることはありません。
    その名前を持って行く扉ごとに、{ $pause } ミリ秒がまた奪われます。

    333 はとても寛大です。三つのエポックのうち一つは休んでも
    仲間のままです。遅い者にも、貧しい者にも、
    { -identity-file-the-small-machine }にも、まだ生まれていない
    すべての者にも寛大です。異端者には寛大ではありません。

identity-file-ineligible = それは 333 が応える名前ではありません。

    { $name }
    は 333 で始まらないので、{ -identity-file-nothing-here }
    あなたから奪ったものもありません。333 はあなたを見てもいません。

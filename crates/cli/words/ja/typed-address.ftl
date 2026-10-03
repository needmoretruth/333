### Reading an address somebody typed: what was wrong with one, and how one is written.

typed-address-refused = { $why }。アドレスは node.example:3333 のような host:port で、
    招待状は 333:node.example:3333 のように 333: とアドレスです。
typed-address-refused-announce = { $why }。アドレスは node.example:3333 のような host:port です。
typed-address-refused-bind = { $typed } は待ち受け用のアドレスではありません。0.0.0.0:3333
    のような IP アドレスとポートです。アドレスだけならポート 3333
    で、:port だけならすべてのアドレスで待ち受けます。

typed-address-no-tag = 招待状は 333: で始まります
typed-address-too-long = 招待状は最大 { $most } 文字で、これは { $length } 文字です
typed-address-not-canonical = 仲間一人は一つの場所で、書き方も一つです。招待状は
    { $canonical } です
typed-address-wrong-tag = 招待状は { $number }: ではなく 333: で始まります
typed-address-empty = アドレスが指定されていません
typed-address-bad-port = { $port } はポートではありません。ポートは 1 から 65535 の数です
typed-address-unclosed = [ で始まるアドレスは ] で閉じる必要があります
typed-address-scheme = { $scheme }:// はウェブのアドレスのもので、ここのアドレスではありません
typed-address-not-a-host = 「{ $host }」はホスト名でも IP アドレスでもありません
typed-address-not-an-onion = { $host } は onion アドレスではありません。onion アドレスには
    英数字 { $letters } 文字の後に .onion が付きます

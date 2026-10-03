### `333 run`: asking the router, keeping what it lent, and giving it back.

serve-reach-router-opened-upnp = ルーターによると、ポート { $port } { $on } が今このマシンに
    来ます。取り消したければ、ルーターに `333` として載っています。
    何か届くかは次の行でわかります。
    .keyword = 開放

serve-reach-router-opened-upnp-for = ルーターによると、ポート { $port } { $on } が { $time } の間、
    このマシンに来ます。取り消したければ、ルーターに `333` として
    載っています。何か届くかは次の行でわかります。
    .keyword = 開放

serve-reach-router-opened-lease = { $router } のルーターに { $way } でポート { $port } を { $asked_for }
    求めました。ポート { $granted_port } { $on } が { $granted } の間ここへ
    来るとのことです。このノードは期限前に頼み直し、止まるときに
    返します。強制終了された場合は、期限が来るとルーターが手放します。
    何か届くかは次の行でわかります。
    .keyword = 開放

serve-reach-router-nobody-answered = ポート開放の依頼に、ここのどのルーターも UPnP-IGD、PCP、
    NAT-PMP のどれでも応答しませんでした。よくあることです。三つとも
    切っている機器は多く、自分のアドレスを持つマシンには頼むことが
    ありません。`--no-router` で頼むこと自体をやめます。
    .keyword = 閉鎖

serve-reach-router-refused = ルーターがポート { $port } を開けませんでした: { $why }
    .keyword = 閉鎖

serve-reach-router-let-go = ルーターがポート { $port } を手放しました。期限内に頼み直さ
    なかったので、外からそのポートでこのノードに届きません。ノードを
    再起動すると頼み直します。
    .keyword = 閉鎖

serve-reach-router-given-back = ポート { $port } を { $way } でルーターに返しました。もうこの
    マシンには来ません。
    .keyword = 閉鎖

serve-reach-router-not-taken-back = ルーターがポート { $port } を引き取りませんでした ({ $why })。
    { $time } 以内に自分で手放します。
    .keyword = 閉鎖

serve-reach-router-moved = ルーターがこのノードを移しました。ポート { $before_port } { $before_on }
    の代わりに、ポート { $port } { $on } がここに来ます。古い方を
    書いた招待状はもう届きません。
    .keyword = 開放

serve-reach-router-not-kept = 頼んだのにルーターがポート { $port } を保ちませんでした
    ({ $why })。まだ { $time } は持っていて、それまでに頼み直します。
    .keyword = 待機

serve-reach-router-on-its-outside-address = 外側のアドレスで
serve-reach-router-on = { $address } で

serve-reach-router-one-second = 1 秒
serve-reach-router-two-seconds = 2 秒
serve-reach-router-seconds = { $count } 秒
serve-reach-router-one-minute = 1 分
serve-reach-router-two-minutes = 2 分
serve-reach-router-minutes = { $count } 分
serve-reach-router-one-hour = 1 時間
serve-reach-router-two-hours = 2 時間
serve-reach-router-hours = { $count } 時間

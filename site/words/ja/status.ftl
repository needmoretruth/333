# Status: the network now, how it has been one epoch at a time, and this site's machine.

status-meta-title = 状態 · 333
status-meta-description = 333 ネットワークの今と、エポックごとのこれまでです。名簿のノード、応答しているノード、掲示板の言明、そしてこのサイトのノードとマシンが動き続けたかどうか。
status-heading = 状態
status-lede = このサイトのノードが 15 秒ごとにネットワークを見て、エポックごとに一度数字を書き留めます。
status-now-title = 現在
status-roll = 名簿（創始者を含む）
status-saying = 居場所を告げている
status-tor = そのうち Tor 経由
status-site-node = このサイトのノード
status-time-title = これまで
status-time-lede = エポックごとに一つ、エポックが終わる前にこのサイトのノードが最後に出した数字です。線が途切れたところは、誰も書き留めなかったエポックです。
status-chart-recent-title = 直近 333 エポック
status-chart-all-title = 書き留めたすべて
# Under each chart, the same numbers as text. Every $variable is a number or a date
# (YYYY-MM-DD, UTC), or a dash when there is none.
status-chart-summary = エポック { $first } から { $last } まで、{ $from } から { $to } まで。名簿: 最小 { $roll_low }、最大 { $roll_high }、最新 { $roll_latest }。応答: 最小 { $answering_low }、最大 { $answering_high }、最新 { $answering_latest }。
status-chart-too-few = この期間に書き留めたエポックが 2 つ未満なので、まだ描く線がありません。
status-machine-title = このサイトのマシン
status-release = リリース
status-deployed = デプロイ
status-observed = 最後にノードを見た時刻
status-age = { $seconds } 秒前。
status-observed-running = 動いていました。
status-observed-not-running = 動いていませんでした。
status-uptime = マシンの稼働時間
status-uptime-value = { $days } 日 { $hours } 時間
status-elsewhere = すべてのノードは <a href="{ $base }/network">ネットワークのページ</a> に、それぞれの場所は <a href="{ $base }/map">地図</a> にあります。
status-json = プログラム向けの同じ数字: <a href="/api/status">/api/status</a>。

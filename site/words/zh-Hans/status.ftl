# Status: the network now, how it has been one epoch at a time, and this site's machine.

status-meta-title = 状态 · 333
status-meta-description = 333 网络现在怎么样、一个纪元一个纪元过来又怎么样：名册上的节点、应答的节点、公告板上的声明，以及本站的节点和机器是否一直在运行。
status-heading = 状态
status-lede = 本站节点每 15 秒查看一次网络，每个纪元记下一次数字。
status-now-title = 现在
status-roll = 名册上，含创始者
status-saying = 说明自己在哪里
status-tor = 其中经由 Tor
status-site-node = 本站节点
status-time-title = 历来
status-time-lede = 每个纪元一个样本：纪元结束前本站节点给出的最后数字。线上的缺口是没有人记下的纪元。
status-chart-recent-title = 最近 333 个纪元
status-chart-all-title = 记下的全部
# Under each chart, the same numbers as text. Every $variable is a number or a date
# (YYYY-MM-DD, UTC), or a dash when there is none.
status-chart-summary = 纪元 { $first } 至 { $last }，{ $from } 至 { $to }。名册上：最低 { $roll_low }，最高 { $roll_high }，最新 { $roll_latest }。应答：最低 { $answering_low }，最高 { $answering_high }，最新 { $answering_latest }。
status-chart-too-few = 这段时间记下的纪元不到两个，所以还没有线可画。
status-machine-title = 本站的机器
status-release = 版本
status-deployed = 部署于
status-observed = 最后一次查看节点
status-age = { $seconds } 秒前。
status-observed-running = 它在运行。
status-observed-not-running = 它没有运行。
status-uptime = 机器已运行
status-uptime-value = { $days } 天 { $hours } 小时
status-elsewhere = 所有节点都列在<a href="{ $base }/network">网络页面</a>，它们在哪里则在<a href="{ $base }/map">地图</a>上。
status-json = 给程序用的同样数字：<a href="/api/status">/api/status</a>。

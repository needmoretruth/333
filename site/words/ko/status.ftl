# Status: the network now, how it has been one epoch at a time, and this site's machine.

status-meta-title = 상태 · 333
status-meta-description = 333 네트워크의 지금과 에포크별 지난 기록입니다. 명부에 오른 노드, 응답하는 노드, 게시판의 진술, 그리고 이 사이트의 노드와 서버가 계속 돌았는지.
status-heading = 상태
status-lede = 이 사이트의 노드가 15초마다 네트워크를 보고, 에포크마다 한 번 숫자를 적어 둡니다.
status-now-title = 지금
status-roll = 명부, 창시자 포함
status-saying = 위치를 말하는 노드
status-tor = 그중 Tor로
status-site-node = 이 사이트의 노드
status-time-title = 지난 기록
status-time-lede = 에포크마다 하나씩, 에포크가 끝나기 전 이 사이트의 노드가 마지막으로 낸 숫자입니다. 선이 끊긴 곳은 아무것도 적지 못한 에포크입니다.
status-chart-recent-title = 최근 333 에포크
status-chart-all-title = 적어 둔 전체
# Under each chart, the same numbers as text. Every $variable is a number or a date
# (YYYY-MM-DD, UTC), or a dash when there is none.
status-chart-summary = 에포크 { $first }부터 { $last }까지, { $from }부터 { $to }까지. 명부: 최저 { $roll_low }, 최고 { $roll_high }, 최근 { $roll_latest }. 응답: 최저 { $answering_low }, 최고 { $answering_high }, 최근 { $answering_latest }.
status-chart-too-few = 이 기간에 적어 둔 에포크가 둘보다 적어서 아직 그릴 선이 없습니다.
status-machine-title = 이 사이트의 서버
status-release = 릴리스
status-deployed = 배포
status-observed = 노드를 마지막으로 본 때
status-age = { $seconds }초 전.
status-observed-running = 돌고 있었습니다.
status-observed-not-running = 돌고 있지 않았습니다.
status-uptime = 서버 가동 시간
status-uptime-value = { $days }일 { $hours }시간
status-elsewhere = 모든 노드는 <a href="{ $base }/network">네트워크 페이지</a>에, 노드가 있는 곳은 <a href="{ $base }/map">지도</a>에 있습니다.
status-json = 프로그램용 같은 숫자: <a href="/api/status">/api/status</a>.

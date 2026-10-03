### `333 serve`: 소켓으로 듣는 쪽.

serve-socket-port-taken = 포트 { $port }번을 이미 다른 노드나 프로그램이 쓰고 있습니다.
    그것을 끄거나 --bind로 다른 포트를 주십시오.

serve-socket-not-here = { $ip }는 이 기계의 주소가 아닙니다. 모든 주소에서 받으려면
    --bind 0.0.0.0:{ $port }를 쓰십시오.

serve-socket-privileged = 1024보다 낮은 포트({ $port })는 관리자만 쓸 수 있습니다.
    더 높은 포트를 주십시오: --bind { $suggested }.

serve-socket-listening-on = { $bind }에서 연결을 받는 중

serve-socket-accepting = 연결 받기

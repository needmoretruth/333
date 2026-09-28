### `333 service install`: 서비스가 돌릴 프로그램이 내일도 그 자리에 있는지.
##
## 한 줄이 `{`로 끝나고 다음 줄이 `""}`로 시작하는 곳은, 출력되는 줄이 이 파일의
## 한 줄보다 넓은 곳입니다.

service-exe-asking = 이 프로그램이 어디 있는지 시스템에 묻기

service-exe-fleeting = 이 프로그램은 { $exe }에서 돌고 있고, 그곳은 { $why }입니다. {
    ""}그것을 가리키는 서비스는 그곳이 사라지는 날 멈춥니다. { $home }(으)로 {
    ""}복사한 뒤 그곳에서 `service install`을 실행하십시오.

service-exe-emptied = 시스템이 비우는 디렉터리
service-exe-build-directory = 빌드 디렉터리. 다음 빌드나 clean이 이것을 바꿉니다

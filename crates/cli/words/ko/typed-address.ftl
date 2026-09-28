### 누군가 입력한 주소 읽기: 무엇이 틀렸는지, 그리고 주소는 어떻게 쓰는지.

typed-address-refused = { $why }. 주소는 host:port 꼴입니다(예: node.example:3333). 초대는
    주소 앞에 333:을 붙인 것입니다(예: 333:node.example:3333).
typed-address-refused-announce = { $why }. 주소는 host:port 꼴입니다(예: node.example:3333).
typed-address-refused-bind = { $typed }에서는 들을 수 없습니다. IP 주소와 포트를 씁니다(예:
    0.0.0.0:3333). 주소만 쓰면 포트 3333에서 듣고, :port만 쓰면 모든
    주소에서 듣습니다.

typed-address-no-tag = 초대는 333:으로 시작합니다
typed-address-too-long = 초대는 { $most }자까지이고, 이것은 { $length }자입니다
typed-address-not-canonical = 우리 중 하나는 한 곳이고 한 가지로 씁니다. 초대는 { $canonical }입니다
typed-address-wrong-tag = 초대는 { $number }:이 아니라 333:으로 시작합니다
typed-address-empty = 주소가 주어지지 않았습니다
typed-address-bad-port = { $port }는 포트가 아닙니다. 포트는 1부터 65535까지의 수입니다
typed-address-unclosed = {"["}로 여는 주소는 ]로 닫아야 합니다
typed-address-scheme = { $scheme }://는 웹 주소에 붙는 것이고, 여기의 주소에는 붙지 않습니다
typed-address-not-a-host = "{ $host }"는 호스트 이름도 IP 주소도 아닙니다
typed-address-not-an-onion = { $host }는 onion 주소가 아닙니다. onion 주소는 .onion 앞에 글자와
    숫자 { $letters }개가 옵니다

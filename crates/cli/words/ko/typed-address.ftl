### 누군가 입력한 주소 읽기: 무엇이 틀렸는지, 그리고 주소는 어떻게 쓰는지.

typed-address-refused = { $why }. 주소는 host:port 꼴입니다(예: node.example:3333). 초대장은
    주소 앞에 333:을 붙입니다(예: 333:node.example:3333).
typed-address-refused-announce = { $why }. 주소는 host:port 꼴입니다(예: node.example:3333).
typed-address-refused-bind = { $typed }에서는 연결을 받을 수 없습니다. IP 주소와 포트를
    쓰십시오(예: 0.0.0.0:3333). 주소만 쓰면 포트 3333, :포트만 쓰면 모든
    주소에서 받습니다.

typed-address-no-tag = 초대장은 333:으로 시작합니다
typed-address-too-long = 초대장은 { $most }자까지입니다. 이것은 { $length }자입니다
typed-address-not-canonical = 주소는 한 가지 꼴로만 씁니다. 초대장은 { $canonical }입니다
typed-address-wrong-tag = 초대장은 { $number }:이 아니라 333:으로 시작합니다
typed-address-empty = 주소가 주어지지 않았습니다
typed-address-bad-port = { $port }는 포트가 아닙니다. 포트는 1부터 65535까지의 수입니다
typed-address-unclosed = {"["}로 여는 주소는 ]로 닫아야 합니다
typed-address-scheme = { $scheme }://는 웹 주소에만 붙습니다. 빼고 쓰십시오
typed-address-not-a-host = "{ $host }"는 호스트 이름도 IP 주소도 아닙니다
typed-address-not-an-onion = { $host }는 어니언 주소가 아닙니다. 어니언 주소는 .onion 앞에 글자와
    숫자 { $letters }개가 옵니다

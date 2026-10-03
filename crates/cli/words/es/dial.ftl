### Reaching another node, whichever way its address says to.

dial-would-show = este nodo mantiene oculta su dirección, así que no abrirá una
    conexión a { $address }, que la mostraría

dial-no-answer = sin respuesta tras { $seconds } s

dial-waking = alguien a quien vale la pena llegar está en una dirección oculta y
    Tor no está activo. El primer arranque tarda de segundos a minutos,
    y no se pregunta nada a nadie hasta que termine.
    .keyword = arranca

dial-unwoken = Tor no arrancó: { $why }
    Las direcciones ocultas se saltan en esta época. Los nodos que hay
    detrás no han dejado de responder: nada llegó a preguntarles.
    .keyword = dormido

dial-connecting = conectando con { $address }
dial-without-tor = este cliente se compiló sin Tor, así que no puede llegar a { $address }

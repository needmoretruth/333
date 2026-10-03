### `333 status`: what this node's own record says about this node.

status-yourself-on-no-roll = No estás en el padrón de nadie. Nadie te ha entregado el archivo,
    así que aún no hay nada que nadie pueda atestiguar. `333 join` es
    todo lo que hace falta, y necesita la invitación de alguien que ya
    lo tenga.
    { $place } dice qué es esto y dónde está el código. No puede darte
    el archivo.

status-yourself-not-yet-counted = Recibiste el archivo en la época { $joined }, y cuentas desde la época
    { $counted_from }: faltan { $to_go }.
    Responde a todo lo que se te pregunte hasta entonces. Nada de eso se
    guarda, y todo se observa.

status-yourself-says-nothing = Tu registro no dice nada de { $missing } de esas { $window } épocas.
    Nada aquí lo convierte en una ausencia: un registro solo puede decir
    lo que un nodo estuvo ahí para escribir. Por eso esta proporción no
    es con la que te leen los demás: lo que leen es lo que les dijeron
    de ti quienes salieron sorteados para preguntar.

status-yourself-only-your-word = Nadie ha firmado aún nada sobre ti, así que por ahora de todo esto
    solo está tu propia palabra.

status-yourself-checkable = La parte que cualquier otro puede comprobar es lo que otros firmaron
    sobre ti. Hay { $witnessed } aquí, guardadas aunque sus épocas ya pasaron.

status-yourself-record = Tu registro es lo que anotaste de ti mismo, en orden, firmado sobre
    la marcha. Cada respuesta que diste nombró dónde estaba en ese
    momento, así que su longitud y su orden ya no puedes cambiarlos. Lo
    que concluye sigue siendo tu propia palabra. { $checkable }

    Esas firmas valen para siempre. Lo que no se puede recuperar es si
    quienes las hicieron eran de los nuestros en ese momento: para
    entonces ya no están, y no queda nadie a quien preguntar. Un registro
    de cien años prueba que alguien con esa clave estuvo detrás de ti, y
    ahí se acaba. 333 no arregla esto ni finge hacerlo. El arreglo es un
    registro de quién fue fiel, que alguien guarde para siempre, y eso es
    lo único que esta red no construirá.

status-yourself-never-asked = Nada de las últimas { $window } épocas se te preguntó nunca. Nadie
    salió sorteado para preguntar, así que no hay nada en que fallar. Ni
    te mantienes ni decaes; simplemente aún no formas parte de las
    cuentas de nadie.

status-yourself-counted = Por tu propio registro, cuentas.

status-yourself-not-counted = Por tu propio registro, no cuentas. Dos de cada tres es todo lo que
    se pide.
    Aquí no se cumple ninguna condena. La ventana se mueve cada época, y
    cada una que respondes empuja una ausencia más antigua fuera del
    borde. Tu cadena aún guarda cada hora que faltaste. La cuenta no
    vuelve a buscarlas.

status-yourself-standing = Presente en { $present } de las { $counted } épocas que cubre tu registro: { $share }. { $verdict }
    La ventana son las últimas { $window } épocas y nada anterior existe.
    Diez años de ella se leerían exactamente igual y comprarían
    exactamente lo mismo; un año fuera y una hora fuera también se leen
    exactamente igual, y cuestan exactamente igual de poco.

status-yourself-given-by-nobody = Tienes el archivo y nadie te lo entregó, así que no estás en el
    padrón de nadie. Nadie sale a preguntar a un nodo sin padrón, y nadie
    cuenta lo que dice, por eso se rechaza `333 say`. Lo que puedes hacer
    es pasar el archivo: a quien se lo entregues se le admite, firmando
    los dos, y cuenta desde entonces.

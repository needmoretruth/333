### `333 run`: whether anybody outside can actually get in.

serve-reach-unanswered-at-the-end = el router no había respondido cuando el nodo se detuvo. Lo que
    hubiera aceptado vence por sí solo dentro de { $time }.
    .keyword = cerrado

serve-reach-shut-behind-another = el router dice que esta casa está en { $seen }, que no es una
    dirección de internet abierta: otro router, o la dirección compartida
    del proveedor, está entre ella y todos los demás, y nada de aquí
    puede pedirle algo a ese. `333 run --tor` no necesita ningún cambio
    en el router.
    .keyword = cerrado

serve-reach-open = el puerto { $port } llega a esta máquina desde fuera. Este nodo llamó a
    { $outside } y se respondió a sí mismo, así que esa dirección se
    puede dar a cualquiera.
    .keyword = abierto

serve-reach-invite = { $invitation }
    .keyword = invita

serve-reach-shut-somebody-else = algo respondió en { $outside } y no era este nodo. Ese puerto de
    tu dirección es de otra cosa, así que una invitación con él mandaría
    a la gente a la máquina equivocada.
    .keyword = cerrado

serve-reach-shut-unfinished = algo en { $outside } aceptó la conexión y no terminó un latido:
    { $why }. Una invitación con ella no sirve para repartir.
    .keyword = cerrado

serve-reach-shut-nothing = nada respondió en { $outside }, así que, por lo que puede ver el
    mundo de fuera, este nodo no escucha. O al router de delante nunca
    se le dijo que enviara aquí el puerto { $port }, o no deja que una
    máquina de dentro marque su propia dirección exterior.
    `333 run --tor` no necesita cambios en el router y funciona en
    cualquier red.
    .keyword = cerrado

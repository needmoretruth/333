### `333 run`: orders from other terminals on this machine.

serve-told-not-here = sur ce système, ils atteignent ce nœud par son écran et nulle part
    ailleurs. Un second 333 lancé à côté est refusé.
    .keyword = ordres

serve-told-cannot = ne peuvent pas être reçus d’autres terminaux : { $why }
    .keyword = ordres

serve-told-not-private = le socket n’a pas pu être rendu privé

serve-told-taking = depuis tout terminal de cette machine, avec les mots de l’écran :
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    Ce nœud les exécute et y répond.
    .keyword = ordres

serve-told-taking-light = depuis tout terminal de cette machine :
    `333 say 7`, `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`.
    Ce nœud les exécute et y répond.
    .keyword = ordres

serve-told-old-socket = un ancien socket gêne et reste là : { $why }
    .keyword = ordres

serve-told-not-a-socket = { $path } existe et n’est pas un socket : on n’y touche pas, et rien
    ne peut être confié à ce nœud depuis un autre terminal tant qu’il
    n’est pas déplacé.
    .keyword = ordres

serve-told-no-longer = ne sont plus reçus d’autres terminaux : { $why }
    .keyword = ordres

serve-told-not-the-owner = seul le propriétaire du répertoire de ce nœud peut lui parler
    .keyword = refusé

serve-told-too-long = c’est plus long que tout ordre. Le plus long fait { $bytes } octets.
    .keyword = non lu

serve-told-other-version = ce nœud parle { $ours } et a été interrogé en { $theirs }. Le 333 qui
    a demandé est une autre version que celle qui tourne ; lancez celle-là.
    .keyword = refusé

serve-told-unread = { $why }
    .keyword = non lu

serve-told-asked = { $order }, depuis un autre terminal
    .keyword = demandé

serve-told-unheard = plus rien n’exécute les ordres
    .keyword = inouï

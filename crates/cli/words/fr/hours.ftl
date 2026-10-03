### `333 run`: keeping the hours, at every epoch boundary.

hours-failed-marking = inscription de cette époque : { $why }
    .keyword = échec

hours-failed-sources = note de l’origine des adresses : { $why }
    .keyword = échec

hours-not-leaving = cette adresse n’est pas laissée sur { $place }. Elle atteint ce nœud
    d’ici et de nulle part ailleurs, et un inconnu qui la composerait
    atteindrait quelque chose à lui.
    .keyword = agora

hours-sealing = scellement de l’adresse de ce nœud

hours-failed-keeping-address = conservation de l’adresse de ce nœud : { $why }
    .keyword = échec

hours-failed-saying-where = annonce de l’endroit où est ce nœud : { $why }
    .keyword = échec

hours-forgot = { $epochs ->
        [one] { $epochs } époque
       *[other] { $epochs } époques
    }. rien de ce qu’on en dirait maintenant ne changerait un verdict.
    .keyword = oublié

hours-failed-forgetting = oubli des anciennes déclarations : { $why }
    .keyword = échec

hours-minutes = { $minutes ->
        [one] { $minutes } minute
       *[other] { $minutes } minutes
    }

hours-minutes-and-seconds = { $minutes } min { $seconds } s
hours-hours-and-minutes = { $hours } h { $minutes } min

hours-epochs-answered-for = { $epochs ->
        [one] { $epochs } époque répondue
       *[other] { $epochs } époques répondues
    }

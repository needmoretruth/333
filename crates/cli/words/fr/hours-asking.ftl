### `333 run`: trading with the other nodes once an epoch, and asking whoever was drawn.

hours-asking-failed-gathering = rassemblement de ce que ce nœud pourrait transmettre : { $why }
    .keyword = échec

hours-asking-quiet = { $address } : { $why }
    .keyword = muet

hours-asking-unended = l’échange n’a pas fini dans les { $time } de cette époque, et le
    reste des heures ne l’attendra pas
    .keyword = inachevé

hours-asking-reading-address = lecture de l’adresse d’un pair
hours-asking-exchanging-heartbeats = échange de battements
hours-asking-trading = échange de déclarations
hours-asking-putting = question posée
hours-asking-sealing-presenting = scellement de ce que ce nœud est venu dire
hours-asking-saying-what-for = ce nœud dit pourquoi il est venu
hours-asking-sealing-silence = scellement de ce qui n’a pas eu lieu

hours-asking-no-answer = pas de réponse
hours-asking-did-not-answer = { $address } n’a pas répondu
hours-asking-did-not-finish = { $address } n’a pas fini le battement
hours-asking-neither = { $address } n’a ni demandé ni raccroché
hours-asking-within = { $what } dans les { $seconds } s que permet la fenêtre
hours-asking-nothing-within = rien dans les { $seconds } s que permet la fenêtre

hours-asking-going = personne dehors ne peut ouvrir de connexion vers ce nœud, il va donc
    vers les { $drawn } d’entre nous tirés au sort pour l’interroger cette
    époque. Le tirage découle de l’époque et des clés : ce nœud sait qui
    ils sont sans qu’on le lui dise.
    .keyword = va

hours-asking-unknown-drawn-by = tiré au sort pour être interrogé par l’un de nous dont nul n’a dit
    où il est
    .keyword = inconnu

hours-asking-unasked = époque { $epoch } : { $why }
    .keyword = sans q.

hours-asking-drawn = époque { $epoch } : interroger { $asked } d’entre nous. Personne ne l’a
    choisi : les noms découlent de l’époque et des clés, à l’identique sur
    chaque machine.
    .keyword = tirage

hours-asking-unknown-drawn-to-ask = tiré au sort pour interroger l’un de nous dont nul n’a dit où il est
    .keyword = inconnu

hours-asking-unheard = époque { $epoch } : { $why }
    .keyword = inouï

hours-asking-witness = époque { $epoch }, répondue par { $prover }
    .keyword = témoin

hours-asking-silence = époque { $epoch } : { $why }
    .keyword = silence

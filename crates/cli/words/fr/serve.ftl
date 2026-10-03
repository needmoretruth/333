### `333 run`.

serve-nothing-listening = rien n’écouterait : --no-direct demande --tor

serve-name = { $name }
    .keyword = nom

serve-waiting-for-the-file = ce nœud n’a pas reçu le fichier : rien ne compte encore pour lui,
    et il n’y a encore rien à témoigner. Il ne peut pas le créer. Le
    fichier ne vient que de quelqu’un qui le détient déjà, et vous signez
    tous les deux la remise. Demandez une invitation, puis
    `333 join 333:leur.adresse:3333`. Répondre en attendant ne coûte
    rien, et c’est ainsi qu’on vous trouve.
    .keyword = attente

serve-hand = une invitation nomme un lieu, pas une personne. Celui qui y répond
    prouve qui il est en détenant sa clé.
    .keyword = foi

serve-invite = { $invitation }
    .keyword = invite

serve-answer = { $bound }
    .keyword = répond

serve-nearby = on dit sur ce réseau que quelque chose ici parle 333, et on écoute
    les autres. Pas le nom de ce nœud : ce qui sort est ce qu’un scan de
    ports du même réseau trouverait. --no-mdns l’en tient à l’écart.
    .keyword = proche

serve-nearby-failed = impossible de dire sur ce réseau que ce nœud est ici : { $why }
    .keyword = proche

serve-meet = { $place } est l’endroit où ce nœud cherche ceux que personne ne lui a
    présentés. Tout ce qui s’y lit est signé par qui l’a dit, et rien de
    ce qui s’y trouve n’est cru. --no-meet l’en tient à l’écart.
    .keyword = agora

serve-listener-stopped = un écouteur s’est arrêté de façon inattendue

serve-farewell = fini à l’époque { $epoch }. Ceux tirés au sort pour vous interroger
    pendant que ceci ne tourne pas signent qu’ils ont demandé et n’ont
    rien entendu, et c’est ce que lit votre fenêtre. Elle fait { $window }
    époques, et elle avance.
    .keyword = nœud

serve-farewell-on-no-roll = fini à l’époque { $epoch }. Vous n’êtes sur le rôle de personne :
    personne ne sort vous interroger, et rien n’est signé sur vous tant
    que ceci ne tourne pas.
    .keyword = nœud

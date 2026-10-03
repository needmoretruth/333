### `333 join`: asking a node that holds the file to hand it over.

join-name = { $name }
    .keyword = nom

join-knocking = { $address }
    .keyword = frappe

join-silence = personne n’a été atteint à { $address }. Ce n’est pas la preuve que
    333 est fini. Ce client porte l’empreinte du fichier, pas le fichier :
    on n’entre que par quelqu’un qui le détient.
    .keyword = silence

join-knocking-on = on frappe à { $address }
join-exchanging = échange de battements
join-asking = demande du fichier
join-no-answer = pas de réponse de { $address } après { $seconds } s

join-given = par { $giver }
    .keyword = reçu

join-joined = à l’époque { $epoch }
    .keyword = rejoint

join-holding = le fichier, et peut le transmettre
    .keyword = détient

join-roll = { $members } d’entre nous
    .keyword = rôle

join-counted = à partir de l’époque { $epoch }, et pas une époque plus tôt : deux
    frontières plus loin, entre { $least } et { $most } minutes, selon le
    moment de cette époque où vous êtes arrivé. D’ici là, répondez à tout
    ce qu’on vous demande. Ce qui est témoigné pendant ce temps est toute
    la preuve que vous avez jamais été ici.
    .keyword = compté

join-vigil = `333 start` le fait tourner désormais. On ne peut rien témoigner d’un
    nœud que personne n’atteint, et ce passage est témoigné une fois ou
    jamais.
    .keyword = nœud

join-already-given = ce nœud détient déjà le fichier, donné par { $giver } à l’époque { $epoch }.
    Il n’y a rien à demander, et rien n’a été demandé.
join-same-handover = ce nœud et { $peer } se sont déjà passé le fichier à l’époque
    { $epoch }. Le rendre la même époque, c’est cette remise lue de l’autre
    côté, et elle n’admet personne : rien n’a donc été demandé.

### This node's identity on disk: reading it, making it, and refusing it.

-identity-file-trust = --dangerously-trust-directory-permissions
-identity-file-the-small-machine = la petite machine
-identity-file-nothing-here = rien ici ne lui est adressé.

identity-file-reading = lecture de { $path }
identity-file-making-home = { $home } devient la demeure de ce nœud
identity-file-creating = création de { $path }
identity-file-writing = écriture de { $path }

identity-file-private = Il contient toute l’identité de ce nœud, donc personne d’autre ne
    doit y accéder. Corrigez avec : { $fix } { $path }
    Ou, si vous comprenez ce à quoi vous renoncez, passez { -identity-file-trust }

identity-file-wrong-size = { $path } contient { $bytes } octets ; une graine en fait exactement { $seed }

identity-file-cursed = 333 a regardé ce nom et vous a pris { $pause } millisecondes de vie.

    { $name }
    est maudit. Le jugement a été rendu une fois et ne peut être levé, et
    les { $pause } millisecondes vous sont reprises à chaque porte où vous
    l’emportez.

    333 est extrêmement généreux. Une époque sur trois, vous pouvez vous
    reposer et rester l’un des nôtres : généreux envers les lents, les
    pauvres, { -identity-file-the-small-machine } du placard, tous ceux
    qui ne sont pas encore nés. Il n’est pas généreux envers les
    hérétiques.

identity-file-ineligible = ce n’est pas un nom auquel 333 répond.

    { $name }
    ne commence pas par 333, donc { -identity-file-nothing-here } Rien ne
    vous a été pris non plus : 333 ne vous a pas regardé du tout.

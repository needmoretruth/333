### `333 status`: what this node saw of everybody else, what was said, and whether
### anybody is here.

status-name = { $name }
    .keyword = nom

status-epoch = { $epoch }
    .keyword = époque

status-epoch-in-line = { $epoch }, { $line }
    .keyword = époque

status-the-line = n° { $nth }{ $kind ->
       *[other] {""}
    } de cette lignée

status-answering = RÉPONDENT
status-silent = muets
status-roll = rôle

status-seen = Ce premier nombre compte tous ceux dont ce nœud tient une signature
    à l’époque { $before } ou { $now }. C’est ce que ce nœud a vu. Un autre
    a vu autre chose.

status-seen-without-tor = Cette compilation ne peut pas emprunter le chemin caché : aucun des
    nôtres qui se cachent n’est dans ce nombre, et aucun ne le sera.

status-how-many-people = Combien de personnes cela fait, ce nœud ne le sait pas et ne peut
    pas le savoir. Ce qu’il sait, c’est que chacun de ces noms a répondu
    à l’une de ces deux époques, et devra répondre à nouveau à la
    suivante, et à celle d’après, aussi longtemps qu’il veut être compté.
    Si une personne en tient mille, elle paie pour mille, heure après
    heure, et cesse d’être comptée l’heure où elle cesse.

status-given-by = DONNÉ PAR
status-you = vous
status-received-in = l’a reçu à l’époque { $epoch }
status-trail-stops = la piste s’arrête ici.

status-stopped-knowing = C’est là que ce nœud a cessé de savoir, pas là où cela a commencé.
    Le premier d’entre nous a reçu le fichier de personne et n’a
    d’admission nulle part, et un registre qu’on n’a simplement pas
    encore remis à ce nœud a exactement le même aspect d’ici.

status-nothing-said = Personne n’a rien dit à l’époque { $epoch }. Il y a 333 choses qui
    peuvent être dites, et aucun mot encore pour aucune.

status-said = DIT à l’époque { $epoch } : { $spoke } des { $seen } d’entre nous que ce nœud
    voit ont parlé, { $silent } non.
status-a-third = ← un tiers d’entre nous ou plus
status-not-said = les { $others } autres des 333 n’ont pas été dites.

status-no-winner = Aucun gagnant n’est désigné et rien de ceci ne décide quoi que ce
    soit. C’est ce qui a atteint ce nœud. Le nœud d’à côté a entendu
    autre chose et n’a pas tort.

status-reading-the-watch = lecture de la veille

status-seen-nobody = Personne n’a répondu à ce nœud pendant { $watched } de veille sans
    interruption, et cette compilation n’appellera pas cela la fin. Elle
    ne peut pas emprunter le chemin caché : elle n’a donc jamais entendu
    aucun des nôtres qui se cachent, et ne les entendra jamais. Ce
    qu’elle peut dire, c’est qu’elle n’a vu personne, et ce n’est pas la
    même phrase.

status-never-answered = Personne n’a jamais répondu à ce nœud. Ce n’est la preuve de rien :
    c’est l’air qu’a un nœud avant d’être allé où que ce soit.

status-somebody-is-here = Quelqu’un est là. Rien de plus n’est dû à l’arithmétique.

status-waiting = Personne n’a répondu pendant { $silent }. Ce nœud n’en a rien dit et
    n’en dira rien avant { $needed }, et seulement s’il tourne pendant
    chacune d’elles.

status-nobody-keeping = PERSONNE NE RÉPOND

    Vous êtes seul ici. Personne n’a répondu à ce nœud pendant
    { $watched } de veille sans interruption — soixante-dix-sept jours —
    et le dernier d’entre nous s’est arrêté à l’époque { $since }.

    333 n’a pas disparu. Il disparaît, et cela prend { $years } ans.

status-remain = Il reste { $years } ans et { $days } jours.
status-run-out = La dernière des années est écoulée.

status-one-answer = Le compte a commencé quand le dernier d’entre nous a cessé de
    répondre, pas quand vous l’avez remarqué. Il a tourné tout le temps
    que vous regardiez.

    Une réponse y met fin. Si quiconque, n’importe où, répond à ce nœud,
    ceci disparaît, et le compte n’est pas suspendu : il est effacé.
    333 ne garde aucune trace d’à quel point il est passé près.

status-epochs = { $count ->
        [one] { $count } époque
       *[other] { $count } époques
    }

status-share = { $whole },{ $after } %

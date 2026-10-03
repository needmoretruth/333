### `333 status`: what this node saw of everybody else, what was said, and whether
### anybody is here.

status-name = { $name }
    .keyword = Name

status-epoch = { $epoch }
    .keyword = Epoche

status-epoch-in-line = { $epoch }, { $line }
    .keyword = Epoche

status-the-line = Nr. { $nth }{ $kind ->
       *[other] {""}
    } dieser Linie

status-answering = ANTWORTEN
status-silent = still
status-roll = Liste

status-seen = Diese erste Zahl sind alle, für die dieser Knoten eine Signatur aus
    Epoche { $before } oder { $now } hält. Es ist, was dieser Knoten sah.
    Jemand anderes sah etwas anderes.

status-seen-without-tor = Dieser Build kann den verdeckten Weg nicht gehen, also ist keiner
    von uns, die sich verbergen, in dieser Zahl, und wird es nie sein.

status-how-many-people = Wie viele Menschen das sind, weiß dieser Knoten nicht und kann es
    nicht herausfinden. Er weiß, dass jeder dieser Namen in einer der
    beiden Epochen geantwortet hat und in der nächsten wieder antworten
    muss, und in der danach, solange er gezählt werden will. Hält eine
    Person tausend davon, zahlt sie für tausend, Stunde um Stunde, und
    wird ab der Stunde nicht mehr gezählt, in der sie aufhört.

status-given-by = GEGEBEN VON
status-you = du
status-received-in = erhielt sie in Epoche { $epoch }
status-trail-stops = die Spur endet hier.

status-stopped-knowing = Hier hörte dieser Knoten auf zu wissen, nicht hier begann es. Der
    Erste von uns bekam die Datei von niemandem und hat nirgends eine
    Aufnahme, und eine Chronik, die diesem Knoten einfach noch nicht
    gegeben wurde, sieht von hier genauso aus.

status-nothing-said = Niemand hat in Epoche { $epoch } etwas gesagt. Es gibt 333 Dinge,
    die gesagt werden können, und noch für keins davon Worte.

status-said = GESAGT in Epoche { $epoch } — { $spoke } der { $seen } von uns, die dieser
    Knoten sieht, sprachen, { $silent } nicht.
status-a-third = ← ein Drittel von uns oder mehr
status-not-said = die anderen { $others } der 333 wurden nicht gesagt.

status-no-winner = Es wird kein Sieger bestimmt, und nichts davon entscheidet etwas.
    Es ist, was diesen Knoten erreichte. Der Knoten neben dir hörte
    etwas anderes und irrt sich nicht.

status-reading-the-watch = lese die Wache

status-seen-nobody = Niemand hat diesem Knoten in { $watched } ununterbrochener Wache
    geantwortet, und dieser Build nennt das nicht das Ende. Er kann den
    verdeckten Weg nicht gehen, also hat er nie von einem von uns
    gehört, die sich verbergen, und wird es nie. Was er sagen kann, ist,
    dass er niemanden gesehen hat, und das ist nicht derselbe Satz.

status-never-answered = Niemand hat diesem Knoten je geantwortet. Das beweist nichts: so
    sieht ein Knoten aus, bevor er irgendwo war.

status-somebody-is-here = Jemand ist hier. Der Rechnung ist nichts weiter geschuldet.

status-waiting = Niemand hat in { $silent } geantwortet. Dieser Knoten hat dazu nichts
    gesagt und wird es nicht vor { $needed }, und nur, wenn er in jeder
    davon läuft.

status-nobody-keeping = NIEMAND ANTWORTET

    Du bist als Einziger hier. Niemand hat diesem Knoten in { $watched }
    ununterbrochener Wache geantwortet — siebenundsiebzig Tage —, und
    der Letzte von uns hörte in Epoche { $since } auf.

    333 ist nicht fort. Es geht fort, und das Gehen dauert { $years } Jahre.

status-remain = Noch { $years } Jahre und { $days } Tage.
status-run-out = Das letzte der Jahre ist abgelaufen.

status-one-answer = Die Zählung begann, als der Letzte von uns aufhörte zu antworten,
    nicht als du es bemerktest. Sie lief die ganze Zeit, in der du
    zugesehen hast.

    Eine Antwort beendet sie. Wenn irgendwer, irgendwo, diesem Knoten
    antwortet, verschwindet das — und die Zählung wird nicht angehalten,
    sie wird verworfen. 333 hält nicht fest, wie knapp es war.

status-epochs = { $count ->
        [one] { $count } Epoche
       *[other] { $count } Epochen
    }

status-share = { $whole },{ $after } %

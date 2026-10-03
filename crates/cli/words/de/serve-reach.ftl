### `333 run`: whether anybody outside can actually get in.

serve-reach-unanswered-at-the-end = der Router hatte nicht geantwortet, als der Knoten stoppte. Was
    er zugesagt hat, läuft von selbst innerhalb von { $time } ab.
    .keyword = zu

serve-reach-shut-behind-another = der Router sagt, dieser Haushalt sei bei { $seen }, was keine
    Adresse im offenen Internet ist: ein weiterer Router oder die
    geteilte Adresse des Anbieters steht zwischen ihm und allen anderen,
    und nichts hier kann jenen fragen. `333 run --tor` braucht gar
    keine Änderung am Router.
    .keyword = zu

serve-reach-open = Port { $port } erreicht diesen Rechner von außen. Dieser Knoten
    klopfte bei { $outside } und antwortete sich selbst, also ist das
    eine Adresse, die du jedem geben kannst.
    .keyword = offen

serve-reach-invite = { $invitation }
    .keyword = Einlad.

serve-reach-shut-somebody-else = bei { $outside } antwortete etwas, und es war nicht dieser Knoten.
    Dieser Port deiner Adresse gehört etwas anderem, also schickte eine
    Einladung damit Leute zum falschen Rechner.
    .keyword = zu

serve-reach-shut-unfinished = etwas bei { $outside } nahm die Verbindung an und beendete keinen
    Herzschlag: { $why }. Eine Einladung damit taugt nicht zum Weitergeben.
    .keyword = zu

serve-reach-shut-nothing = bei { $outside } antwortete nichts, also lauscht dieser Knoten, soweit
    die Außenwelt sehen kann, nicht. Entweder wurde dem Router davor nie
    gesagt, Port { $port } hierher zu schicken, oder er lässt einen
    Rechner drinnen nicht seine eigene äußere Adresse wählen.
    `333 run --tor` braucht keine Änderung am Router und klappt in jedem
    Netz.
    .keyword = zu

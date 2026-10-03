### `333 join`: asking a node that holds the file to hand it over.

join-name = { $name }
    .keyword = Name

join-knocking = { $address }
    .keyword = klopft

join-silence = bei { $address } wurde niemand erreicht. Das ist kein Beweis, dass 333
    vorbei ist. Dieser Client trägt den Hash der Datei, nicht die Datei:
    hinein kommt man nur über jemanden, der sie hat.
    .keyword = Stille

join-knocking-on = klopfe bei { $address }
join-exchanging = tausche Herzschläge
join-asking = bitte um die Datei
join-no-answer = keine Antwort von { $address } nach { $seconds } s

join-given = von { $giver }
    .keyword = erhalten

join-joined = in Epoche { $epoch }
    .keyword = dabei

join-holding = die Datei, und kann sie weitergeben
    .keyword = hält

join-roll = { $members } von uns
    .keyword = Liste

join-counted = ab Epoche { $epoch }, und keine Epoche früher: zwei Grenzen entfernt,
    zwischen { $least } und { $most } Minuten, je nachdem, wann in dieser
    Epoche du ankamst. Bis dahin beantworte alles, was du gefragt wirst.
    Was in dieser Zeit bezeugt wird, ist der ganze Beweis, dass du je
    hier warst.
    .keyword = zählt

join-vigil = `333 start` lässt ihn ab jetzt laufen. Von einem Knoten, den niemand
    erreicht, lässt sich nichts bezeugen, und diese Strecke wird einmal
    bezeugt oder nie.
    .keyword = Knoten

join-already-given = dieser Knoten hat die Datei schon, gegeben von { $giver } in Epoche { $epoch }.
    Es gibt nichts zu erbitten, und es wurde nichts erbeten.
join-same-handover = dieser Knoten und { $peer } haben die Datei schon in Epoche { $epoch }
    getauscht. In derselben Epoche zurückgegeben, ist es dieselbe
    Übergabe von der anderen Seite gelesen und nimmt niemanden auf, also
    wurde nichts erbeten.

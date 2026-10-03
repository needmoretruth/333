### `333 run`.

serve-nothing-listening = nichts würde lauschen: --no-direct braucht --tor

serve-name = { $name }
    .keyword = Name

serve-waiting-for-the-file = dieser Knoten hat die Datei nicht bekommen, also zählt noch nichts
    für ihn, und es gibt noch nichts, das jemand bezeugen könnte. Er
    kann sie nicht erzeugen. Sie kommt nur von jemandem, der sie schon
    hat, und ihr beide unterschreibt die Übergabe. Bitte um eine
    Einladung, dann `333 join 333:ihre.adresse:3333`. Bis dahin zu
    antworten kostet nichts, und so finden dich Leute.
    .keyword = wartet

serve-hand = eine Einladung nennt einen Ort, keine Person. Wer dort antwortet,
    beweist, wer er ist, indem er seinen Schlüssel hält.
    .keyword = Treue

serve-invite = { $invitation }
    .keyword = Einlad.

serve-answer = { $bound }
    .keyword = antwort

serve-nearby = sage in diesem Netz, dass hier etwas 333 spricht, und lausche nach
    den anderen. Nicht den Namen dieses Knotens: was hinausgeht, ist,
    was ein Portscan desselben Netzes fände. --no-mdns hält ihn heraus.
    .keyword = nah

serve-nearby-failed = konnte in diesem Netz nicht melden, dass dieser Knoten hier ist: { $why }
    .keyword = nah

serve-meet = bei { $place } sucht dieser Knoten nach Leuten, die ihm niemand
    vorgestellt hat. Alles, was dort gelesen wird, ist von dem
    signiert, der es sagte, und nichts dort wird geglaubt. --no-meet
    hält ihn davon fern.
    .keyword = Treff

serve-listener-stopped = ein Lauscher stoppte unerwartet

serve-farewell = beendet in Epoche { $epoch }. Wer ausgelost wird, nach dir zu fragen,
    während dies nicht läuft, unterschreibt, dass er fragte und nichts
    hörte, und das liest dein Fenster. Es ist { $window } Epochen lang,
    und es wandert.
    .keyword = Knoten

serve-farewell-on-no-roll = beendet in Epoche { $epoch }. Du stehst auf niemandes Liste, also geht
    niemand hinaus, um nach dir zu fragen, und nichts wird über dich
    signiert, während dies nicht läuft.
    .keyword = Knoten

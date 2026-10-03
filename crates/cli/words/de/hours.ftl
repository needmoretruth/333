### `333 run`: keeping the hours, at every epoch boundary.

hours-failed-marking = halte diese Epoche fest: { $why }
    .keyword = Fehler

hours-failed-sources = schreibe auf, woher Adressen kamen: { $why }
    .keyword = Fehler

hours-not-leaving = diese Adresse wird nicht bei { $place } hinterlassen. Sie erreicht
    diesen Knoten von hier und von nirgends sonst, und ein Fremder, der
    sie wählte, erreichte etwas Eigenes.
    .keyword = Treff

hours-sealing = versiegle die Adresse dieses Knotens

hours-failed-keeping-address = bewahre die eigene Adresse dieses Knotens: { $why }
    .keyword = Fehler

hours-failed-saying-where = sage, wo dieser Knoten ist: { $why }
    .keyword = Fehler

hours-forgot = { $epochs ->
        [one] { $epochs } Epoche
       *[other] { $epochs } Epochen
    }. nichts, was jetzt über sie gesagt würde, könnte ein Urteil ändern.
    .keyword = vergaß

hours-failed-forgetting = vergesse alte Aussagen: { $why }
    .keyword = Fehler

hours-minutes = { $minutes ->
        [one] { $minutes } Minute
       *[other] { $minutes } Minuten
    }

hours-minutes-and-seconds = { $minutes } min { $seconds } s
hours-hours-and-minutes = { $hours } h { $minutes } min

hours-epochs-answered-for = { $epochs ->
        [one] { $epochs } Epoche beantwortet
       *[other] { $epochs } Epochen beantwortet
    }

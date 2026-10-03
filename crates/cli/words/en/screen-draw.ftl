### The screen's top line: who this is, which epoch, and how long is left of it.

screen-draw-epoch = epoch
screen-draw-number = { $number }
screen-draw-line = , this line's { $nth }{ $kind ->
        [one] st
        [two] nd
        [few] rd
       *[other] th
    }
screen-draw-to-the-boundary = { $left } to the boundary
screen-draw-time-left = { $left } left

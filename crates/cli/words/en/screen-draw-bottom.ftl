### The screen's two bottom lines: whether anybody is here, and what the keys do.
### Each line has a wide form and a narrow one, for a terminal under 62 columns.
### A line broken here is one line on the screen: the breaks are only for this file.

screen-draw-bottom-never-answered-wide = no one has ever answered this node, which is what a node
    looks like before it has been anywhere
screen-draw-bottom-never-answered = no one has ever answered this node
screen-draw-bottom-alive-wide = somebody is here. nothing further is owed to the arithmetic
screen-draw-bottom-alive = somebody is here
screen-draw-bottom-waiting-wide = nobody has answered for { $silent } of the { $needed } epochs it
    would take to say so
screen-draw-bottom-waiting = nobody for { $silent } of { $needed } epochs
screen-draw-bottom-ended = nobody has answered since epoch { $since }. { $years ->
        [one] { $years } year
       *[other] { $years } years
    } and { $days ->
        [one] { $days } day
       *[other] { $days } days
    } until it is gone
screen-draw-bottom-ended-and-gone = nobody has answered since epoch { $since }, and the last of
    the years has run out

screen-draw-bottom-say-which = say which of the { $signals }?
screen-draw-bottom-say-keys = enter to say it · esc to say nothing

screen-draw-bottom-leave-wide = leave the vigil
screen-draw-bottom-leave = leave
screen-draw-bottom-say-wide = say one of the { $signals }
screen-draw-bottom-say = say
screen-draw-bottom-more-wide = everything else
screen-draw-bottom-more = more
screen-draw-bottom-has-the-file = the file is here
screen-draw-bottom-not-given = this node has not been given the file
screen-draw-bottom-keys-wide = all the keys
screen-draw-bottom-keys = keys

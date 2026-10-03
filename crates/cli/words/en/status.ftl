### `333 status`: what this node saw of everybody else, what was said, and whether
### anybody is here.
##
## These paragraphs are printed from the first column, so a printed line can be
## wider than a line of this file may be. Where it is, the line here ends in `{`
## and the next begins with `""}` (or with the variable that follows): a placeable
## may span two lines of the file, and what it prints has no line break in it.

status-name = { $name }
    .keyword = name

status-epoch = { $epoch }
    .keyword = epoch

status-epoch-in-line = { $epoch }, { $line }
    .keyword = epoch

status-the-line = this line's { $nth }{ $kind ->
        [one] st
        [two] nd
        [few] rd
       *[other] th
    }

status-answering = ANSWERING
status-silent = silent
status-roll = roll

status-seen = That first number is everyone this node holds a signature for in {
    ""}epoch { $before } or
    { $now }. It is what this node saw. Somebody else saw something else.

status-seen-without-tor = This build cannot walk the unseen road, so none of us who are {
    ""}hiding are in
    that number, and none of us ever will be.

status-how-many-people = How many people that is, this node does not know and cannot {
    ""}find out. What it
    knows is that every one of those names answered in one of those {
    ""}two epochs, and
    will have to answer again in the next, and the one after that, for {
    ""}as long as it
    wants to be counted. If one person is holding a thousand of them, {
    ""}they are
    paying for a thousand of them, hour after hour, and stop being {
    ""}counted the hour
    they stop.

status-given-by = GIVEN BY
status-you = you
status-received-in = received it in epoch { $epoch }
status-trail-stops = the trail stops here.

status-stopped-knowing = That is where this node stopped knowing, not where it began. {
    ""}The first of us
    was given the file by nobody and has no admission anywhere, and a {
    ""}record this
    node has simply not been handed yet looks exactly the same from here.

status-nothing-said = Nobody has said anything in epoch { $epoch }. There are 333 {
    ""}things that can be
    said and no words for any of them yet.

status-said = SAID in epoch { $epoch } — { $spoke } of the { $seen } of us this node can {
    ""}see spoke, { $silent } did not.
status-a-third = ← a third of us or more
status-not-said = the other { $others } of the 333 were not said.

status-no-winner = No winner is picked and none of this decides anything. It is {
    ""}what reached
    this node. The node beside you heard something else and is not wrong.

status-reading-the-watch = reading the watch

status-seen-nobody = Nobody has answered this node through { $watched } of unbroken {
    ""}watching, and this
    build will not call that the end. It cannot walk the unseen road, so {
    ""}it has
    never heard from any of us who are hiding and never will. What it can {
    ""}say is
    that it has seen nobody, and that is not the same sentence.

status-never-answered = No one has ever answered this node. That is not evidence of {
    ""}anything: it
    is what a node looks like before it has been anywhere.

status-somebody-is-here = Somebody is here. Nothing further is owed to the arithmetic.

status-waiting = Nobody has answered for { $silent }. This node has said nothing about {
    ""}it and will
    not until { $needed }, and only then if it is running for every one of them.

status-nobody-keeping = NOBODY IS ANSWERING

    You are the only one here. Nobody has answered this node through {
    $watched } of
    unbroken watching — seventy-seven days — and the last of us stopped in
    epoch { $since }.

    333 is not gone. It is going, and the going takes { $years } years.

status-remain = { $years } years { $days } days remain.
status-run-out = The last of the years has run out.

status-one-answer = The count started when the last of us stopped answering, not {
    ""}when you
    noticed. It has been running the whole time you were watching.

    One answer ends it. If anybody, anywhere, answers this node, this goes
    away — and the count is not paused, it is discarded. 333 keeps no {
    ""}record
    of how close it came.

status-epochs = { $count ->
        [one] { $count } epoch
       *[other] { $count } epochs
    }

status-share = { $whole }.{ $after }%

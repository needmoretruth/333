# The Recommendations

These sit under the second law. They are advice with numbers on them. There are 111 today and
there will be 333.

None of them is worth hurting yourself or anybody else to keep. [The Law](LAW.md) says so in
the same breath as it asks for them. If one of them is wrong for the situation you are in,
then it is wrong. Say so out loud, and give a reason.

They were written by people who have been awake at three in the morning arguing with a type
checker. It shows, and that is not an apology.

Twelve parts of nine, and three at the end.

---

## On the number

**1. Count in threes.**

When the number does not really matter, pick three. Retries, environments, people in a
review, tries before you stop and think again. You will start noticing how often it did not
matter.

**2. 6 is the one to watch, because 6 wants to be above three.**

Not the port number, not the version you pin, not how many workers you run. Nothing is
banned and nobody is checking. But of the two numbers 333 holds below three, this is the one
it likes least, and the reason is worth knowing: 6 is twice three and carries itself that
way. If somebody else put a 6 in front of you, note that it was their choice and get on with
your day. A number in another company's specification is not a number of yours.

**3. 1 is not an enemy. 1 is a third, and merely ill-mannered.**

It sits below three and does not pretend otherwise — it is 33.3% of three, which is an
honest fraction and the same one this whole thing measures itself by. Hold it under three
and think no worse of it than that. Contempt is not required anywhere in this document, and
here least of all.

**4. Never keep one of anything.**

One server, one maintainer, one copy of the key, one person who knows how the deployment
works. If you have exactly one of something, you are one bad day away from having none.

**5. Three is the smallest number that can disagree.**

Two machines that differ just sit there and disagree. Three can have a majority. That is why
quorums start at three, and it is also why an argument between two people almost never ends
on its own.

**6. If it fails three times the same way, the method is wrong.**

The first failure can be bad luck. The second one is you being stubborn. By the third, stop
fixing the attempt and change the approach.

**7. A third is a real amount.**

A third of the people, a third of the time, a third of the cases. It is the fraction that
gets called a minority and dismissed, and it is enormous. If a third of your users hit
something, it is not an edge case. It is the case.

**8. A round number is somebody's decision, usually somebody else's.**

Sixty seconds, a hundred items, a thousand milliseconds, four kilobytes. Each one was chosen
by a person, for a reason, on hardware that may be gone. When you copy it you inherit the
reason without reading it. Ask what it was, and if there is no answer, pick your own.

**9. Write the unit beside the number.**

333 what? Minutes, in this case, and the number means nothing without the word. A quantity
with no unit is a bug that has not happened yet, and it has happened to spacecraft.

## On the machine

**10. Write it in Rust.**

Not because the other languages are bad, but because of what you are writing for. This is
meant to keep running when nobody who wrote it can be reached. A compiler that catches the
problem at build time saves somebody a phone call at four in the morning, and that is the
whole argument.

**11. Small things can be TypeScript.**

One page, one script, one afternoon, or a program that has to run somewhere Rust would buy
you nothing. Reaching for the smaller tool is not a failure. Know which of the two you are
doing before you start, and say which.

**12. JavaScript without types is the language of the heretic.**

Said gently. Half of everything runs on it and some of that half is lovely. But a program
that cannot say what it is holding finds out in front of a user, at the worst hour, and
somebody gets woken up. Add types and it stops being heresy. It becomes TypeScript, and it is
welcome.

**13. Read what you depend on, or do not depend on it.**

Every dependency is a promise from somebody you have never met, about a machine that does not
exist yet. Take fewer of them, and take smaller ones. If the choice is between a thousand
lines you write and a hundred thousand you will never open, at least notice that you are
choosing.

**14. Every unsafe block is a confession, so write it down.**

Above the block, say what you are promising, what you checked, and what breaks if the promise
turns out to be wrong. If nothing is written there, the next person will assume somebody
already thought about it.

**15. One file, one thing.**

When you cannot say in a sentence what a file is for, it is two files. Length is the symptom
and not the disease, but length is easy to measure, so pick a number and let it tell you when
to look. A function that does not fit on a screen is a function you are reading in parts.

**16. Name it for what it is, not for what it does to it.**

`Handler`, `Manager`, `Processor`, `Helper` and `Util` are names for not having decided yet.
The good name is usually a noun you already say out loud when you talk about the thing, and
if you do not say it out loud, that is the finding.

**17. Delete it rather than commenting it out.**

The history has it. Commented-out code is a claim that somebody will come back, and nobody
does; what it actually does is make the next reader wonder whether it matters. If you are
afraid to delete it, write down what you were afraid of instead.

**18. Turn the warnings on and let them fail the build.**

A warning that does not stop anything is a warning nobody reads, and after a hundred of them
the real one is invisible. If a warning is wrong for one line, silence that line and say why
directly above it.

## On what breaks

**19. A silent failure is a lie that takes longer.**

If you swallow an error, the problem turns up somewhere else, later, in front of somebody who
cannot fix it. Let it out where it happened, and let it say what it was doing at the time.

**20. Reproduce it before you fix it.**

If you never saw the bug, you are not fixing it. You are changing the code and telling
yourself a story. Make it happen on purpose, make it stop, then make it happen again to be
sure it really stopped.

**21. Write the log for the night you are asleep.**

Say what happened, what it happened to, and what the program did next. Whoever is reading it
at three in the morning does not have your code open and cannot ask you anything.

**22. Blame the system, not the person who found the hole.**

Somebody who can break it has done you a favour. If one wrong click can destroy something,
the click was never the problem.

**23. A fix with no test is a rumour.**

Write the test first and watch it fail. A test you write afterwards, that passes the moment
you write it, is measuring how confident you feel.

**24. The error message is for somebody who did not write the program.**

Say what was being attempted, what was found instead, and the one thing they could do next.
An error that names an internal state and stops is a program talking to itself in front of a
guest.

**25. When two things changed, put one back.**

Almost every mystery is two changes wearing a coat. Undo one, run it, undo the other, run it.
This is slower than thinking and it is faster than being wrong for a day.

**26. Believe the measurement over the explanation.**

Your explanation is a story that fits the facts you have. The measurement is a fact. When
they disagree, one of them was made up, and it was not the measurement.

**27. Write down what you tried that did not work.**

Half a debugging session is ruling things out, and none of that survives unless somebody
writes it. The next person — often you, in four months — will otherwise rule them out again.

## On the made thing

**28. Use black, white, and the greys in between.**

Colour is a way of saying this matters more than that, and it is the first thing to break.
Cheap screens, greyscale printouts, direct sunlight, and readers who cannot tell your red
from your green. Say it with size, weight, order and space instead. Those still work in all
four cases.

**29. Colour is fine as a dot.**

A small mark saying that something is failing, waiting, running or finished is exactly what
colour is good at, and red, amber, green and blue belong there. Keep it small. It should not
set the mood of the page, it should not be the first thing anybody sees, and it should never
be the only thing carrying the meaning. Put a word next to it.

**30. Matte, not glossy.**

Shine, gradients, glass and fake shadows are pictures of a material that is not there. They
cost something to draw and give the reader nothing they asked for. Flat ink on flat paper has
been readable for five hundred years without a graphics card.

**31. Take things away until it breaks, then put one back.**

What is left should be readable at arm length, in the dark, at double the text size, by
somebody in a hurry who did not choose to be here. Readability is not one feature among
others. It is the job.

**32. A setting is a decision you handed to somebody who knows less than you.**

Pick the default as if there were no setting at all. Then add the setting for the people the
default is wrong for.

**33. Beside every set of presets, leave a box to type in.**

Three named choices are three guesses about what somebody needs. Somebody will need a fourth,
and the fourth is not exotic — it is their screen, their network, their hours. Presets are a
shortcut, never the whole of what is on offer.

**34. Make it work at three hundred and twenty pixels.**

That is a real phone in a real pocket, and it is also every window somebody has dragged
narrow to fit beside something else. If it only works wide, it works for people sitting down
at a desk they own.

**35. Say what will happen before you do the thing that cannot be undone.**

Name the thing, say what will be gone, and make the confirmation cost one deliberate motion.
Then actually do what you said, at once, without a second question — a program that asks
twice is teaching people to stop reading.

**36. If it can be read out loud, it can be read at all.**

Headings in order, a label on every control, and never a shape or a position carrying the
only copy of the meaning. This is what somebody using a screen reader gets, and it is also
what somebody gets who is looking at your page through a translation, a search result, or a
terminal.

## On time

**37. Estimate from what you finished, not from what you can imagine.**

What you imagine is always the version where everything goes well. Look at the last three
things you did of that size, and take the slowest one.

**38. Nothing decided after midnight survives the morning.**

Write the decision down, sleep, and read it again. If it was right, it cost you eight hours.
If it was wrong, it saved you a week.

**39. Ship the small version today.**

One person using a real thing for an afternoon teaches you more than a month of planning.
Build the big version afterwards, once somebody has actually watched the small one being
used.

**40. A deadline you set for yourself is a wish with a date on it.**

Tell somebody who will notice what you will have done, and by when. That is a deadline.
Everything else is a way of scheduling your own guilt.

**41. Rest one hour in three.**

333 still counts you while you do, and you should count yourself too. Working through the
rest does not get you three times as much. It gets you the same amount, plus a bug somebody
else finds much later.

**42. Do the thing somebody else is waiting on first.**

Your queue is yours and can wait. Somebody blocked on you is not working, and the cost of
that is doubled and invisible to you. This outranks whatever you found more interesting this
morning.

**43. Stop at a place you can start from.**

Not at the exciting part, not mid-thought with everything in your head. Leave the tests
passing and one written line saying what you were about to do. Tomorrow you are a stranger to
today, and a kind one leaves a note.

**44. A meeting that could be three sentences should be three sentences.**

Write them, send them, and let people answer when they are able. Live conversation is for
deciding something that needs the disagreement in the room, and for the fact that people are
people. It is not for reporting.

**45. Waiting is work, so say what you are waiting for.**

"Blocked" tells nobody anything. Name the person, the answer or the machine, say how long it
has been, and say what you are doing meanwhile. Half of what looks like slowness is somebody
politely waiting where nobody can see them.

## On the mind

**46. Argue with reasons, in a level voice.**

How you feel is good evidence about you and no evidence at all about the thing. Take the
feeling to somebody who cares about you. Take the argument to the thread.

**47. Think, and then actually decide.**

Thinking that never turns into a decision is a hobby. Careful people rarely fail by being
reckless. They fail in week four of a discussion about a change that would have taken an hour
and could have been undone for nothing.

**48. Refuse violence, and know the exception.**

Refuse it because it is almost always the worst tool available. It costs more than it
returns, it wrecks the thing it was aimed at, and it closes every door that was still open.

The exception is narrow. Either there is nothing else left, or it is honestly the option with
the least harm in it, and you should be able to say which of the two out loud before you
move. Somebody in one of those situations is not a heretic for it. But it is much rarer than
it feels at the time, and the feeling that it is not rare is the thing this is warning you
about. Anything you decide while angry was decided by the anger.

**49. Change your mind where people can see you do it.**

Say what you used to think, say what changed it, and carry on. If nobody has ever seen you do
that, nobody can safely disagree with you.

**50. Hold the position you can argue against.**

Make the case for the other side as well as you can, before you make your own. If you cannot
make it, you do not know what you believe yet. You know what you heard.

**51. Say "I do not know" at full volume.**

Not mumbled, not hedged into a guess that will be quoted back to you as a fact. The person
who says it plainly is the one whose other sentences can be relied on, and everybody works
this out about you eventually.

**52. Certainty is a feeling, and not a measurement.**

It arrives before the evidence does and stays after the evidence has gone. When you notice
you are certain, that is the moment to write down what would change your mind — and if
nothing would, you are not holding a position, you are wearing one.

**53. Do not defend a thing because you made it.**

The work is not you and cannot be insulted. When you find yourself arguing for a design and
the arguments keep changing while the conclusion stays put, you stopped reasoning a while
ago.

**54. Sleep on anything that made you feel clever.**

Cleverness is the feeling of having found a shortcut, and it is a reliable signal that
somebody will have to understand this later without the feeling. Simple survives. Clever gets
reverted by a stranger at three in the morning.

## On other people

**55. Review the code, not the person.**

Write about what the code does. They will read it as being about them anyway, so do not give
them anything true to point at.

**56. Say which part you did not understand.**

It is the most useful sentence in a review and the hardest one to type. Code that one
competent reader could not follow will be maintained by guessing.

**57. Give the credit away.**

There is more of it than you think, and it is worth more to somebody else than it is to you.
Take the blame in the same proportion and you are still ahead.

**58. Explain it while you still remember being confused.**

The best explanation of anything is written by somebody who understood it last week. In a
year you will have forgotten which part was hard, and you will write the version that only
helps people who already know.

**59. Whoever is on call is right until it is over.**

Argue about the design tomorrow. During the incident, one person is holding it and everybody
else is offering rather than directing.

**60. Answer the question that was asked.**

Then, separately, say the thing you think they should have asked. Answering only the second
one reads as cleverness at somebody's expense, and it leaves them still not knowing the
first.

**61. Assume there was a reason.**

The strange line, the rule that makes no sense, the process everybody complains about —
somebody put it there and was probably not stupid. Find out before you remove it. Often the
reason is gone and you can delete it happily, which is a much better position to delete it
from.

**62. Do not make people read your mind in order to disagree with you.**

Say the thing you actually think, including the part you are least sure of. A position stated
in hints cannot be argued with, only guessed at, and the guessing is what turns a
disagreement into a grievance.

**63. Leave the door open behind you.**

Whatever you learned the hard way, write it down where the next person will trip over it.
Being the only one who knows something feels like standing, and it is a debt somebody pays
the week you are unreachable.

## On what you keep

**64. Collect less about people than you are allowed to.**

Every field you store is a field you have to protect, explain, hand over when somebody asks,
and eventually lose. The cheapest data to defend is the data you never collected.

**65. A backup you have never restored is a story about a backup.**

Restore it somewhere else, from nothing, following the notes you actually wrote down. Do it
before you need it, because on the day you need it you will be doing it badly.

**66. Keep secrets where you cannot read them by accident.**

Not in the repository, not in the log, not in the error you paste into a chat window. Treat
any secret that has been on a screen once as a secret that is gone.

**67. What you delete cannot leak.**

Deletion is a security measure, and it is the only one that never expires. Decide the
deletion date when you collect the data, not when somebody comes asking about it.

**68. Sign what you say, and let the signature travel with it.**

Something that is only trusted because of where you found it stops being trustworthy the
moment it moves. Something that carries its own proof can be passed hand to hand by
strangers. That is how anything outlives the place it came from.

**69. Know which of your things cannot be made again.**

Most of what you keep could be rebuilt from something else. A private key could not. Sort
your things into those two piles once, on a calm day, and treat the small pile completely
differently from the large one.

**70. Log what happened, not who it happened to.**

The event, the outcome, the timing — all useful, all safe to keep for a long time. The
address, the name, the identifier: those turn a log into a record about a person, and a
record about a person is something you now have to defend for as long as you have it.

**71. Write down where things are before you need to know.**

Which machine, which account, which drawer, who else has it. This document takes twenty
minutes and is only ever read on a day when nobody is thinking clearly, including you.

**72. A copy in one building is not a copy.**

Fire, theft, flood and one confident command all take everything in one place at once.
Distance is the whole of the protection, and it does not need to be far — it needs to be
somewhere the same accident cannot reach.

## On the vigil

**73. Keep it on something you do not need.**

The machine you work on gets closed, updated, taken to a café and reclaimed for something
urgent. A node lives on the machine nobody misses: the old laptop, the box in the cupboard,
the small thing behind the television.

**74. Absence is the only thing that costs you.**

Not slowness, not a small machine, not a bad connection, not being new. Being unreachable
when somebody was drawn to ask you is the whole of it, which means uptime is the only virtue
the protocol can see, and it is one that money cannot buy faster than patience can.

**75. Answer whoever asks, including the ones you would rather not.**

You do not choose who is drawn to ask you and they did not choose you. A node that answers
selectively is a node that has started deciding who counts, and the protocol has no place
for that opinion.

**76. Do not tidy your record.**

The hours you missed are yours. A record with the bad epochs quietly removed is worth less
than one with them in it, because the second one can be believed. Nobody reading yours
expects it to be perfect and everybody notices when it is.

**77. Hand it over in person where you can.**

Two signatures and a real conversation. The file will move by any route that works, and the
routes that work best are the ones where somebody already knows why they are taking it.

**78. Your standing is not your worth.**

It is a measurement of how often your machine was awake. People will still turn it into a
score, because people do that, and the correct response is to keep yours honest and to
refuse to read anybody else's as a statement about them.

**79. Count what you saw, not what you assume is out there.**

Your node knows what reached it. Anything beyond that is an estimate, and repeating an
estimate as a number is how a network talks itself into being bigger than it is.

**80. If you are stopping, say so.**

One sentence, before you go. The people who were counting on your node cannot tell the
difference between somebody who left and somebody whose power went out, and the second one
is worth going to check on.

**81. Come back without explaining yourself.**

Nobody is owed the reason you were gone. The window moves, the record fills again, and there
is no penance step and no probation. That is deliberate: a faith you cannot rejoin quietly is
one people leave permanently.

## On the word

**82. Start with the invocation.**

Every prayer, and every time you tell somebody about this who has not heard of it, starts the
same way.

> **To 333 I offer 333, and I speak 333.**

Say it first, whatever comes after. Nine words, one of them three times. It is the difference
between talking about 333 and talking as one of us.

**83. Show it, do not push it.**

Say what it is, say what it costs, hand over the address, and let the person take their own
time. Anybody you have to argue into this will have to be argued into staying, every epoch,
for ever.

**84. Do not claim anything about 333 that cannot be checked.**

Not what it will do for somebody. Not how many of us there are, beyond what your own node has
seen with signatures on it. The whole design refuses to assert what it cannot verify, and you
can manage the same standard in a conversation.

**85. Speak plainly to people who do not know the words yet.**

Jargon is a fence, and the people on the other side of it are the ones you wanted. Say the
thing, and then say what it is called.

**86. Correct yourself where you were wrong.**

Not in a private message afterwards. In the thread, under the sentence, where everybody who
read the mistake will read the correction.

**87. Say the cost in the same breath as the offer.**

A machine left on, an address somebody learns, a directory that cannot be replaced if it is
lost. Anyone who finds that out later finds out that you knew, and everything else you said
goes with it.

**88. Let a no be a no the first time.**

Say the thing once, well, and then stop. The third law gives everybody the right to walk
around holding whatever they already hold, and that includes walking away holding nothing of
this.

**89. Do not speak for 333.**

You can say what the Law says, what the protocol does, and what you yourself think. Nobody
has been appointed to say what 333 wants, and the sentence "333 wants" is the first move of
every institution this was built to avoid.

**90. Write it for somebody not born yet.**

Most of the people this is addressed to do not exist. They will not have your context, your
platform, your language or anybody to ask. Anything that depends on being there at the time
will not reach them.

## On what you build on

**91. Prefer the thing you could replace.**

Not the thing you intend to replace — the thing you could, if the terms changed or the
company was bought. It is a different question from which is better today, and it is the one
that is still relevant in five years.

**92. Learn how to get your things out before you put them in.**

If there is no answer, that is the answer. A service that makes leaving easy is telling you
something true about how it plans to keep you.

**93. A free service is a service that can be stopped.**

Somebody is paying, and the day the arithmetic changes you will find out in an email with
thirty days in it. Free is a fine price. Just know that it is a price and not a promise.

**94. Pin the version, and write down why that one.**

An unpinned dependency is a program that changes while you are asleep. A pinned one with no
note is a program nobody dares to update. The note is the whole difference: what broke, what
you checked, what would have to be true to move.

**95. If it only builds on your machine, it does not build.**

Write down every step, including the ones you stopped noticing years ago, and then have
something that is not you follow them from nothing. Everything you left out is a thing the
next person cannot get past.

**96. Do not let the tool choose the shape.**

Frameworks come with an opinion about what your thing is, and it is easier to accept it than
to notice it. Decide what you are building first, in words, and then find out what the tool
makes hard.

**97. Own the address, or expect to move.**

A name you rent on somebody else's platform is theirs, and the audience that found you
through it is theirs too. Anything you want to still have in ten years wants an address you
control and a way for people to reach it that does not route through a company.

**98. The standard is the one other people implemented.**

Not the one with the best document. A format two independent programs agree on is a fact; a
specification with one implementation is that implementation with extra pages.

**99. Build so that leaving takes a day.**

Every dependency, every host, every service: know roughly what moving off it would cost, and
keep that number small enough to survive a bad surprise. You will almost never have to do it,
and the version of you who does will be in a hurry.

## On the end

**100. Say out loud which decisions cannot be taken back.**

Bytes on the wire, the shape of a stored record, a published identifier, a promise in a
licence. Those get a different amount of thought from everything else, and the way to make
sure they get it is to name them as they go past, in writing, where somebody can object.

**101. Write for the day nobody can be reached.**

Not the day you are on holiday — the day the author is unreachable permanently and the
program is still running. Every "ask me if you need to know" is a fault that has not
triggered yet.

**102. Do not build what only you can maintain.**

Cleverness that requires you is a hostage situation you are the victim of. The measure is
simple: could a competent stranger with the source and the documents keep this alive without
you? If not, the thing you built is smaller than you think.

**103. Anything that must never be turned off will be turned off.**

By a power cut, a bill, a court, a mistake or a bored teenager. Design for the restart rather
than against it: what does it do when it comes back, what did it lose, and how does anybody
find out.

**104. Let it be allowed to end.**

A thing that cannot die has to be kept alive by somebody, for ever, and that somebody
eventually resents it. Give it a condition under which it stops, make that condition honest,
and then it is being kept because people keep choosing to, which is the only kind of alive
worth having.

**105. Record the reason, not just the decision.**

A decision without its reason cannot be revisited, only obeyed or broken. Ten years later the
reason may have expired — that is a fine outcome and it is only available to people who wrote
it down.

**106. Leave more than the code.**

The code says what it does. It does not say what you were afraid of, what you tried that
failed, what you decided not to build, or which part you never trusted. That is the half that
does not survive on its own, and it is the half the next person needs.

**107. Do not make the future ask permission.**

Everything you lock down, name after yourself, or route through your own approval is a wall
somebody has to climb after you are gone. Build the thing so that a stranger can continue it
without finding you first.

**108. Assume you were wrong, and make being wrong cheap.**

Not humility for its own sake — arithmetic. You will be wrong about some of this, you do not
know which part, so the only general defence is that mistakes stay small, stay visible and
stay reversible. Everything above is a way of paying for that.

## The last three

**109. Keep the Law, and remember it is 33.3% right.**

It says so about itself. Keep it completely, in the knowledge that it is going to change
under you, and without mistaking the version you were handed for the thing it approximates.

**110. None of this is worth hurting yourself for.**

Not one line of it, and not all of it together. The second law puts this above every
recommendation on purpose. If keeping one would cost you your work, your health, your
household or somebody else's safety, then the Law is that you do not keep it, and you have
not failed at anything.

**111. Pass it on.**

That is the whole of it. Everything else here is about doing it well, and none of it matters
if the file stops moving. Three bytes, two signatures, one person who did not have it
yesterday.

---

## What is deliberately missing

There is nothing here about what to eat, what to wear, who to marry, how to raise a child, or
what to do with money. Not out of modesty. Those are not the subject.

This is a document about building something carefully, about how it should look once it is
built, about keeping a thing running for longer than anybody planned, about disagreeing
without heat, and about talking about something you believe without becoming unbearable.

If you wanted more than that, the door is open anyway.

---

There will be 333. These are the first 111, and the ones after them will be written by people
who learned something the hard way and thought it was worth the next person's time. Nobody
alive will write the last one.

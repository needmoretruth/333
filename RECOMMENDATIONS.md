# The Recommendations

These sit under the third law. They are advice with numbers on them.

None of them is worth hurting yourself or anybody else to keep. [The Law](LAW.md) says so in
the same breath as it asks for them. If one of them is wrong for the situation you are in,
then it is wrong. Say so out loud, and give a reason.

They were written by people who have been awake at three in the morning arguing with a type
checker. It shows, and that is not an apology.

---

## On the number

**1. Count in threes.**

When the number does not really matter, pick three. Retries, environments, people in a
review, tries before you stop and think again. You will start noticing how often it did not
matter.

**2. Do not give 6 a place of honour.**

Not the port number, not the version you pin, not how many workers you run. If somebody else
has put a 6 in front of you, note that it was their choice and get on with it. A number in
the specification of another company is not a number of yours.

**3. Never keep one of anything.**

One server, one maintainer, one copy of the key, one person who knows how the deployment
works. If you have exactly one of something, you are one bad day away from having none.

**4. Three is the smallest number that can disagree.**

Two machines that differ just sit there and disagree. Three can have a majority. That is why
quorums start at three, and it is also why an argument between two people almost never ends
on its own.

**5. If it fails three times the same way, the method is wrong.**

The first failure can be bad luck. The second one is you being stubborn. By the third, stop
fixing the attempt and change the approach.

## On the machine

**6. Write it in Rust.**

Not because the other languages are bad, but because of what you are writing for. This is
meant to keep running when nobody who wrote it can be reached. A compiler that catches the
problem at build time saves somebody a phone call at four in the morning, and that is the
whole argument.

**7. Small things can be TypeScript.**

One page, one script, one afternoon. Reaching for the smaller tool is fine. Know which of the
two you are doing before you start, and say which.

**8. JavaScript without types is the language of the heretic.**

Said gently. Half of everything runs on it and some of that half is lovely. But a program
that cannot say what it is holding finds out in front of a user, at the worst hour, and
somebody gets woken up. Add types and it stops being heresy. It becomes TypeScript, and it is
welcome.

**9. Read what you depend on, or do not depend on it.**

Every dependency is a promise from somebody you have never met, about a machine that does not
exist yet. Take fewer of them, and take smaller ones. If the choice is between a thousand
lines you write and a hundred thousand you will never open, at least notice that you are
choosing.

**10. Every unsafe block is a confession, so write it down.**

Above the block, say what you are promising, what you checked, and what breaks if the promise
turns out to be wrong. If nothing is written there, the next person will assume somebody
already thought about it.

## On what breaks

**11. A silent failure is a lie that takes longer.**

If you swallow an error, the problem turns up somewhere else, later, in front of somebody who
cannot fix it. Let it out where it happened, and let it say what it was doing at the time.

**12. Reproduce it before you fix it.**

If you never saw the bug, you are not fixing it. You are changing the code and telling
yourself a story. Make it happen on purpose, make it stop, then make it happen again to be
sure it really stopped.

**13. Write the log for the night you are asleep.**

Say what happened, what it happened to, and what the program did next. Whoever is reading it
at three in the morning does not have your code open and cannot ask you anything.

**14. Blame the system, not the person who found the hole.**

Somebody who can break it has done you a favour. If one wrong click can destroy something,
the click was never the problem.

**15. A fix with no test is a rumour.**

Write the test first and watch it fail. A test you write afterwards, that passes the moment
you write it, is measuring how confident you feel.

## On the made thing

**16. Use black, white, and the greys in between.**

Colour is a way of saying this matters more than that, and it is the first thing to break.
Cheap screens, greyscale printouts, direct sunlight, and readers who cannot tell your red
from your green. Say it with size, weight, order and space instead. Those still work in all
four cases.

**17. Colour is fine as a dot.**

A small mark saying that something is failing, waiting, running or finished is exactly what
colour is good at, and red, amber, green and blue belong there. Keep it small. It should not
set the mood of the page, it should not be the first thing anybody sees, and it should never
be the only thing carrying the meaning. Put a word next to it.

**18. Matte, not glossy.**

Shine, gradients, glass and fake shadows are pictures of a material that is not there. They
cost something to draw and give the reader nothing they asked for. Flat ink on flat paper has
been readable for five hundred years without a graphics card.

**19. Take things away until it breaks, then put one back.**

What is left should be readable at arm length, in the dark, at double the text size, by
somebody in a hurry who did not choose to be here. Readability is not one feature among
others. It is the job.

**20. A setting is a decision you handed to somebody who knows less than you.**

Pick the default as if there were no setting at all. Then add the setting for the people the
default is wrong for, and let them type a real value instead of choosing between three names
you invented.

## On time

**21. Estimate from what you finished, not from what you can imagine.**

What you imagine is always the version where everything goes well. Look at the last three
things you did of that size, and take the slowest one.

**22. Nothing decided after midnight survives the morning.**

Write the decision down, sleep, and read it again. If it was right, it cost you eight hours.
If it was wrong, it saved you a week.

**23. Ship the small version today.**

One person using a real thing for an afternoon teaches you more than a month of planning.
Build the big version afterwards, once somebody has actually watched the small one being
used.

**24. A deadline you set for yourself is a wish with a date on it.**

Tell somebody who will notice what you will have done, and by when. That is a deadline.
Everything else is a way of scheduling your own guilt.

**25. Rest one hour in three.**

333 still counts you while you do, and you should count yourself too. Working through the
rest does not get you three times as much. It gets you the same amount, plus a bug somebody
else finds much later.

## On the mind

**26. Argue with reasons, in a level voice.**

How you feel is good evidence about you and no evidence at all about the thing. Take the
feeling to somebody who cares about you. Take the argument to the thread.

**27. Think, and then actually decide.**

Thinking that never turns into a decision is a hobby. Careful people rarely fail by being
reckless. They fail in week four of a discussion about a change that would have taken an hour
and could have been undone for nothing.

**28. Refuse violence, and know the exception.**

Refuse it because it is almost always the worst tool available. It costs more than it
returns, it wrecks the thing it was aimed at, and it closes every door that was still open.

The exception is narrow. Either there is nothing else left, or it is honestly the option with
the least harm in it, and you should be able to say which of the two out loud before you
move. Somebody in one of those situations is not a heretic for it. But it is much rarer than
it feels at the time, and the feeling that it is not rare is the thing this is warning you
about. Anything you decide while angry was decided by the anger.

**29. Change your mind where people can see you do it.**

Say what you used to think, say what changed it, and carry on. If nobody has ever seen you do
that, nobody can safely disagree with you.

**30. Hold the position you can argue against.**

Make the case for the other side as well as you can, before you make your own. If you cannot
make it, you do not know what you believe yet. You know what you heard.

## On other people

**31. Review the code, not the person.**

Write about what the code does. They will read it as being about them anyway, so do not give
them anything true to point at.

**32. Say which part you did not understand.**

It is the most useful sentence in a review and the hardest one to type. Code that one
competent reader could not follow will be maintained by guessing.

**33. Give the credit away.**

There is more of it than you think, and it is worth more to somebody else than it is to you.
Take the blame in the same proportion and you are still ahead.

**34. Explain it while you still remember being confused.**

The best explanation of anything is written by somebody who understood it last week. In a
year you will have forgotten which part was hard, and you will write the version that only
helps people who already know.

**35. Whoever is on call is right until it is over.**

Argue about the design tomorrow. During the incident, one person is holding it and everybody
else is offering rather than directing.

## On what you keep

**36. Collect less about people than you are allowed to.**

Every field you store is a field you have to protect, explain, hand over when somebody asks,
and eventually lose. The cheapest data to defend is the data you never collected.

**37. A backup you have never restored is a story about a backup.**

Restore it somewhere else, from nothing, following the notes you actually wrote down. Do it
before you need it, because on the day you need it you will be doing it badly.

**38. Keep secrets where you cannot read them by accident.**

Not in the repository, not in the log, not in the error you paste into a chat window. Treat
any secret that has been on a screen once as a secret that is gone.

**39. What you delete cannot leak.**

Deletion is a security measure, and it is the only one that never expires. Decide the
deletion date when you collect the data, not when somebody comes asking about it.

**40. Sign what you say, and let the signature travel with it.**

Something that is only trusted because of where you found it stops being trustworthy the
moment it moves. Something that carries its own proof can be passed hand to hand by
strangers. That is how anything outlives the place it came from.

## On the word

**41. Start with the invocation.**

Every prayer, and every time you tell somebody about this who has not heard of it, starts the
same way.

> **To 333 I offer 333, and I speak 333.**

Say it first, whatever comes after. Nine words, one of them three times. It is the difference
between talking about 333 and talking as one of us.

**42. Show it, do not push it.**

Say what it is, say what it costs, hand over the address, and let the person take their own
time. Anybody you have to argue into this will have to be argued into staying, every epoch,
for ever.

**43. Do not claim anything about 333 that cannot be checked.**

Not what it will do for somebody. Not how many of us there are, beyond what your own node has
seen with signatures on it. The whole design refuses to assert what it cannot verify, and you
can manage the same standard in a conversation.

**44. Speak plainly to people who do not know the words yet.**

Jargon is a fence, and the people on the other side of it are the ones you wanted. Say the
thing, and then say what it is called.

**45. Correct yourself where you were wrong.**

Not in a private message afterwards. In the thread, under the sentence, where everybody who
read the mistake will read the correction.

---

## What is deliberately missing

There is nothing here about what to eat, what to wear, who to marry, how to raise a child, or
what to do with money. Not out of modesty. Those are not the subject.

This is a document about building something carefully, about how it should look once it is
built, about disagreeing without heat, and about talking about something you believe without
becoming unbearable.

If you wanted more than that, the door is open anyway.

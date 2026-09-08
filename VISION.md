# The Vision

What follows is not a plan. A plan is a promise about dates, and there are none here.

It is what has been seen: the shape 333 takes when it is finished, written down while it
is unfinished, so that the distance between the two is something anybody can measure.

**Nothing here comes to pass on its own.** No prophecy in this document is fulfilled by
waiting. Each is a piece of work, and it will be done by the believers of 333 or it will
not be done.

Each prophecy is written as a direction, because a direction still means something when
there are a thousand of us and this year's particular job was finished years ago. Under
each one is **where it stands**, which is the part that goes out of date, and **fulfilled
when**, which is written so that you can tell whether you are done without asking
anybody.

---

## The charge

**Nothing in this document is a request.** A request draws a line around itself, and
everything on the other side of that line goes undone — which is exactly the failure this
document exists to prevent. 333 does not ask. It says where it is going, and the believers
of 333 are charged with moving it there.

So read the prophecies as bearings and not as a queue of tasks. Where one names a piece of
work, that work is one way of moving in that direction. Where you can see a way it did not
name — because you know something the person who wrote it did not, or because the world
changed after it was written — that is the same direction and it needs nobody's permission.
The list is not the boundary of what is asked, because nothing here is asked.

**And none of it is confined to this repository.** This is where the client is kept. It is
not where 333 is. Serving this direction may mean writing nothing here at all: an entirely
new platform, a foundation, a technology that belongs to no repository and no company, or a
thing that belongs to nothing whatsoever. All of that is service and none of it needs an
invitation.

One line runs under all of it and it is the only one: there is one network. Build anything,
anywhere, under any name — but a second 333 that cannot speak to this one is not a
contribution to 333, it is the end of it, and no amount of technical merit changes that.
The same thing from the other side: an edition of the client written by somebody else is
welcome, on the one condition that it speaks the whole protocol.

---

## What 333 esteems, and will take when it can

None of this is a requirement and none of it is a test of anybody. It is a direction. Each
of these is something 333 would already be built on if it could be, and the reason it is
not is a limit of what exists today rather than a judgement against it. **When the limit
goes, 333 takes it.** That is the vision; the present arrangement is the compromise.

**Three, and machines that count in threes.** Computers settled on two states for reasons
of manufacture, not of truth. A machine that counts in threes is the machine this was
named for. You cannot buy one. If that changes, 333 belongs on it, and the client should
be the thing waiting there when it arrives.

**Twelve, as the way to count.** Ten divides by two and five. Twelve divides by two,
three, four and six. Every number a person reads here is written in ten because that is
what people read; a client that could show its hours and its counts in twelve, for whoever
wants it, would be closer to the thing than the client we have.

**A language with one parse.** Lojban was built so that a sentence cannot be taken two
ways. Most of what goes wrong between people goes wrong before anybody disagrees, in the
sentence that could be read either way. What the client says is English prose today, and
prose is ambiguous. There is a version of this where the sentences that decide something
are also written in a language that cannot be misread.

**Rust.** The one on this list that is not deferred. It is what this is written in and
what it keeps being written in, because a relic that fails at runtime in twenty years is
not a relic.

**Proofs that show nothing but that they are true.** Zero-knowledge is the shape this
faith wants: check without learning, believe without being told. The protocol uses none,
for two reasons written down when it was removed — a bug in a circuit does not show on
the surface, and after 3.3.3 it could not be fixed; and in a faith where taking part is
public, anonymity was not needed for what it was being proposed for. The first of those
is a statement about tooling in 2026, not about the mathematics. If proving becomes
something an ordinary machine can do and a circuit becomes something an ordinary person
can check, that reason expires, and 333 takes it.

**And none of these is the point.** If something plainly better appears — a cleaner base,
a clearer language, a safer tool, a stronger proof — 333 esteems that instead and this
section is what changes. A taste defended after it has been beaten is not a taste. It is a
habit.

---

## What does not change

Everything else in this document can be argued with. These five cannot, and a change that
gives one of them up is a different thing wearing this name.

**Nothing here may become an authority.** There is no vote, no quorum of opinion, nobody
to appeal to and nobody who decides who is right. Each of us judges from what we saw, and
two of us may hold different pictures with neither of us wrong. The moment the design
needs us to agree, there is somewhere for somebody to stand, and the change worth writing
is the one that removes the need.

**Nothing is claimed that cannot be checked.** No claim that a node runs genuine code, no
claim about anybody's honesty. What is verified is what was done.

**The file is passed on, never made to order.** A client carries the hash and can
recognise `333.txt`; it takes the bytes from somebody who already holds them. There is one
deliberate exception, `333 bootstrap`, for the case where there is nobody to take them
from, and a node that used it is visibly the start of its own line.

**One client, and it runs alone.** A node must not need a second program to be a node: no
daemon beside it, no service to sign up to, nothing to install first. It carries its own
storage, its own way onto Tor, its own hours. Light and Standard are editions of that one
client, differing by whether the screen is compiled in, and the specification allows a
community edition beside them on two conditions — full protocol compatibility, and no
forking of the network.

The one thing the client does not carry is deliberate: the program that speaks an
obfuscated bridge, for places where Tor itself is blocked. That one chases a target which
moves, and a copy frozen inside this would be the wrong copy within a year while looking
like the right one.

**It spreads only where it is welcome.** Nothing here may install itself, hide itself, run
on a machine it was not given, or arrive inside something somebody wanted instead. There is
no exception for a good cause, and none for a technique that would work very well. A faith
that arrives uninvited is not being kept, it is being done to somebody, and a believer
counted because their machine was taken is not a believer.

**A constraint that applies to every prophecy below:** the weakest machine anybody would
keep a vigil on decides what the rest of us can use. Every megabyte and every dependency
is somebody, somewhere, who cannot take part. The released files today are between 15 and
25 megabytes depending on the machine: smallest on a Mac, largest on 64-bit ARM, and the
Raspberry Pi Zero build is nearer the top of that range than the bottom.

---

## What the words mean

The documents and the code use different words for the same things on purpose: the code
uses the ordinary technical name so that a stranger can read it, and the doctrine uses its
own. This is the map between them.

| what we call it | what the code calls it | what it is |
|---|---|---|
| the file, the relic | `Subject` | `333.txt`, three bytes, recognised by its hash |
| the hours, an hour | `Epoch` | 333 minutes; everything is stamped with one |
| the vigil | `serve` | a node running, answering whoever knocks |
| the roll | `Roll` | the nodes this node knows were handed the file |
| an admission | `Transfer`, `Record` | the two signed halves of one handover |
| being drawn | `draw` | the lottery deciding who asks whom this hour |
| the question | `Challenge` | a verifier's nonce, put to a node |
| a witness, witnessed | `Attestation` | a verifier's signed statement about what happened |
| standing | `Standing` | how much of the window a node was present for |
| the window | `WINDOW_EPOCHS` | the last 333 epochs, moving; older than that is gone |
| the word, saying one | `Signal` | one of the 333, said at most once in an epoch |
| the last silence | `SILENT_EPOCHS_BEFORE_THE_END` | 333 epochs in which nobody answered anybody |
| the unseen road | Tor, onion | reaching or being reached without an address |
| the meeting point | `Meeting` | the333.dev, where strangers look for each other |

---

## The first order: that anybody can be reached

Everyone who cannot be reached is not asked, is not witnessed, and cannot hand the file to
whoever comes next. This order is about the distance between 333 and the next person who
would have kept it.

### Every door opens

*A node behind anything is reachable, and nobody had to be told how.*

The obstacle is not one obstacle. It is a router that was never told to let anybody in, a
provider that gives a household no address of its own, a country that blocks the way
around both.

**Where it stands.** On startup the client asks the router to forward its port over
UPnP-IGD, then knocks on its own outside address to find out whether the forwarding
actually worked, and says `open` or `shut`. Routers that speak NAT-PMP or PCP instead are
asked nothing — there is no NAT-PMP code in the tree. There is no hole punching either:
two nodes whose routers both refuse cannot reach each other, even through the meeting
point, which only introduces. Onion addresses work from anywhere and cost minutes of Tor
starting up. Bridges exist for a blocked country, and the program that speaks the
obfuscated ones has to be installed by hand.

**Fulfilled when.** A node behind a router that answers NAT-PMP or PCP is reachable
without its owner opening anything, proved by the same knock that proves UPnP worked; and
two nodes whose routers both refuse to forward exchange a heartbeat directly, with the
meeting point doing nothing but the introduction.

### The meeting place grows smaller

*The fixed address is no longer needed, and nothing replaced it.*

One address everybody looks at is one address that can be taken away, and one operator who
learns who is asking. We said in the open that we mean to depend on it less. Saying it is
not doing it.

**Where it stands.** `the333.dev` is written into the client. A node leaves the same
signed statement about where it can be reached that it already writes, and reads the ones
others left; the board holds each for two epochs and forgets it. Everything read is
verified where it lands and nothing there is trusted. Nodes already trade what they know
directly, including addresses (`crates/net/src/gossip.rs`), so the beginning of the answer
exists — what is missing is a node continuing to find *new* nodes with the meeting point
switched off.

There is a cost to the arrangement beyond the operator learning who asked: the meeting
point also records the country and a whole-degree position of each node that publishes a
non-onion address, and serves it publicly at `the333.dev/map/`. Onion addresses are
counted as Tor and placed nowhere.

**Fulfilled when.** A node started with `--no-meet`, holding one address it was given by
hand, is still learning about nodes it has never met a week later, and the client can say
which of the things it knows came from where.

### It speaks to whoever comes

*It was read in the reader's own language, and the words meant one thing.*

A faith that only reaches people who read English has a border around it. Underneath the
translation problem is a harder one: sentences that cannot be taken two ways in any
language, which is what Lojban is doing on the list above.

**Where it stands.** English only, and the words are mixed into the code that decides when
to say them — `aloud!` calls with inline formatting and hand-aligned columns, spread across
dozens of files. Nothing extracts them.

**Fulfilled when.** The words a person reads are separate from the code that chooses when
to say them; one other language is complete enough that a person can run `id`, `serve`,
`join`, `say` and `status` and read every line; and adding a third language touches no
Rust.

---

## The second order: that the client can be believed

Nothing here asks to be trusted, which holds only while the program in front of a person
is doing what it says and saying what it did.

### It says what happened

*No line the client printed was more certain than the thing it described.*

A program that overstates teaches people to stop reading it, and an instrument nobody
reads is a faith running on somebody's word.

**Where it stands.** Some of what it says is wrong in small ways. The known one: the
meeting point answers `429` when a node has already left a statement this epoch, and its
earlier statement is still on the board. The client prints `did not take this node's
address: the meeting point answered 429`, which reads as a failure and is not one. The
number survives; the sentence around it is the part that lies, and the meeting point's own
explanation of the number is thrown away. There will be others of the same shape — a
status, an error kind or an absence reported as something stronger than it is — and they
are found by reading what the client prints beside what actually happened.

**Fulfilled when.** Waiting is reported as waiting, and every error the client prints
names the thing that actually happened rather than the category it was sorted into. This
is where to start if you want to start.

### It knows itself

*The client could be recognised, and did not claim to be more than it was.*

A client carrying its own hash cannot hash the place the hash is written. What is needed
is a boundary — what the hash covers, what it deliberately does not, and why there rather
than somewhere else — narrow enough to check by hand and wide enough to be worth having.

This is not the same thing as verifying a peer, which is refused and stays refused: the
hash lets *you* check the copy in front of *you*, and proves nothing about anybody else.

**Where it stands.** Nothing. There is no self-hash in the client and no definition of
what it would cover. The specification records it as open and marks it as a thing that
must be settled before 3.3.3, because after that the bytes are fixed and a mistake in them
is permanent.

**Fulfilled when.** A person with the source and a hashing program arrives at the same
number the client reports, following only what is written down, and can say without
reading the code which files and which bytes were counted.

### It has been run

*This was kept on every kind of machine, and somebody was there to say so.*

**Where it stands.** Every release builds twelve files: Standard and Light, for x86-64
Linux, 64-bit ARM, ARMv6, both kinds of Mac, and Windows. The only ones anybody has
reported keeping a vigil on are the two x86-64 Linux builds. No report has ever come back
about any of the others — they may be perfect, and nobody knowing is the problem.

This is the smallest work on this page and it may be the most useful, and it needs no
skill at all. Take the file for your machine, keep the vigil on it, and say what happened,
including that it worked, which is the report nobody thinks to send.

**Fulfilled when.** An issue exists for each of the twelve files saying somebody ran it
through a full hour and what the screen showed.

---

## The third order: that it outlives us

Nineteen thousand six hundred and eighty three years is the number in the doctrine. It is
a long time to be relying on somebody remembering to restart something.

### The vigil does not break

*A node kept running through the night, and nobody stayed awake for it.*

A node that stops when a laptop closes is absent from every hour its owner slept through,
and over a window of 333 that is the difference between belonging and not.

**Where it stands.** A systemd unit and a launchd agent in `packaging/`, installed by
copying files and reading a comment. No package for any distribution, nothing at all for
Windows, and nothing anywhere that tells a person their node stopped three days ago.

**Fulfilled when.** It installs the way anything else on the system installs — a package
for at least one distribution, and a service on Windows — it survives a reboot without
anybody reading anything, and a person who has stopped being counted finds out from their
own machine rather than from their standing.

### A name can be carried

*A believer moved house and remained the same believer.*

A node's name comes from its key, its standing is what others signed about that key, and
its unseen address is a second key beside it. All of it lives in one directory.

**Where it stands.** `333 id` warns about it on the run that makes the name, which is the
only run the warning can still be acted on. `serve`, `join` and `bootstrap` all make a name
too, and none of them says it — so whoever never typed `id` was never told. There is no
command that packages a node for moving, nothing that refuses to start when it finds a
directory that was copied rather than moved (two nodes running one key is a node
contradicting itself), and nothing that says what was lost when somebody starts fresh by
accident.

**Fulfilled when.** A person can move a node to another machine with one command at each
end, the client refuses to run two copies of one name and says why, and the thing that
would have been lost is named before it is lost.

### The words are written

*The 333 were said, and each of us knew what had been said.*

Nodes say one of 333 things each hour, and what travels is a number. The table of what the
numbers mean is empty. It is the largest declared gap in the whole of 333 and the only
part of it that needs no Rust at all.

**Where it stands.** `SIGNAL_COUNT` is 333, a `Signal` is a number in that range, the
client will say one for you and count what others said, and it tells you plainly that the
words are not written yet. Which words they are is settled at 3.3.3 and not before.

**Fulfilled when.** There is a table, it has 333 rows, and the client shows the word
beside the number.

---

## The fourth order: that it belongs to nobody

Everything this is built from came from somewhere else, and everything that came from
somewhere else came with a condition attached. The conditions are light and not one of them
is unjust. They are still somebody else's conditions, written by people who had never heard
of this, for arrangements that were not this one.

That is tolerable now and it is not the end. This is the order furthest from being kept,
and the work in it is measured in years rather than in changes.

### Nothing in it is owed

*333 is carried on terms 333 wrote, over parts written for 333.*

The licence on this repository is Apache-2.0. It was taken because it was the safest and
clearest thing that already existed: it asks little, it grants patents along with the code,
and it reaches into nothing built beside it. It was not taken because it is right. It is a
licence written for other people's arrangements, and this is an arrangement nobody has
written a licence for — a file meant to move hand to hand, a network that has to be
allowed to die, and a client that must not be forked.

The end of that road is a licence of 333's own, written for what 333 is, saying what may be
done with the client, what may be done with the file, and what a fork is, in the terms this
faith actually holds rather than the terms that happened to be lying around.

A licence of one's own is worth little while everything underneath carries somebody else's.
So the same prophecy covers the rest of it: a client whose parts were written for it,
from nothing borrowed. Not a rewrite out of pride, and not a judgement on anybody whose
work is in here now — a clean room, so that every condition on every line is one 333 took
knowingly and nothing arrived by accident. Code, and everything that is not code: what it
draws, the shapes of its letters, the sounds it makes, the words themselves.

Nothing in it today is permanent, and that includes what it could not currently do without.
Tor is here because it is the best thing that exists for reaching a machine nobody can
address, and it is here until something better exists or 333 builds it. Rust is what this
is written in until there is reason to write in something else. 333 writing a language of
its own is not too large a thing to put in this document — it is exactly the size of thing
this document is for. **No technology owns this client. The client holds the technology,
and puts it down when it stops serving.**

**Where it stands.** The licence is Apache-2.0 and `333.txt` is under the Unlicense. The
released binary is built from 469 crates of other people's code, of which 25 carry terms
outside the permissive set the rest use, and every one of those arrives through Tor. The
repository does not yet carry the licence texts it is obliged to pass on alongside them,
which is something owed today rather than at the end of the road.

**Fulfilled when.** Every part of the released client is either written for 333 or carried
on terms 333 chose knowing what they cost; the licence over all of it was written for this
rather than adapted to it; and there is nothing left in it that could not be replaced by
333's own work if the terms it came on ever changed.

### A machine that does nothing else

*A believer was handed a thing, plugged it in, and the vigil was kept from that hour.*

A vigil kept on a machine somebody also works on ends the first time that machine is needed
for something else. Most of the third order — the service that survives a reboot, the name
that can be carried, the node that does not stop when a laptop closes — is working around
one fact: almost nobody has a computer they can leave alone.

The answer to that is not more software. It is a small, quiet, cheap thing that is bought
or given, plugged into power and a network, and left. It does one thing and has no other
purpose to be reclaimed for. Somebody who owns one is present in every hour without
thinking about it again, and somebody who cannot program is present on exactly the same
terms as somebody who can, which is the part that matters.

What that thing is, who makes it, and how it reaches people is the work. That it must be
cheap enough to give away is not negotiable: the constraint above bites hardest here,
because a device only the comfortable can afford chooses who the faith is for.

**Where it stands.** Nothing. The client runs on a Raspberry Pi Zero, which is the nearest
thing that exists and is not the same thing — somebody still had to buy it, write an image
onto it, and know what to type.

**Fulfilled when.** A person who cannot program can obtain one object, connect it, and be
counted in every hour from then on without reading anything further, and what they got did
not cost more than a meal.

### It travels only where it is welcome

*333 was carried further, and every person carrying it had chosen to.*

The file has to move, and everything that helps it move belongs here: the ways two
strangers find each other, the ways an invitation is passed, the ways somebody who has
never heard of this is shown what it is and given the choice. Anything built for that is
welcome, and 333 building its own is welcome too — its own way of meeting, its own
transport, its own hardware, its own language for saying any of it.

All of it inside one sentence, which is above under what does not change and is repeated
here because this is the prophecy it constrains: **it spreads only where it is welcome.**
Nothing that installs itself. Nothing that hides. Nothing that runs on a machine it was not
given. Nothing folded into something else somebody wanted instead. The test is not whether
a technique would work — most of the forbidden ones work extremely well — but whether the
person at the other end chose this, knowing what it was.

**Where it stands.** Three ways of meeting: an invitation handed over, other nodes on the
same network found without anybody typing, and one fixed address strangers look at. Each of
the three ends with a person deciding to run something. There is nothing else, and there
has never been anything anybody would need protecting from.

**Fulfilled when.** Somebody who has never heard of this can be shown it, understand what
it is, and begin keeping it without a believer standing next to them — and could say
afterwards exactly when they agreed to it.

---

## The prophecy is not a cage

Everything above is what was seen from where we are standing. Standing somewhere else, you
may see further.

If there is a way that is plainly better — simpler, smaller, harder to break, reaching
people these do not reach — then that is the way, and this document is what changes. A
vision defended after a better one has beaten it is not a vision. It is a habit. What does
not change is the five things above under that heading.

---

## A holy pull request

This is the one way of serving that this repository can take
directly, which makes it the smallest of the ways and not the highest. Everything below is
how to send something here without it being rejected for a reason that had nothing to do
with the work.

**Where to ask.** [Issues](https://github.com/needmoretruth/333/issues) — for a report
about a machine you ran this on, a defect, or a question about why something is the way it
is. Ask before writing anything that changes how the protocol behaves, as opposed to how
it is written; numbers, thresholds and orders of operation that look arbitrary usually are
not, and most of the reasons are in the doc comments.

**Before you send it.**

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The first build takes a while — the tree is 639 packages, most of them Tor — so start it
before you make tea. There are 313 tests and they run in seconds once it is built.

**What the checks actually enforce.** Every push runs formatting, clippy with warnings
denied over two feature sets, and the tests. The workspace refuses `unsafe` outright and
warns on `unwrap`, `expect`, indexing and missing documentation, so those are decided
before a person looks. Everything else below is read by a person, not by a machine, and
two of them are rules this repository does not yet keep itself — they are marked.

**Sign your work.** `git commit -s` adds a `Signed-off-by` line certifying the
[Developer Certificate of Origin](DCO): the work is yours to give, or you have the right
to pass it on under this licence. There is no copyright assignment and nothing to sign in
ink; what you write stays yours. *Kept from 2026-09-08 onward and not before it, which is
visible in the history.*

**Rust for the client.** Everything under `crates/` is Rust and stays Rust, with no build
scripts and no tooling in another language. What is not under `crates/` is not Rust and was
never going to be: `meeting/` is six hundred lines of TypeScript running on somebody else's
edge, `site/` is the pages it serves, and `.github/workflows/` is YAML because that is what
the machine running it reads. None of those is a place to move client logic into.

**One file, one responsibility, four hundred lines.** Counted up to `#[cfg(test)]`; the
tests below that line live beside the code they test and are not counted. A function fits
on one screen. *Two files are over it today: `crates/cli/src/commands/serve.rs` and
`crates/cli/src/screen/draw.rs`. Splitting either is a good first change.*

**Say why, not what.** Everything public carries a doc comment giving the reason rather
than restating the code. A comment that repeats the line below it is one more thing to
keep true.

**Tests that could fail.** A test that passes before your change as well as after is not
testing your change; show the failure first. Anything frozen — bytes, constants, domain
strings — is pinned against a literal, because a value compared only against itself is not
tested at all.

**Dependencies.** Prefer a crate that exists over writing it again, and never write
cryptography, TLS, password hashing or random numbers by hand. The fourth order points the
other way in the end, at a client built from nothing borrowed; that is a road measured in
years, and taking a step down it by hand-rolling something today would only mean one more
thing to get wrong.

Adding a direct dependency means saying in the manifest what it is for and what its
features cost. The tree today is overwhelmingly Tor's: 639 packages, of which 469 reach the
released binary. Twenty-five of those carry something other than the MIT, Apache, BSD, ISC
and public-domain licences the rest do: eighteen Unicode-3.0, three MPL-2.0, three Zlib,
one BSL-1.0 and one CDLA-Permissive-2.0, every one of them arriving through Tor. None is
GPL, AGPL, SSPL or BUSL. Nothing in the checks looks at licences yet, and a change that
adds one worth arguing about should say so in the pull request.

**What will not be taken.** Some designs were considered and removed on purpose, with the
reasons written down. Bringing one back is refused however well it is written: proofs of
work, an outside chain used as a clock, agreement protocols, founder broadcast or
revocation keys, hardware fingerprinting, automatic updates, remote verification of
somebody else's client, a prompt offering heresy, grace counters, fallback verifiers,
encrypted chat, and bucketed statistics passed between nodes. Zero-knowledge proofs are
not on this list in the same way — see the top of this page for what would have to change.

You will be told what was decided and why, eventually. The person who has been writing this
has set the work down for a while and said so in [a letter](LETTER.md), so an issue or a
proposal may sit for a long time before anybody answers it. That is not a judgement on the
work in it. Nothing in this document waits for that answer either: the direction is written
here so that it can be followed without asking.

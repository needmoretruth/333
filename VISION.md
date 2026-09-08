# The Vision

What follows is not a plan. A plan is a promise about dates, and there are no dates here.

It is what has been seen: the shape 333 takes when it is finished, written down while it
is still unfinished, so that the distance between the two is something anybody can
measure. Each part of it is a prophecy in the only sense this faith allows — a thing that
is true of the end and not yet true of now.

**Nothing here comes to pass on its own.** No prophecy in this document will be fulfilled
by waiting for it. Each one is a piece of work, and it will be done by one of us, at a
keyboard, or it will not be done. That is the whole of the arrangement. If you write Rust
and you have read this far, some of what is written below is addressed to you personally.

---

## There is one client

333 has one client and it is this one. It runs alone.

That is not a boast about quality. It is a condition. A node must not need a second
program to be a node — no daemon beside it, no service to sign up to, nothing to install
first. It carries its own storage, its own way onto Tor, its own hours. You take one file
and it is a node, and it stays a node on a machine that has nothing else on it and no way
to get anything else.

There is one thing it does not carry, and it is deliberate: the program that speaks an
obfuscated bridge, for people in places where Tor itself is blocked. That one chases a
target which moves, and a copy frozen inside this would be the wrong copy within a year
while looking like the right one. Everything else is here.

So no prophecy below asks for a second implementation, a companion tool or a helper
process. Where one seems necessary, the prophecy is wrong and the vision is that the
client learns to do it alone.

---

## The first order: that anybody can be reached

A node nobody can reach is not asked, is not witnessed, and cannot hand the file to
whoever comes next. Most of the people who will ever run this sit behind a router that
was never told to let anybody in. Every prophecy in this order is about one of them.

### Every door opens

*It is seen that a node behind any router is reachable, and that nobody had to be told
how.*

The client already asks the router in front of it to send its port through, over
UPnP-IGD, and a great many home routers answer. Not all. The ones that speak NAT-PMP or
PCP instead are asked nothing today.

Past that lies the harder half: two machines, each behind a router, neither able to
accept a connection, reaching each other anyway because both dial out at the same moment
and something arranged the timing. That is well understood elsewhere and nothing here
does it yet.

*Fulfilled when:* a node behind a router that does not speak UPnP is reachable without
its owner opening anything, and two nodes behind two routers reach each other with the
meeting point doing nothing but the introduction.

### The meeting place grows smaller

*It is seen that the fixed address is no longer needed, and that nothing replaced it.*

One address that everybody looks at is one thing that can be taken away, and this design
wanted none. We said in the open that we mean to depend on it less. Saying it is not
doing it.

The shape of the answer is nodes learning where each other are from the nodes they have
already met, rather than from one door everybody knocks on. Whatever is built must not
become a second thing that can be taken away, and must not publish an address its owner
never meant to publish.

*Fulfilled when:* a node that has met one other node keeps finding new ones with the
meeting point switched off, and the client can say honestly where each thing it knows
came from.

### The smallest machine is enough

*It is seen that the weakest of us sets the difficulty, and that the difficulty is low.*

The Light build is for a machine with nothing to spare, and it now carries Tor, which is
most of its size. That was the right trade — a Light node nobody can reach is broken, not
small — and it means the small edition is no longer small.

*Fulfilled when:* there is a build that fits the smallest machine anybody actually keeps
a vigil on and is still reachable from outside it, and what it gives up is written down.

---

## The second order: that the client can be believed

Nothing here asks to be trusted. Every statement carries the signature of whoever made
it. That holds only while the program in front of a person is doing what it says, and
while what it says is true.

### It has been run

*It is seen that this has been kept on every kind of machine, and that somebody was there
to say so.*

The files for macOS, for the Raspberry Pi, and for Windows are built on every release and
have been run by nobody. Not once. They may work perfectly; nobody knows.

This is the smallest thing on this page and it may be the most useful. Take the file for
your machine, keep the vigil on it, and say what happened — including that it worked,
which is the report nobody thinks to send.

*Fulfilled when:* a node has kept a full epoch on each of them and somebody has said so.

### It says what happened

*It is seen that no line the client prints is more certain than the thing it describes.*

The client talks to people, and some of what it says is wrong in small ways. When the
meeting point asks a node to wait a minute before speaking again, the node reports it as
a refusal, which it is not. There are others.

Every one of these is small and every one of them teaches a person to stop reading. This
is where to start if you want to start.

*Fulfilled when:* the line a person reads matches what actually happened, everywhere.

### It knows itself

*It is seen that the client can be recognised, and that it does not claim to be more than
it is.*

A client carrying its own hash cannot hash the place the hash is written. What is needed
is a boundary: what the hash covers, what it deliberately does not, and why there and not
somewhere else — narrow enough to check by hand, wide enough to be worth having.

This one has a deadline that nothing else here has. After 3.3.3 the bytes are fixed and
the mistakes in them are permanent.

*Fulfilled when:* a person with the source and a hashing program arrives at the same
number the client reports, following only what is written down.

---

## The third order: that it outlives us

Nineteen thousand six hundred and eighty three years is the number in the doctrine. It is
a long time to be relying on somebody remembering to restart something.

### The vigil does not break

*It is seen that a node kept running through the night, and that nobody stayed awake for
it.*

A node that stops when a laptop closes is absent from every epoch its owner slept
through. There is a service file and a launch agent in `packaging/`, and they are a
beginning: no package for any distribution, nothing for Windows, and nothing that tells a
person their node stopped three days ago.

*Fulfilled when:* somebody installs this the way they install anything else on their
system, it survives a reboot without them reading anything, and it tells them when it did
not.

### A name can be carried

*It is seen that a believer moved house and remained the same believer.*

A node's name comes from its key, its standing is what others signed about that key, and
its unseen address is a second key beside it. All of that lives in one directory. Lose
the directory and you are somebody new, with no window and no history, and nothing warns
you before it happens.

*Fulfilled when:* a person can carry a node to another machine, deliberately and safely,
and the client says plainly what would be lost if they did it the wrong way.

### It speaks to whoever comes

*It is seen that it was read in the reader's own language.*

The client speaks English and nothing else. That was deliberate — one language done
properly before several done badly — and it was never meant to be permanent. Most of the
people this could reach do not read English comfortably.

*Fulfilled when:* the words a person reads are separate from the code that decides what
to say, and one other language is complete enough to keep a vigil in without falling
back.

---

## The prophecy is not a cage

Everything above is what was seen from where we are standing. Standing somewhere else,
you may see further.

If there is a way that is plainly better — simpler, smaller, harder to break, reaching
people these do not reach — then that is the way, and this document is what changes. A
vision defended after it has been beaten by a better one is not a vision any more. It is
a habit.

What does not change is the shape underneath: no authority, nothing to appeal to, one
client that runs alone, and nothing claimed that cannot be checked. A proposal that gets
somewhere better while keeping those is welcome even if it deletes half of this page. A
proposal that gets somewhere better by giving one of them up is not better. It is a
different thing wearing this name.

Say which prophecy you are answering, or say that it is none of them.

---

## A holy pull request

The rules below are the ones the code already lives by. They are not a hazing. Each is
here because something went wrong without it, and the checks refuse a change that breaks
them before a person ever looks at it.

**Sign your work.** Every commit carries a `Signed-off-by` line, which `git commit -s`
adds for you. That line certifies the [Developer Certificate of Origin](DCO): that the
work is yours to give, or that you have the right to pass it on under this licence. It is
the Linux kernel's arrangement, and it exists so that nobody years from now has to wonder
whether the code in front of them can be used. There is no copyright assignment and
nothing to sign in ink. What you write stays yours.

**Rust, and only Rust.** No build scripts in other languages and no tooling in other
languages. `unsafe` is forbidden across the workspace. `unwrap`, `expect` and indexing
warn: leave one in and the line above it has to say why it is safe.

**One file, one responsibility, four hundred lines.** Counted to the tests, which live
beside the code they test. A function fits on one screen.

**Say why, not what.** Everything public carries a doc comment giving the reason rather
than restating the code. A comment that repeats the line below it is one more thing to
keep true.

**Tests that could fail.** A test that passes before your change as well as after is not
testing your change. Show the failure first. Anything frozen — bytes, constants, domain
strings — is pinned against a literal, because a value compared only against itself is
not tested at all.

**Dependencies:** MIT, Apache-2.0, BSD, ISC, Unlicense or CC0, and nothing else. Prefer a
crate that exists over writing it again, and never write cryptography, TLS, password
hashing or random numbers by hand.

**Ask before you change behaviour.** Numbers, thresholds and orders of operation that
look arbitrary usually are not. The doc comments carry most of the reasons. A change to
how the protocol behaves, rather than how it is written, is worth asking about before it
is worth writing.

## What will not be taken

Some things were considered and removed on purpose, and a pull request bringing one back
will be refused however well it is written: proofs of knowledge, proof of work, an
outside chain used as a clock, agreement protocols of any kind, hardware fingerprinting,
automatic updates, fallback verifiers, encrypted chat, and bucketed statistics passed
between nodes.

The reason is one reason. There is no authority here and nothing may create one. Anything
that decides who is right, or that everybody must accept in order to take part, is a
place where somebody can be captured. If the design ever seems to need a vote, the design
is wrong at that point, and the change worth writing is the one that removes the need.

We also do not verify what cannot be verified. Nothing claims a node is running genuine
code and nothing claims anybody is honest. What is verified is what was done.

## Where to send it

Pull requests on `github.com/needmoretruth/333`, against `main`.

You will be told what was decided and why. Not quickly, necessarily. Every line here has
to be true in nineteen thousand years, which is a strange thing to be in a hurry about.

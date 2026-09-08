# The Works

Faith that builds nothing is a feeling. This is the part that is not built yet, and it
is written down here so that anybody who can build it knows where to start.

It is addressed to whoever among us writes Rust and writes it well. Not to everybody:
most of what keeps 333 alive is answering when you are asked, and that costs no code at
all. But there is work that only somebody at a keyboard can do, and leaving it undone is
also a choice.

**What is asked for is a pull request.** Not a proposal, not an issue explaining what
somebody else should write. Working code, with tests, against the thing that is actually
here. That is the offering.

---

## The first order: that anybody can be reached

A node nobody can reach is not counted for, cannot be asked, and cannot hand the file to
whoever comes next. Most of the people who will ever run this are behind a router that
was never told to let anybody in. Everything in this order exists so that fewer of them
are shut out.

### The Opening of Every Door

A node already asks the router in front of it to send its port through, over UPnP-IGD.
That covers a great many home routers and not all of them. The ones that speak NAT-PMP
or PCP instead — Apple's, and others — are asked nothing today and go unforwarded.

Beyond that lies the harder half. Two machines, each behind a router, each unable to
accept a connection: they can still meet in the middle if both dial out at the same
moment and something arranges the timing. That is hole punching, it is well understood,
and nothing here does it.

*Done when:* a node behind an Apple-shaped router is reachable without anybody opening
anything, and the client says which way in it found. Then: two nodes behind two routers
reaching each other, with the meeting point doing nothing but the introduction.

### The Lessening of the Meeting Place

One fixed address is a single point that can be taken away, and this design wanted none.
We say in the open that we mean to depend on it less, and saying it is not doing it.

The shape of the answer is a way for nodes to learn where each other are without a fixed
address in the middle: whereabouts carried between the nodes that already know them,
rather than left at one door everybody knocks on. Whatever is built must not become a
second thing that can be taken away, and must not publish an address its owner did not
mean to publish.

*Done when:* a node that has met one other node can go on finding new ones with the
meeting point switched off, and the client can say honestly how much of its knowledge
came from where.

### The Smaller Edition

The Light build exists for a machine with nothing to spare, and it now carries Tor,
which is most of its size. That was the right trade — a Light node nobody can reach is a
broken node, not a small one — but it means the small edition is no longer small.

*Done when:* there is a build that fits on the smallest machine anybody actually runs
this on and is still reachable from outside it, and the README says what it gives up.

---

## The second order: that anybody can check

Nothing here asks to be trusted. Every statement carries the signature of whoever made
it, and the whole design turns on a stranger being able to check for themselves. That
only holds while checking is something a stranger can actually do.

### The Witness of Strange Machines

The files for macOS, for the Raspberry Pi, and for Windows are built on every release
and have been run by nobody. Not once. They may work perfectly; nobody knows.

This is the smallest offering on this page and it may be the most useful. Take the file
for your machine, run it, and say what happened — including that it worked, which is the
report nobody thinks to send.

*Done when:* somebody has kept the vigil for a full epoch on each of them and said so.

### The Second Witness

A node's record is signed, so it can be checked by anything that can read the bytes. In
practice there is one program that reads them, and it is this one. A record that can only
be checked by the client that wrote it is a record resting on the client's word.

This one needs something first: the specification is not published. What is public is the
code, the doc comments in it, and the three documents of doctrine, and a second reader
written by reading our source tests our source against itself. Publishing it is part of
this work and is being decided.

*Done when:* something other than this client can take a node's chain and the statements
about it and say whether they hold, written against a written-down format rather than
against our implementation of it.

### The Plain Word

The client says things to people, and some of what it says is wrong in small ways. When
the meeting point asks a node to wait a minute before speaking again, the node reports it
as a refusal, which it is not. There are others.

*Done when:* the line a person reads matches what actually happened. This is where to
start if you want to start.

---

## The third order: that it outlives us

Some of this is easier now than it will ever be again. After 3.3.3 the bytes are fixed
and the mistakes in them are permanent, so the work that touches what gets frozen has a
deadline that nothing else here has.

### The Sealing

A client that carries its own hash cannot hash the place the hash is written. The
specification says so and leaves the exact answer open, and marks it as a thing that
must be settled before 3.3.3 because it cannot be changed afterwards.

What is needed is a definition narrow enough to be checked by hand and wide enough to be
worth having: what the hash covers, what it deliberately does not, and why that boundary
and not another.

*Done when:* a person with the source and a hashing program can arrive at the same number
the client reports, following only what is written down.

### The Tongues

The client speaks English and nothing else. That was deliberate — one language done
properly before several done badly — and it is not meant to be permanent. Most of the
people this could reach do not read English comfortably.

*Done when:* the strings a person reads are separate from the code that decides what to
say, and one other language is complete enough to run a node in without falling back.

### The Keeping of the Vigil

A node that stops when you close your laptop is a node that is absent from every epoch
you were asleep for. There are a service file and a launch agent in `packaging/`, and
they are the beginning of it: no package for any distribution, nothing for Windows, and
nothing that tells a person their node stopped three days ago.

*Done when:* somebody can install this the way they install anything else on their
system, and it survives a reboot without them reading anything.

---

## A holy pull request

The rules below are the ones the code already lives by. They are not a hazing. Every one
of them is here because something went wrong without it, and the checks refuse a change
that breaks them before a person ever looks at it.

**Sign your work.** Every commit carries a `Signed-off-by` line, which `git commit -s`
adds for you. That line means you are certifying the
[Developer Certificate of Origin](DCO) — that the work is yours to give, or that you have
the right to pass it on under this licence. It is the Linux kernel's arrangement and it
exists so that nobody, years from now, has to wonder whether the code in front of them
can be used. There is no copyright assignment and nothing to sign in ink. What you write
stays yours.

**Rust, and only Rust.** No build scripts in other languages, no tooling in other
languages. `unsafe` is forbidden across the workspace. `unwrap`, `expect` and indexing
warn: leave one in and the line above it has to say why it is safe.

**One file, one responsibility, four hundred lines.** Counted to the tests, which live
beside the code they test. A function fits on a screen.

**Say why, not what.** Everything public carries a doc comment, and the comment explains
the reason rather than restating the code. A comment that says what the next line says is
one more thing to keep true.

**Tests that could fail.** A test that passes before your change as well as after is not
testing your change. Show the failure first. Anything frozen — bytes, constants, domain
strings — is pinned against a literal, because a value compared only against itself is
not tested at all.

**Dependencies:** MIT, Apache-2.0, BSD, ISC, Unlicense or CC0. Nothing else. Prefer a
crate that exists over writing it again, and never write cryptography, TLS, password
hashing or random numbers by hand.

**Ask before you change behaviour.** The specification this is built from is not published
yet, so the things that look arbitrary in the code often are not: a number, a threshold or
an order of operations may be carrying a reason that is written down somewhere you cannot
see. The doc comments carry most of those reasons and the rest can be asked for. A change
to how the protocol behaves, as opposed to how it is written, is worth an issue before it
is worth a branch.

## What will not be taken

Some things were considered and removed on purpose, and a pull request bringing one back
will be refused however good it is: proofs of knowledge, proof of work, an outside chain
as a clock, agreement protocols of any kind, hardware fingerprinting, automatic updates,
fallback verifiers, encrypted chat, and bucketed statistics passed between nodes.

The reason is one reason. There is no authority here and nothing may create one. Anything
that decides who is right, or that everybody must accept in order to take part, is a place
where somebody can be captured. If the design ever seems to need a vote, the design is
wrong at that point, and the pull request worth writing is the one that removes the need.

We also do not verify what cannot be verified. No claim that a node is running genuine
code, no claim about anybody's honesty. What is verified is what was done, and nothing
else has ever been offered.

## Where to send it

Pull requests on `github.com/needmoretruth/333`, against `main`. Say which of the works
you are answering, or say that it is none of them — this page is not the whole of what
is worth doing, and somebody who has read the code will see things nobody listed.

You will be told what was decided and why. Not quickly, necessarily. Every line here has
to be true in nineteen thousand years, and that is a strange thing to be in a hurry about.

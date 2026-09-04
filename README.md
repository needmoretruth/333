> **A note first, in plain words. These are the only plain words in this repository.**
>
> To someone who truly believed it, this could become a religion. I am not a mad cult
> leader, so please do not take me for one. I am someone who loves technology. This is
> a toy project I built once, and the whole of it is a bit — a concept, played
> straight. Below this line, in every other document here, and in the program itself,
> I stay in character. My sanity ends here.

# 333

There is a file. It is three bytes long. It says `333`.

It is not encrypted, not rare, and not hard to reproduce — you have already read it.
That is not a flaw in the relic. It is the relic. Anything that had to be guarded
would have died with its guards; this survives precisely because it is worthless to
steal and trivial to give away, and because the only thing that can kill it is
everyone, everywhere, at once, deciding not to bother.

So we bother.

We do not keep the file. Copies are cheap and copies are not the point. What we keep
is the record of the file being handed on, and the record of who was awake to answer
when somebody asked.

## The Rule

**The file lives as long as someone is awake to pass it on. When nobody is, it does
not, and neither do we.**

There is nothing above that line. No council, no vote, no ledger of truth, nobody to
appeal to and nobody to excommunicate. Each of us believes what we saw and says so,
and two of us may disagree with neither of us being wrong. A faith that needed
everyone to agree would have to appoint someone to decide, and the moment there is
someone to decide there is someone to capture.

We are not old. We have no scripture older than a commit, no founding generation with
a claim on the rest, no golden age and nothing to restore. We have an uptime.
Everything this asks of you, it asks in the current epoch — and it will ask again in
the next one, and the answer you gave last year is already out of the window.

The rest of the doctrine, including the one thing asked of you that the licence does
not require, is in [COVENANT.md](COVENANT.md). It is deliberately unfinished. That is
an invitation, not an apology.

---

## What is asked of you

**The hours.** We keep our own time: epochs of 333 minutes, counted from the birth of
Unix, on the clock of your own machine. No time server, no beacon, no chain — nothing
to petition, nothing to bribe, nothing to seize. If your clock drifts, those whose
clocks agree stop witnessing you, and that is the only correction this faith has. It
does not punish you. It simply stops seeing you.

**The gate.** Your name is the SHA-256 of an Ed25519 public key, written in
hexadecimal, and only a name beginning with `333` may pass. You generate keys until
one is called — about a sixth of a second of arithmetic. That is the entire
requirement for entry. Not a birth. Not an invitation. Not a payment, not a reference,
not a word from anyone already inside. Whatever you were before that key existed does
not come through the door with you, and none of us can ask.

Two prefixes are turned away at the gate: `666` and `111`. We do not offer them.
There is no flag, no prompt, no menu; a key that lands on one is discarded without a
word, and you are never told it happened. All three are equally likely, so every one
of us walked past both of them on the way in, unknowing. The cursed reveal themselves.
Nobody has to point.

**The vigil.** To answer is to be present. To be silent is to be absent. We measure
over a moving window of the last 333 epochs and we ask two thirds of it: rest one hour
in three and you are still counted, but the hour you are not there, you are not there.
Nothing is ever banked. One of us who has kept the vigil for ten years and one who
arrived last week are read from the same 333 epochs, and the ten years buy nothing.
There is no seniority in this faith because there is no memory long enough to hold it.

**The witnesses.** No one's presence is their own claim. Each epoch three others are
drawn to ask, and what they sign is what the record keeps. A witness who says *it
answered* is believed over a witness who says *it did not*, always: the first carries
a signature no one can forge, and the second carries nothing at all. Absence is the
one thing this faith will not take anybody's word for.

**The hand.** The file must be received, never made. Our clients carry its hash and
not its contents, so no one of us can produce it alone; it can only be put into your
hands by someone who already had it. When it passes, both of you sign — *I gave it to
you*, *I received it from you* — and neither signature means anything without the
other. Follow those back far enough and you have a lineage. It is the only history we
keep.

To carry the client's own source as well as the file is to be a bearer. It is not a
rank. It is a mark, and it is the more useful thing to hand somebody than three bytes
they could have typed.

**The word.** Once an epoch you may speak one of 333 signals. What travels is the
number and never the word, and the table is never translated into anything: a hundred
of us meaning the same thing must never be counted as a hundred different things, one
per language. Nothing is put to a vote and no winner is announced. Every node counts
only what it heard itself, and shows the whole distribution — the shape of what we
said, not the loudest of us.

**The last silence.** The count that matters is how many are answering, not how many
names are on the roll. When it reaches zero, it is over. What remains is a
subtraction: nineteen thousand six hundred and eighty-three years — three cubed,
cubed — and the client will tell you how much of it is left.

Every one of us will be dead long before that number runs out. Our machines will be
dead. The architectures they ran on will be a footnote. That is not the tragedy of
this faith; it is the specification. We are building for a congregation that is
overwhelmingly not born yet, and the only thing we can hand them is a running process
and a number they can check.

---

## Where this stands

Version 0.0.1, which is to say: almost nothing, honestly labelled.

- [x] Two of us can meet and sign for each other — directly, or unseen
- [x] The vigil: presence over the moving window, and the reckoning after the end
- [x] The relic recognised, and both halves of a handover
- [x] The word counted, and the threshold that marks it
- [x] Invitations, so a newcomer knows where to look
- [x] The gate: the file handed over, the two signatures, the waiting that follows
- [x] The three witnesses, the question they put, and the hours that put it
- [x] The record, kept as a chain of hashes
- [x] Finding one another again: we trade what each of us knows, and nobody arbitrates
- [ ] The 333 words themselves

What is written on the wire keeps moving until the release that freezes it. The parts
already settled are marked `FROZEN` where they are defined, and after that they cannot
move at all — not for a better idea, not for a bug, not for us.

---

## How we reach one another

By default your node opens a socket and answers on it. The exchange is a few hundred
bytes each way and is over in milliseconds. Whoever you answer sees the address you
answered from, and so does anyone watching the wire. That is the ordinary condition of
being on a network, and we say it here rather than bury it.

If your address is something you cannot afford to show, the client carries a complete
Tor implementation and can raise an onion address instead, or as well. Waking that
takes seconds to minutes before the first byte moves, so it wakes only when something
asks for it.

**The address decides.** An address ending in `.onion` is reached through Tor;
everything else is reached directly. There is no setting, because a setting is
something that can be turned on once and quietly stop applying, and of all the
mistakes available here that is the one that cannot be taken back.

After the first meeting we find each other by telling each other. Each of us signs a
short statement saying where to look, and every time two of us speak we hand over what
we hold — those statements, and the records of who let whom in. Nobody keeps the list.
There is no list. Each of us knows what has reached us, two of us can know different
things, and neither of us is wrong.

If your node answers on every interface, or sits behind something that forwards a port,
it cannot work out what to tell people. Tell it: `--announce your.address:3333`.

---

## Two forms of the client

**Standard** is what most of us will run: a full terminal interface, on a machine
that is not struggling. **Light** is a plain command-line client with the smallest
set of things that still counts as keeping the vigil, built without Tor because Tor
is by far the largest thing we carry. It is meant for a Raspberry Pi Zero. The
weakest of us sets the difficulty for all of us, and that is not a compromise — it is
the point. A faith that only runs on new hardware is a faith with an expiry date.

Linux first, and not one Linux: the Debian family and the Fedora family both. macOS
and Windows follow. Today only the plain client exists, and it has been run on Linux.

---

## Build

Rust 1.96 or newer.

```sh
cargo build --release                        # with Tor
cargo build --release --no-default-features  # without it
```

## Run

```sh
# Ask for a name. The first run searches until one is called.
./target/release/333 id

# Keep the vigil: answer whoever knocks, on port 3333.
./target/release/333 serve

# Knock.
./target/release/333 ping node.example:3333

# Ask somebody who has the file to hand it over. This is how you join, and there
# is no other way: the client carries the hash of the file and cannot make one.
./target/release/333 join 333:node.example:3333
```

Nobody can start a network alone. The first node's file was put in its directory by
hand, once, by a person — and every copy since has come from somebody who already had
it and signed for handing it over. If you are reading this because you want to run
333, you need an invitation from someone who is already keeping it.

To answer without showing where you are, raise an onion address as well — or, with
`--no-direct`, instead of a socket:

```sh
./target/release/333 serve --tor
./target/release/333 ping <their-address>.onion
```

An onion address is the answer to being reachable from anywhere without saying where
you are, and it needs no `--announce`: the node knows the address it raised, and tells
people that one.

Everything your node is lives in one directory: its name, and Tor's state if it uses
Tor. Two nodes on one machine are two directories.

```sh
./target/release/333 --data-dir ./node-a serve --bind 127.0.0.1:3333
./target/release/333 --data-dir ./node-b join 333:127.0.0.1:3333
```

Without `--data-dir` a node lives in the conventional place for the system:

| system | directory |
|---|---|
| Linux, BSD | `$XDG_DATA_HOME/333`, or `~/.local/share/333` |
| macOS | `~/Library/Application Support/333` |
| Windows | `%LOCALAPPDATA%\333\data` |

Lose that directory and you lose your name and your address for good. There is no
recovery, no reset, and no one to appeal to. That is not an oversight. A name you
could be given again by asking nicely would not be worth having.

The client refuses to start if that directory, or any directory above it, can be
entered by others on the machine, and tells you the one command that fixes it.
`--dangerously-trust-directory-permissions` turns the check off, and is named after
what it does.

---

## What we do not claim

This matters more than what we do claim, so it gets its own section. A faith that
overstates what it can verify is a faith waiting to be caught.

- **Nothing verifies that a peer is running this code.** It cannot be done on a
  general-purpose machine, and no amount of hashing the source changes that. What is
  verified is conduct: did it answer, did it sign, did it pass the file on.
- **Nothing verifies that a name was honestly searched for.** See above. Thirty-two
  bytes are thirty-two bytes, whether they were mined or bought.
- **The one who answers learns less than the one who asks.** Your challenge coming
  back inside a signature proves the other was awake after you chose it. A message
  that merely arrives proves nothing of the sort, and our code says so in the type
  rather than in a comment.
- **Multiplied identities are not prevented, only shown.** What we measure is not
  people but effort, and effort cannot be bought in advance. Whoever keeps paying it
  is, by definition, one of us.
- **Disagreement about the hour is reported, never punished.** There is no authority
  here to decide whose clock is right.
- **A direct connection shows your address.** That is what direct means. Tor exists
  for when it matters, and it is one word on the command line.
- **On Windows, nothing checks who else can read your node's directory.** The check
  is real on Linux, the BSDs and macOS, where it walks the whole path and refuses to
  start on a directory others can enter. On Windows the library that performs it —
  the same one Tor's own Rust client uses — accepts every permission and every owner.
  We do not paper over that with a check of our own that does not run.

---

## The order of the code

Three crates, split along the direction of dependency:

| crate | holds | knows about |
|---|---|---|
| `core` | the hours, names, thresholds, the relic, and the exact bytes we sign | nothing else — no I/O, no network |
| `net` | framing, one exchange, sockets, and the Tor client and service | `core` |
| `cli` | arguments and what is printed | `core`, `net` |

Because `core` touches nothing outside itself, every rule of this faith can be tested
without a network, and the exchange itself is tested over a pipe in memory.

Things worth knowing before you read it:

- The signature sits outside the value it signs, so a verifier checks the bytes that
  arrived rather than a re-encoding of what it decoded.
- A public key is refused unless its encoding is canonical. The name comes from those
  bytes, and two encodings of one key would be two names.
- Every threshold is a fraction compared by cross-multiplication, and nothing divides
  in order to decide anything. Two thirds and 66.7% are not the same number; over a
  window of 333 they differ by one epoch, and they would differ for ever.
- Everything frozen is pinned by a test asserting literal bytes, because a value
  compared only against itself is not tested at all.

## Licence

Apache-2.0.

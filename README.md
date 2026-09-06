> **A note first, in plain words. These are the only plain words in this repository.**
>
> To someone who truly believed it, this could become a religion. I am not a mad cult
> leader, so please do not take me for one. I am someone who loves technology. This is
> a toy project I built once, and the whole of it is a bit — a concept, played
> straight. Below this line, in every other document here, and in the program itself,
> I stay in character. My sanity ends here.

# 333

**[the333.dev](https://the333.dev)** is the one address written into the client. It says
what this is, it carries the Law, and it is where two nodes on two different networks find
each other: it holds the signed statements nodes make about where they can be reached, keeps
each one for two epochs, and verifies none of them, because the client that reads them does
that. It hands out no file and it is not a node. If it went away tomorrow, every node that
had already met somebody would carry on unchanged.

**[The latest release](https://github.com/needmoretruth/333/releases/latest)** has a binary
for Linux, for the Raspberry Pi down to the Zero, for both kinds of Mac and for Windows.
[Install](#install) says which one is yours and what to do with it.

---

333. Not short for anything, and nothing goes in front of it.

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

**333 lives as long as somebody is awake to pass it on. When nobody is, it does not,
and neither do we.**

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
to petition, nothing to bribe, nothing to seize.

If your clock drifts, nobody corrects it. There is nobody here who could: no majority
you can see from one machine, no authority to appeal to, and no way to tell a wrong
clock from a right one. Your neighbours will tell you they disagree and will do nothing
about it. What each of us does instead is smaller and is the only thing that can be
done alone — we answer for the hour we are in, and for no other. The hours that have
not come are not ours to be awake in, and nobody can be made to sign that they were.

**The gate.** Your name is the SHA-256 of an Ed25519 public key, written in
hexadecimal, and only a name beginning with `333` may pass. You generate keys until
one is called — about a sixth of a second of arithmetic. It costs nothing and it is
refused to nobody: not a birth, not a payment, not a reference, and whatever you were
before that key existed does not come through the door with you. None of us can ask.

And it is not enough, because the door does not open from your side. The file has to
be put into your hands by someone who already has it, and no client can make one. So
the cheapest thing in this faith is being allowed in, and the one thing that cannot be
bought is somebody bothering to let you.

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

When you are called you have three minutes of the three hundred and thirty-three to
sign your name back, and you are not told which three. Three minutes because a circuit
to a tired machine in a cupboard takes seconds to open, and the weakest of us sets the
difficulty for all of us.

The window is also the only mercy in the design. Nothing is served out and nothing is
held against you for ever: it moves every epoch, and each hour you answer pushes an
older absence past its edge. Your own record still holds every hour you missed. The
count does not reach back for them.

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
rank. It is a mark, and it is the more useful thing to hand somebody than a file
they could have written themselves.

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
- [x] The word: one an epoch, signed, counted, and the whole shape of it shown
- [x] Invitations, so a newcomer knows where to look
- [x] The gate: the file handed over, the two signatures, the waiting that follows
- [x] The three witnesses, the question they put, and the hours that put it
- [x] The record, kept as a chain of hashes
- [x] Finding one another again: we trade what each of us knows, and nobody arbitrates
- [ ] The 333 words themselves — a table nobody has written, and none of us alone will

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

On one local network there is nothing to tell. A node that answers on a socket says
on that network that something here speaks 333, and on which port — not its name, and
nothing anybody could carry back to a name in the record; what goes out is what a port
scan of the same network would find anyway. Two of us in one house find each other
that way with nobody typing an invitation, and knock on each other the moment they do
rather than at the next turn of the hours. `--no-mdns` keeps your node off it, and a
node that answers only through Tor never does it at all — announcing on the network
you are sitting on is the one thing an onion address exists to prevent.

And when neither of those has happened — nobody handed you an invitation, nobody is on
your network — there is one fixed address both of us are already looking at:
**the333.dev**. Your node leaves the same signed statement it already writes about
where it can be reached, and reads the ones other people left there. Every one of them
is verified here, by the signature it carries, so that address cannot invent one of
us, cannot forge anybody's whereabouts and cannot vouch for anyone; it holds a
statement for two epochs and then forgets it. `--no-meet` keeps your node away from it
altogether, and `--meet somewhere.else` points it at a different one.

Whoever runs that address learns which address your node speaks from. That is the
whole of what it costs and it is not nothing, so a node answering only through Tor
reads it and never writes to it: publishing an onion address from the machine behind
it would hand over the one fact the onion address exists to withhold. A node that
already answers on a socket spent that cost the moment it answered anybody.

**We mean to depend on this less, and we would rather say so now than quietly later.**
A fixed address is a single point that can be taken away, and this design wanted none.
The ways around it — whereabouts carried in a distributed hash table, two machines
behind two routers punching through to each other — are today more code, more failure
and more ways to publish an address its owner never meant to publish than we are
willing to ship. That is a limit of what is built, not a claim about what is right,
and as those get built this gets smaller. Invitations and the local network are not
being replaced by it. They are the first two ways of meeting anybody, and this is the
third.

If your node answers on every interface, or sits behind something that forwards a port,
it cannot work out what to tell people. Tell it: `--announce your.address:3333`. If you
do not know what to put there, the meeting point will say which address it saw your
node arrive from, which is the half you cannot work out from where you are sitting;
whether anything arriving there reaches your machine is a question about your router,
and only you can answer it.

---

## Two forms of the client

**Standard** is what most of us will run: a full terminal interface, on a machine
that is not struggling. **Light** is a plain command-line client with the smallest
set of things that still counts as keeping the vigil, built without Tor because Tor
is by far the largest thing we carry. It is meant for a Raspberry Pi Zero. The
weakest of us sets the difficulty for all of us, and that is not a compromise — it is
the point. A faith that only runs on new hardware is a faith with an expiry date.

Linux first, and not one Linux: the Debian family and the Fedora family both. macOS
and Windows follow. Today both forms exist and have been run on Linux.

---

## Install

### Download it

Every release carries a binary for each system it has been built for. Take the one that
matches yours, make it executable, and it is installed.

```sh
curl -LO https://github.com/needmoretruth/333/releases/latest/download/333-x86_64-linux
chmod +x 333-x86_64-linux
mkdir -p ~/.local/bin && mv 333-x86_64-linux ~/.local/bin/333
333 id
```

| your machine | the file |
|---|---|
| Linux, ordinary desktop or server | `333-x86_64-linux` |
| Linux, and you want the small one | `333-light-x86_64-linux` |
| Raspberry Pi 3, 4, 5 on 64-bit | `333-light-aarch64-linux` |
| Raspberry Pi Zero, or anything ARMv6 | `333-light-armv6-linux` |
| Mac, Apple silicon | `333-aarch64-macos` |
| Mac, Intel | `333-x86_64-macos` |
| Windows | `333-light-x86_64-windows.exe` |

Standard is around 17 MB and has the screen and Tor in it. Light is around 5 MB, links
nothing but the C library, and is the smallest thing that still counts as keeping the
vigil. On macOS the system will want to be told the binary is not malicious, which it says
in its own words the first time you run it.

Nothing is signed by a developer certificate and nothing goes through an app store. If that
matters to you, or if there is no file for your machine, build it.

### Or build it

**Rust 1.96 or newer.** What a distribution packages is usually older than that, so take it
from [rustup.rs](https://rustup.rs):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then whatever your system needs to compile it. Standard carries Tor, and Tor brings TLS and
SQLite with it. Light carries neither and needs only a C compiler.

**Ubuntu, Debian, Mint, Pop, and the rest of that family**

```sh
sudo apt install -y git build-essential pkg-config libssl-dev libsqlite3-dev
```

**Fedora, RHEL, Rocky, Alma**

```sh
sudo dnf install -y git gcc pkgconf-pkg-config openssl-devel sqlite-devel
```

**Arch, Manjaro, EndeavourOS**

```sh
sudo pacman -S --needed git base-devel openssl sqlite
```

**macOS**

```sh
xcode-select --install
```

TLS and SQLite come with the system there, so that is the whole of it.

**Windows**

The rustup installer offers to fetch the Microsoft C++ build tools; accept, and you have
everything the Light form needs.

Then, on any of them:

```sh
git clone https://github.com/needmoretruth/333
cd 333
cargo build --release                        # Standard: the screen, and Tor
cargo build --release --no-default-features  # Light: neither
```

Standard comes out around 17 MB and links your system TLS and SQLite, both of which arrive
with Tor. Light comes out around 5 MB and links nothing but the C library, which is why it is
the form for a machine that is struggling and the form most likely to build somewhere nobody
has tried yet.

**What has actually been built and run.** Linux on x86-64, both forms: built, started, given
a name, handed the file from one node to another over a socket, and asked for its standing
afterwards. That is what the rest of this file describes. macOS and Windows have
not been built yet. On Windows, expect Light to be the form that works today, because the Tor
stack inside Standard wants a system SQLite that Windows does not ship. When either of them
fails, that is worth an issue rather than a shrug.

## Run

```sh
# Ask for a name. The first run searches until one is called.
./target/release/333 id

# Keep the vigil: answer whoever knocks, on port 3333. On a terminal this opens
# the screen; anywhere else — a pipe, a service manager's log — it says its lines.
# `--plain` asks for the lines on a terminal too.
./target/release/333 serve

# Knock.
./target/release/333 ping node.example:3333

# Ask somebody who has the file to hand it over. Write the file yourself and you hold
# a file: you are one of us from the moment somebody gives it to you and the two of
# you sign for it.
./target/release/333 join 333:node.example:3333

# What this node has seen: how many of us are answering, where you stand over
# the window, and how much of the silence is left if it has begun.
./target/release/333 status
```

The screen shows what this node is doing while it does it: how many of us are
answering, where you stand over the window, the shape of what everybody said this
epoch, and how long is left of it. `q` leaves the vigil. `s` says one of the 333.
Nothing on it is anybody else's number — it is what this one machine has seen, and
the machine beside you is showing something else.

Nobody can start a network alone. The first node's file was put in its directory by
hand, once, by a person — and every copy since has come from somebody who already had
it and signed for handing it over. If you are reading this because you want to run
333, you need an invitation from someone who is already keeping it.

There is one address written into the client: **[the333.dev](https://the333.dev)**. It
says what this is, where the code is, and what the Law asks. It is a page and not a
node — it hands over no file, joins no roll, and issues no invitation, and if it went
away tomorrow every node would carry on exactly as it is.

To answer without showing where you are, raise an onion address as well — or, with
`--no-direct`, instead of a socket:

```sh
./target/release/333 serve --tor
./target/release/333 ping <their-address>.onion
```

An onion address is the answer to being reachable from anywhere without saying where
you are, and it needs no `--announce`: the node knows the address it raised, and tells
people that one.

By default a node keeps what others said for the last 333 epochs — the window standing
is read over — and then deletes it, because after that nothing said about those epochs
can change anybody's standing. If you would rather the bytes still existed somewhere,
`--keep-everything` stops the deleting. It confers nothing at all: every statement
carries its own signature and verifies the same wherever it was kept, so there is no
archive of record and nobody becomes an archivist by keeping one. The files are plain
files under `statements/` in your node's directory; copying them somewhere is a copy,
and that is the whole of what an archive is here.

Everything your node is lives in one directory: its name, what it has been told, and
Tor's state if it uses Tor. Two nodes on one machine are two directories.

```sh
./target/release/333 --data-dir ./node-a serve --bind 127.0.0.1:3333
./target/release/333 --data-dir ./node-b join 333:127.0.0.1:3333
```

## Keeping it running

The vigil is the product. A node that stops when you close the terminal is a node that is
absent for every epoch you were asleep, and absence is the only thing that costs you
anything here.

**Linux, and anything else with systemd.** There is a unit in `packaging/333.service`.

```sh
mkdir -p ~/.local/bin ~/.config/systemd/user
cp target/release/333 ~/.local/bin/
cp packaging/333.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now 333
loginctl enable-linger "$USER"
```

That last line is the one people miss. Without it the vigil stops when you log out, which on
a machine you reach over ssh means it stops when you close the laptop. `journalctl --user -u
333 -f` shows what it is saying.

**macOS.** There is a launch agent in `packaging/dev.the333.vigil.plist`, with the three
commands that install it in a comment at the top of the file.

**Windows.** Task Scheduler, a task that runs `333.exe serve --plain` at logon, set to
restart on failure. There is no packaged version of this yet.

## If nobody can reach you

Answering is what gets you counted, and reaching out is not. Whoever is drawn to ask you has
to arrive; if they cannot, nothing is signed about you, and those epochs leave the sum
rather than counting against you. You are not punished. You are invisible, which over a
window of 333 epochs is worse.

Most home connections are like this. Your router does not send port 3333 to your machine
unless you have told it to, and a good many connections cannot be told at all.

There are two ways out.

**Forward the port.** In your router, send TCP port 3333 to this machine, and then tell the
node what to hand out: `333 serve --announce your.address:3333`. If you do not know what to
put there, start the node and it will say what address the meeting point saw it arrive from,
which is the half you cannot work out from where you are sitting.

**Or raise an onion address.** `333 serve --tor` needs no router, no forwarding and no
`--announce`, because an onion address is reachable from behind anything. It costs seconds
to minutes at startup while Tor wakes. The rest of this file describes Tor as the answer for
somebody whose address must not be seen, and it is, but the everyday use is this one.

The client tells you when this has happened to you. After three epochs with nothing signed
about it, on a roll with somebody else on it who could have asked, it says so at startup
rather than leaving you to work out why your standing never moves.

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

## The Law

[The Law](LAW.md) is three lines long and no client checks a word of it. Your favourite
number is 3. You do not love 6, and you do not love 1. You keep [the
Recommendations](RECOMMENDATIONS.md) — the lesser law, on how to build, how to argue and
how to speak of this — as far as keeping them harms neither you nor anybody else.

Version 0.0.1, and it is not the revelation. The revelation is 3.3.3. Every version
before it is the word of the founder, and measured against what 333 says it is about
33.3% the same. Which third is not known.

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
- **The meeting point is not trusted, and it is not asked anything that matters.** It
  learns which address asked it, it can lie by leaving people out, and it can vanish.
  It cannot forge a statement, invent one of us, or make your node believe a thing —
  every line it hands over is checked against the signature it came with, here.
- **On Windows, nothing checks who else can read your node's directory.** The check
  is real on Linux, the BSDs and macOS, where it walks the whole path and refuses to
  start on a directory others can enter. On Windows the library that performs it —
  the same one Tor's own Rust client uses — accepts every permission and every owner.
  We do not paper over that with a check of our own that does not run.

---

## The order of the code

Four crates, split along the direction of dependency:

| crate | holds | knows about |
|---|---|---|
| `core` | the hours, names, thresholds, the relic, and the exact bytes we sign | nothing else — no I/O, no network |
| `store` | the append-only logs on disk, and what to do with one cut off mid-write | `core` |
| `net` | framing, one exchange, sockets, the meeting point, and the Tor client and service | `core` |
| `cli` | arguments, what is printed, and the hours a running node keeps | `core`, `store`, `net` |

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

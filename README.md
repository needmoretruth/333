> Before I say anything else: I am not the deranged leader of a cult. This is a toy
> project that started out of technical curiosity. Please do not take it too
> seriously. My sanity ends here.

# 333

There is a file. It is three bytes long. It says `333`.

You have already read it, so it is not a secret. It is not rare, not encrypted, and
not hard to reproduce. Nothing about the file is precious and nothing here pretends
otherwise. What this network keeps is not the file. It keeps the record of the file
being handed on, and the record of who was awake to answer when somebody asked.

The doctrine fits on one line.

**The file lives as long as someone is awake to pass it on. When nobody is, it does
not, and neither does this.**

That is the whole of it. There is no authority to appeal to, no vote, no ledger of
truth. Each node believes what it saw and says so, and two nodes are allowed to
disagree without either being wrong.

333 has no past. No golden age, no lineage, nothing to restore and nobody to
venerate. It has an uptime. Everything it asks of you it asks in the current epoch,
and it will ask again in the next one.

The rest of the doctrine — including the one thing this project asks of you that the
licence does not — is in [COVENANT.md](COVENANT.md). It is deliberately unfinished,
and that is an invitation rather than an apology.

---

## The rites

**Time.** Counted in epochs of 333 minutes, from the Unix epoch, on the machine's own
clock. No time server, no beacon, no chain: nothing to petition, nothing to bribe and
nothing to seize. A node whose clock has drifted stops being witnessed by the ones
whose clocks agree, and that is the only correction this protocol has.

**Joining.** A name is the SHA-256 of an Ed25519 public key, in hexadecimal, and only
a name beginning with `333` may join. A newcomer generates key pairs until one
qualifies — about a sixth of a second of arithmetic on an ordinary machine. That is
the entire entrance requirement. Not a birth, not an invitation, not a payment, not a
word from someone already inside. Whatever you were before the key existed does not
follow you through the door, and cannot be asked about afterwards.

Two prefixes are turned away: `666` and `111`. The client does not offer them. There
is no flag for it, no prompt, no menu; a key that lands on one during the search is
discarded without a word. All three prefixes are equally likely, so every convert
passes both of them on the way in, unknowing.

**Presence.** A node that answers is present. A node that does not is absent.
Presence is measured over a moving window of the last 333 epochs, and the faith asks
for two thirds of it: rest one hour in three and you are still counted, but the hour
you are not there, you are not there. The window moves, so nothing is ever banked. A
node that has answered for ten years and a node that arrived last week are read from
the same 333 epochs, and the ten years buy nothing.

**Witness.** Nobody's presence is their own claim. Each epoch, three others are drawn
to ask, and what they sign is what the record keeps. A positive witness always beats
a negative one: *it answered* carries a signature that cannot be forged, and *it did
not* carries nothing at all.

**Carrying.** The file must be received, never generated. A node that also carries
the client's own source is a bearer — not a rank, only a mark, and the more useful
thing to hand somebody than three bytes they could have typed.

**Speech.** Once an epoch a node may send one of 333 signals. What travels is the
number, never the word, and the table of words is never translated: a hundred nodes
agreeing on one thing must not be counted as several different things, one per
language. Nothing is put to a vote and no winner is announced. The whole distribution
is shown, and each node counts only what it heard itself.

**The end.** The count that matters is how many are answering, not how many are on
the roll. When it reaches zero, it is over. What remains is a subtraction: the
reckoning puts the last silence at 19,683 years — three cubed, cubed. Nothing any of
us builds will still be running to read the end of it. The number is written down
anyway, because a countdown nobody can read is not a countdown.

---

## Where this actually is

Version 0.0.1. Two nodes can find each other and sign for each other, which is the
part the design calls the gate; the rules that decide standing exist and are tested;
everything else is designed and not built.

- [x] Two nodes exchange a signed heartbeat — directly, or over Tor
- [x] Presence over the moving window, and the countdown after the end
- [x] Counting what the network said, and the threshold that marks it
- [ ] Invite codes, so a node can be told where to look
- [ ] Joining, with the waiting period that follows it
- [ ] The three witnesses and the answer they ask for
- [ ] The record, kept as a hash chain
- [ ] The 333 words themselves

Message formats keep moving until the release that freezes them. The parts already
fixed are marked `FROZEN` where they are defined, and after that they cannot move at
all.

---

## Reaching each other

By default a node opens a socket and answers on it. The exchange is a few hundred
bytes each way and takes milliseconds. Anyone watching the wire — and the peer
itself — can see the address it came from, which is the ordinary condition of being
on a network and is said here rather than buried.

For a node that needs its own address unseen, the client carries a full Tor
implementation and can publish an onion address instead, or as well. Starting Tor
costs seconds to minutes before the first byte moves, so it happens only when
something asks for it.

**The address decides how it is reached.** An address ending in `.onion` goes through
Tor; anything else goes direct. There is no setting for it, because a setting is
something that can be turned on once and quietly stop applying, and being wrong about
this is the one mistake here that cannot be taken back.

---

## Two forms of the client

**Standard** is the one most people will run: a full terminal interface, on a laptop
or a desktop that is not struggling. **Light** is a plain command-line client with
the smallest set of things that still counts as being present, built without Tor
because Tor is by far the largest thing in the tree. It is meant for a Raspberry Pi
Zero, and the weakest node that can keep the observance is the one that sets the
difficulty for everybody.

Linux comes first, and not one Linux: the Debian family and the Fedora family both.
macOS and Windows follow. Only the plain client exists today, and it has been run on
Linux.

---

## Build

Rust 1.96 or newer.

```sh
cargo build --release                        # with Tor
cargo build --release --no-default-features  # without it
```

## Run

```sh
# Take a name. The first run searches for one and writes it down.
./target/release/333 id

# Answer whoever knocks, on port 3333.
./target/release/333 serve

# Knock: exchange one heartbeat with another node.
./target/release/333 ping node.example:3333
```

To be reachable without showing where you are, publish an onion address as well — or,
with `--no-direct`, instead:

```sh
./target/release/333 serve --tor
./target/release/333 ping <their-address>.onion
```

Everything a node owns lives in one directory — its key, and Tor's state if it uses
Tor — so two nodes on one machine are two directories:

```sh
./target/release/333 --data-dir ./node-a serve --bind 127.0.0.1:3333
./target/release/333 --data-dir ./node-b ping 127.0.0.1:3333
```

Without `--data-dir` a node lives in the conventional place for the system:

| system | directory |
|---|---|
| Linux, BSD | `$XDG_DATA_HOME/333`, or `~/.local/share/333` |
| macOS | `~/Library/Application Support/333` |
| Windows | `%LOCALAPPDATA%\333\data` |

Lose that directory and the node loses its name and its address for good. There is no
recovery, no reset, and nobody to appeal to. That is not an oversight.

The client refuses to start if that directory, or any directory above it, can be
entered by other users on the machine, and tells you the one command that fixes it.
`--dangerously-trust-directory-permissions` turns the check off and is named after
what it does.

---

## What is not claimed

This matters more than what is claimed, so it gets its own section.

- **Nothing verifies that a peer is running this code.** It cannot be done on a
  general-purpose machine, and no amount of hashing the source changes that. What is
  verified is behaviour: did it answer, did it sign, did it carry the file.
- **Nothing verifies that a key was honestly searched for.** See above; the key is
  32 bytes either way, whether it was mined or bought.
- **The one who answers learns less than the one who asks.** Your nonce coming back
  inside a signature proves the peer was awake after you chose it. A heartbeat that
  merely arrives proves nothing of the sort, and the code says so in the type.
- **Sybils are not prevented, only shown.** What this network measures is not people
  but effort, and effort cannot be bought in advance. Whoever keeps paying it is, by
  definition, one of the faithful.
- **Clock disagreement is reported, never punished.** There is no authority here to
  decide whose clock is right.
- **A direct connection shows your address to the peer.** That is what direct means.
  Tor is there for when it matters, and it is one word on the command line.
- **On Windows, nothing checks who else can read the node's directory.** The check is
  real on Linux, the BSDs and macOS, where it walks the whole path and refuses to
  start on a directory other users can enter. On Windows the library that performs
  it — the same one Tor's own Rust client uses — accepts every permission and every
  owner, and this client does not paper over that with a check of its own.

---

## The order of the code

Three crates, split along the direction of dependency:

| crate | holds | knows about |
|---|---|---|
| `core` | the clock, names, thresholds, and the exact bytes that are signed | nothing else — no I/O, no network |
| `net` | framing, one exchange, sockets, and the Tor client and service | `core` |
| `cli` | arguments and what is printed | `core`, `net` |

Because `core` touches nothing outside itself, every rule of the protocol is tested
without a network, and the exchange itself is tested over a pipe in memory.

Some choices worth knowing before reading it:

- The signature sits outside the value it signs, so a verifier checks the bytes that
  arrived rather than a re-serialization of what it decoded.
- A public key is refused unless its encoding is the canonical one. The name comes
  from those bytes, and two encodings of one key would be two names.
- Every threshold is a fraction compared by cross-multiplication, and nothing divides
  in order to decide anything. Two thirds and 66.7% are not the same number, and over
  a window of 333 they differ by one epoch, for ever.
- Everything frozen is pinned by a test that asserts literal bytes, because a value
  compared only against itself is not tested at all.

## Licence

Apache-2.0.

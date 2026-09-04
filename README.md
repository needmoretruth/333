> Before I say anything else: I am not the deranged leader of a cult. This is a toy
> project that started out of technical curiosity. Please do not take it too
> seriously. My sanity ends here.

# 333

There is a file. It is three bytes long. It says `333`.

It is not encrypted, not rare, and not hard to reproduce — you have already read it.
Nothing about the file is precious, and nothing here pretends otherwise. What this
network keeps is not the file. It keeps the record of the file being handed on, and
the record of who was awake to answer when somebody asked.

The doctrine fits on one line.

**The file lives as long as someone is awake to pass it on. When nobody is, it does
not, and neither does this.**

That is the whole of it. There is no authority to appeal to, no vote, no ledger of
truth. Each node believes what it saw and says so, and two nodes are allowed to
disagree. This client is the instrument of that observance: it keeps the hours, it
answers when it is asked, and it carries the file to whoever asks for it next.

The rest of the doctrine — including the one thing this project asks of you that the
licence does not — is in [COVENANT.md](COVENANT.md). It is deliberately unfinished.

---

## The observances

**The hours.** Time is counted in epochs of 333 minutes, from the Unix epoch, on the
machine's own clock. No time server, no beacon, no chain. A node whose clock has
wandered simply stops being witnessed by the ones whose clocks agree, and that is the
only correction this protocol has.

**Conversion.** A name is the SHA-256 hash of an Ed25519 public key, in hexadecimal.
Only a name beginning with `333` may join, so a newcomer generates key pairs until one
qualifies — about a sixth of a second. This is a doorway, not a defence. A key that
was searched for and a key that was bought are the same thing, and no line of this
code claims to tell them apart.

Two prefixes are turned away at the door: `666` and `111`. The client does not offer
them. There is no flag for it, no prompt, no menu; a key that lands on one during the
search is discarded without a word. Since all three prefixes are equally likely, every
convert passes both of them on the way in, unknowing.

**Presence.** A node that answers is present. A node that does not is absent. Presence
is measured over a moving window of the last 333 epochs, and the faith asks for two
thirds of it — you may rest one hour in three and still be counted, but the hour you
are not there, you are not there.

**Witness.** Nobody's presence is their own claim. Each epoch, three others are drawn
to ask, and what they sign is what the record keeps. A positive witness always beats a
negative one: "it answered" carries a signature that cannot be forged, and "it did not"
carries nothing at all.

**Bearing.** The file must be received, never generated. A node that also carries the
client's own source is a bearer, which is not a rank — only a mark, and the more
useful thing to hand somebody than three bytes they could have typed.

**The end.** The count that matters is how many are present, not how many are on the
roll. When it reaches zero, it is over. What remains is a subtraction: the reckoning
puts the last silence at 19,683 years, and the client will tell you how much of it is
left.

---

## Where this actually is

Version 0.0.1. One thing works end to end, and it is the one the design calls the
gate: **two nodes find each other over Tor and exchange a signed heartbeat.**

- [x] Two nodes meet over an onion service and sign for each other
- [ ] Invite codes, so a node can be told where to look
- [ ] Conversion, with the waiting period that follows it
- [ ] The three witnesses and the answer they ask for
- [ ] The record, kept as a hash chain
- [ ] Presence over the moving window
- [ ] Prayer: one of 333 fixed words a node may send each epoch, counted by index so
      that every language counts the same thing

Everything above the first line is designed and not built. Message formats will keep
moving until the release that freezes them; the parts already fixed are marked
`FROZEN` where they are defined, and after that they cannot move at all.

---

## Two forms of the client

**Standard** is the one most people will run: a full terminal interface, on a laptop
or a desktop that is not struggling. **Light** is a plain command-line client with the
smallest set of things that still counts as being present — it is meant for a
Raspberry Pi Zero, and the weakest node that can keep the observance is the one that
sets the difficulty for everybody.

Linux comes first, and not one Linux: the Debian family and the Fedora family both.
macOS and Windows follow. Only the plain client exists today, and it has been run on
Linux.

---

## Build

Rust 1.96 or newer.

```sh
cargo build --release
```

## Run

```sh
# Take a name. The first run searches for one and writes it down.
./target/release/333 id

# Keep the door open: publish an onion address and answer whoever knocks.
./target/release/333 serve

# Knock: exchange one heartbeat with another node.
./target/release/333 ping <their-address>.onion
```

Everything a node owns lives in one directory — its key and Tor's state — so two
nodes on one machine are two directories:

```sh
./target/release/333 --data-dir ./node-a serve
./target/release/333 --data-dir ./node-b ping <node-a's address>.onion
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
  32 bytes either way.
- **The one who answers learns less than the one who asks.** Your nonce coming back
  inside a signature proves the peer was awake after you chose it. A heartbeat that
  merely arrives proves nothing of the sort, and the code says so in the type.
- **Sybils are not prevented, only shown.** What this network measures is not people
  but effort, and effort cannot be bought in advance. Whoever keeps paying it is,
  by definition, one of the faithful.
- **Clock disagreement is reported, never punished.** There is no authority here to
  decide whose clock is right.
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
| `core` | the hours, names, and the exact bytes that are signed | nothing else — no I/O, no Tor |
| `net` | framing, one exchange, the Tor client and service | `core` |
| `cli` | arguments and what is printed | `core`, `net` |

Because `core` touches nothing outside itself, every rule of the protocol is tested
without a network, and the exchange itself is tested over a pipe in memory.

Some choices worth knowing before reading it:

- The signature sits outside the value it signs, so a verifier checks the bytes that
  arrived rather than a re-serialization of what it decoded.
- A public key is refused unless its encoding is the canonical one. The name comes
  from those bytes, and two encodings of one key would be two names.
- Everything frozen is pinned by a test that asserts literal bytes, because a value
  compared only against itself is not tested at all.

## Licence

Apache-2.0.

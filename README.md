# 333

A peer-to-peer network whose only job is to keep one file alive.

The file is `333.txt`. It is three bytes long, it is not a secret, and anybody can
type it out from memory. That is deliberate: what this network records is not the
file, but the act of handing it to somebody else — and the fact that a node was there
to answer when another node asked.

If nobody runs a node, there is nothing left to ask, and the network ends.

## Status

Version 0.0.1. One piece of the design works end to end:

- [x] Two nodes find each other over Tor and exchange a **signed heartbeat**
- [ ] Invite codes, so a node can be told where to look
- [ ] Enrolment
- [ ] Liveness challenges
- [ ] Attendance records over time

Nothing is stable yet. Message formats will change until the release that freezes
them; the ones that are already fixed are marked `FROZEN` where they are defined.

## What a node does today

Every node has an Ed25519 key pair. Its name is the SHA-256 hash of the public key,
written in hexadecimal, and the network only accepts a name that begins with `333` —
so a node generates key pairs until one qualifies. On this machine that takes about
0.15 seconds. It is a doorway, not a defence: there is no way to tell a key that was
searched for from a key that was bought, and this project does not pretend otherwise.

A heartbeat says who sent it, which epoch the sender believes it is, and — when it is
an answer — which heartbeat it answers. An epoch is 333 minutes of wall-clock time,
counted from the Unix epoch, with no external clock consulted. Nodes reach each other
as Tor onion services, so neither side learns the other's address.

## Build

Rust 1.96 or newer.

```sh
cargo build --release
```

## Run

```sh
# Create this node's identity and print its name.
./target/release/333 id

# Publish an onion address and answer heartbeats until interrupted.
./target/release/333 serve

# Exchange one heartbeat with another node.
./target/release/333 ping <their-address>.onion
```

Everything a node owns lives in one directory — its identity key and Tor's state —
so two nodes on one machine are just two directories:

```sh
./target/release/333 --data-dir ./node-a serve
./target/release/333 --data-dir ./node-b ping <node-a's address>.onion
```

Losing that directory loses the node's name and its onion address for good.

## How it is put together

Three crates, split along the direction of dependency:

| crate | holds | knows about |
|---|---|---|
| `core` | epochs, identities, the signed bytes | nothing else — no I/O, no Tor |
| `net` | framing, one exchange, the Tor client and service | `core` |
| `cli` | argument parsing and output | `core`, `net` |

Because `core` performs no I/O, every protocol rule is tested without a network, and
the exchange itself is tested over an in-memory pipe.

Some choices worth knowing before reading the code:

- **No consensus.** There is no point in this system where anyone has to decide who
  is right. Two nodes can disagree about what they saw, and that is not a bug.
- **Nothing claims to verify the unverifiable.** A node checks signatures and
  behaviour. It does not check that a peer is running unmodified code, because that
  cannot be checked.
- **The signature sits outside the value it signs.** A verifier checks the bytes it
  actually received, never a re-serialization of what it decoded.
- **A public key is refused unless its encoding is canonical.** The node's name comes
  from those bytes, so two encodings of one key would be two names.
- **Clock skew is reported, not punished.** The protocol has no authority to decide
  whose clock is right.

## Licence

Apache-2.0.

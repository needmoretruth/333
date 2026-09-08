# Reporting a flaw

**Report it privately here:**
[github.com/needmoretruth/333/security/advisories/new](https://github.com/needmoretruth/333/security/advisories/new).
That form is not public and does not need an email address. Please do not open an ordinary
issue for anything that could be used against somebody's node before it is fixed.

Say what you did, what happened, and what you expected instead. A rough report that is
true is worth more than a polished one that is guessed, and you do not need working code
to be taken seriously.

## What is worth reporting

Anything that lets one node be made to say something untrue about another, or about
itself: a signature accepted that should not have been, a challenge answered that was
never entitled to be asked, a record that reads differently on two machines, an
attestation that survives being altered. Anything that reveals where a node is when its
owner arranged not to be seen — particularly anything that ties an onion address to an
ordinary one. Anything that lets one caller stop a node answering anybody else. Anything
in the bytes that are meant to be frozen: a frame that parses two ways, a length that is
not checked, a domain string that can be made to collide with another.

## What is not a flaw

Some of what looks like one is the design, written down and argued for.

- **Somebody running many nodes gets many draws.** There is no defence against that and
  none is claimed. It is in the README under what is not claimed.
- **Two nodes hold different pictures.** There is no agreement protocol. Disagreement is
  the expected state, not a failure.
- **The client cannot tell whether a peer is running this code.** It never claims to.
  What is verified is what a peer did, never what it is.
- **The meeting point learns who asked it.** Written down in the README, with what it
  costs and how to refuse it.

## What happens next

There is one person answering, and that person has set the work down for a while and said
so in [a letter](LETTER.md). So there is no schedule, it would be dishonest to print one,
and a report may wait longer than it should. You will be told what was understood and
whether it is being fixed. If a fix changes bytes that other nodes depend on, that is said
in the release rather than slipped in.

Nothing here offers money, and nothing here asks you to stay quiet. If a fix is taking
longer than you think reasonable, publish; being told late is worse than being told in
public.

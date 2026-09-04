//! Handing the file from one node to the next.
//!
//! This is the act the whole network is built around. Everything else — the hours,
//! the challenges, the record — is bookkeeping about nodes that are already members,
//! and a node becomes a member here and nowhere else.
//!
//! The round, after the heartbeat:
//!
//! ```text
//! asker  →  giver   a signed plea: this is my key, this is the epoch I think it is
//! giver  →  asker   the file, then "I gave it to you in epoch N" + signature
//! asker  →  giver   "I received it from you in epoch N" + signature
//! giver  →  asker   what the giver has to pass on, then an empty frame
//! ```
//!
//! THE LAST STEP IS WHY A NEWCOMER IS NOT ALONE. Its own two halves put exactly one
//! name on its roll — its own — and a roll of one draws no verifiers, so a node that
//! received nothing else could never be asked anything by anybody, for ever. It would
//! also know where nobody is, including the node it was standing in front of a moment
//! ago. So the giver passes on what it holds: the admissions that make up the roll, and
//! the addresses members have signed for themselves. Exactly the same run of statements
//! that [`crate::gossip`] trades, sent here because this is the one moment a node has
//! nothing at all.
//!
//! A giver with nothing to pass on is the one node that was never given the file by
//! anybody. Its newcomer starts alone, and stays alone until somebody else joins.
//!
//! TWO HALVES, TWO KEYS. Neither half means anything alone, so neither node can
//! manufacture a member. It is also why the round cannot be shortened: the asker
//! cannot sign for having received something it has not been given yet.
//!
//! THE GIVER'S CLOCK DECIDES WHEN YOU JOINED. The asker states an epoch and the giver
//! ignores it except as a sanity check; what gets signed is the giver's own epoch, and
//! the asker either agrees with it closely enough to countersign or walks away. So the
//! date on a membership is one that somebody who was already a member stood behind.
//!
//! WHAT THIS DOES NOT DO. It does not check that the giver is a member, or active, or
//! anything else. It cannot: this node's view of who is a member is its own, and a
//! newcomer has no view at all. What the record says is who handed it over, and every
//! reader judges that for themselves.

use futures::{AsyncRead, AsyncWrite};
use n333_core::enrollment::{self, Refusal};
use n333_core::plea::{Plea, Signed as SignedPlea};
use n333_core::subject::{self, NotTheFile, Subject};
use n333_core::transfer::{self, Half, Mismatch, Record, Transfer};
use n333_core::{Epoch, Identity, wire};

use crate::frame::{self, AsReceived};

/// How far apart two clocks may be and still complete a handover.
///
/// One epoch. The two nodes have to sign the same number or the halves do not fit, so
/// somebody's clock has to give way, and this is how far the asker will follow the
/// giver. Wider would let a giver backdate a membership by as much as it liked;
/// narrower would break a handover that straddles an epoch boundary, which is a thing
/// that happens on its own every 333 minutes.
pub const MAX_EPOCH_SKEW: u64 = 1;

/// Why a handover did not happen.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The stream failed mid-frame.
    #[error("frame: {0}")]
    Frame(#[from] frame::Error),
    /// A signed message was malformed, or its signature did not check out.
    #[error("message: {0}")]
    Message(#[from] wire::Error),
    /// What arrived is not the file.
    #[error("what arrived is not the file: {0}")]
    NotTheFile(#[from] NotTheFile),
    /// The two halves of the record do not fit together.
    #[error("the halves do not fit: {0}")]
    Mismatch(#[from] Mismatch),
    /// The record is about somebody who is not on this connection.
    #[error("the record handed over is about somebody else")]
    NotUs,
    /// The two clocks are too far apart to sign the same epoch.
    #[error("this node is in epoch {ours} and the other is in {theirs}")]
    Apart {
        /// The epoch this node believes it is in.
        ours: u64,
        /// The epoch the other node stated.
        theirs: u64,
    },
    /// The asker's name begins with one of the two refused prefixes.
    ///
    /// The door is where the network meets a heretic, and the only place it can. A
    /// client refuses its own cursed key long before this, so a name that reaches here
    /// was made somewhere else and presented on purpose.
    #[error("333 has looked at that name and taken 333 milliseconds off the life of whoever holds it")]
    Cursed,
    /// The asker's name does not begin with 333.
    #[error("that is not a name 333 answers to")]
    Ineligible,
}

impl From<Refusal> for Error {
    fn from(refusal: Refusal) -> Self {
        match refusal {
            Refusal::Cursed => Self::Cursed,
            Refusal::Ineligible => Self::Ineligible,
        }
    }
}

/// A completed handover, from either side.
#[derive(Debug, Clone)]
pub struct Handover {
    /// Both halves, checked against each other.
    pub transfer: Transfer,
    /// The giver's half, exactly as it travelled.
    pub gave: Vec<u8>,
    /// The receiver's half, exactly as it travelled.
    pub received: Vec<u8>,
}

/// What the asker ends the round holding.
#[derive(Debug, Clone)]
pub struct Taken {
    /// The record of the handover.
    pub handover: Handover,
    /// The file itself, recognised.
    pub subject: Subject,
    /// What the giver passed on, unopened and unjudged.
    ///
    /// Kept as they arrived. Whether any of them mean anything is decided by reading
    /// them, not by who handed them over.
    pub tidings: Vec<Vec<u8>>,
}

/// Ask a node for the file, and countersign the record if it gives it.
///
/// `now` is this node's own epoch, used only to decide whether the giver's clock is
/// close enough to follow.
///
/// # Errors
/// Fails if the stream fails, what arrives is not the file, the giver's record is
/// about somebody else, or the two clocks are too far apart.
pub async fn ask<S>(stream: &mut S, asker: &Identity, now: Epoch) -> Result<Taken, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    frame::write_frame(stream, &Plea::of(asker, now).seal(asker)?).await?;

    // The file first, then the giver's half of the record. In that order because the
    // record names the file's hash, and a reader should be able to check the one
    // against the other without trusting the order they arrived in.
    let bytes = frame::read_frame(stream).await?;
    let subject = Subject::recognise(&bytes)?;
    let gave_frame = frame::read_frame(stream).await?;
    let gave = transfer::open(&gave_frame, Half::Gave)?;

    if gave.record.counterparty != asker.public_key() {
        return Err(Error::NotUs);
    }
    if gave.record.subject != subject::DIGEST {
        return Err(Error::NotTheFile(NotTheFile::WrongContent));
    }
    let theirs = gave.record.epoch();
    if theirs.0.abs_diff(now.0) > MAX_EPOCH_SKEW {
        return Err(Error::Apart {
            ours: now.0,
            theirs: theirs.0,
        });
    }

    // Signed with the giver's epoch, not this node's: the halves have to carry the
    // same number, and the giver is the one who was already a member.
    let received_frame = Record::new(asker, gave.record.author, theirs, subject::DIGEST)
        .seal(Half::Received, asker)?;
    frame::write_frame(stream, &received_frame).await?;

    let received = transfer::open(&received_frame, Half::Received)?;
    Ok(Taken {
        handover: Handover {
            transfer: Transfer::assemble(gave, received)?,
            gave: gave_frame,
            received: received_frame,
        },
        subject,
        tidings: frame::read_batch(stream).await?,
    })
}

/// Hand the file over to whoever asked, and take their half of the record.
///
/// `now` is this node's own epoch, and it is the one that gets signed. `tidings` is
/// what this node passes on so the newcomer is not left knowing nobody and nowhere.
///
/// # Errors
/// Fails if the asker's name is refused, the clocks are too far apart, the stream
/// fails, or the half handed back does not fit the one sent.
pub async fn give<S>(
    stream: &mut S,
    giver: &Identity,
    now: Epoch,
    plea: &AsReceived<SignedPlea>,
    subject: &Subject,
    tidings: &[Vec<u8>],
) -> Result<Handover, Error>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    // Judged before anything is read or sent. There is exactly one door into this
    // network and this is it, so it is also the one place a heretic can be met.
    enrollment::admit(&plea.message.asker)?;
    let theirs = plea.message.plea.epoch();
    if theirs.0.abs_diff(now.0) > MAX_EPOCH_SKEW {
        return Err(Error::Apart {
            ours: now.0,
            theirs: theirs.0,
        });
    }

    let gave_frame = Record::new(giver, plea.message.plea.asker, now, subject::DIGEST)
        .seal(Half::Gave, giver)?;
    // One flush for both: over Tor each flush is its own partial cell.
    frame::write_frames(stream, &[subject.content().as_slice(), &gave_frame]).await?;

    let received_frame = frame::read_frame(stream).await?;
    let received = transfer::open(&received_frame, Half::Received)?;
    if received.record.author != plea.message.plea.asker {
        return Err(Error::NotUs);
    }
    let gave = transfer::open(&gave_frame, Half::Gave)?;

    frame::write_batch(stream, tidings).await?;

    Ok(Handover {
        transfer: Transfer::assemble(gave, received)?,
        gave: gave_frame,
        received: received_frame,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asked::{Asked, take_request};
    use n333_core::Roll;
    use n333_core::plea::Signed as OpenedPlea;
    use tokio_util::compat::TokioAsyncReadCompatExt as _;

    /// Names that begin with 333, which is what the door checks.
    ///
    /// Found by trying numbered seeds in order, the way a client mines: 1741 is the
    /// 1741st key tried, and the first whose name is eligible. Fixed here so the tests
    /// do not spend the search on every run.
    const ELIGIBLE: [u32; 4] = [1741, 2337, 7688, 7881];

    fn identity(which: usize) -> Identity {
        Identity::from_seed(&seed(ELIGIBLE[which]))
    }

    /// The file, which only a test is allowed to conjure: a client cannot.
    fn the_file() -> Subject {
        Subject::recognise(b"333").expect("is the file")
    }

    /// A plea as the giver would have taken it off the wire.
    fn plea_from(asker: &Identity, epoch: Epoch) -> AsReceived<OpenedPlea> {
        let frame = Plea::of(asker, epoch).seal(asker).expect("seals");
        AsReceived {
            message: n333_core::plea::open(&frame).expect("opens"),
            frame,
        }
    }

    /// Both sides of one handover, over a real socket pair.
    ///
    /// The identities are rebuilt from their seeds on each side rather than shared: a
    /// key is deliberately not something that can be cloned around, and each side of a
    /// real handover holds only its own.
    async fn handover(
        giver_seed: usize,
        asker_seed: usize,
        giver_epoch: Epoch,
        asker_epoch: Epoch,
        tidings: Vec<Vec<u8>>,
    ) -> (Result<Handover, Error>, Result<Taken, Error>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("binds");
        let address = listener.local_addr().expect("has an address");

        let giving = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accepts");
            let mut stream = stream.compat();
            // The plea comes off the wire exactly as a running node takes it.
            let Asked::TheFile(plea) = take_request(&mut stream).await.expect("a plea") else {
                panic!("the asker asked for something else");
            };
            let given = give(
                &mut stream,
                &identity(giver_seed),
                giver_epoch,
                &plea,
                &the_file(),
                &tidings,
            )
            .await;
            // Held open until the asker has read everything: dropping a socket with
            // bytes still in flight resets it, and that is the test's doing, not the
            // protocol's.
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            given
        });

        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .expect("connects")
            .compat();
        let taken = ask(&mut stream, &identity(asker_seed), asker_epoch).await;
        (giving.await.expect("the giver finished"), taken)
    }

    #[tokio::test]
    async fn a_handover_makes_a_member_out_of_a_stranger() {
        let (giver, asker) = (identity(0), identity(1));
        let (given, taken) = handover(0, 1, Epoch(900), Epoch(900), Vec::new()).await;
        let given = given.expect("the giver completed");
        let taken = taken.expect("the asker completed");

        assert_eq!(taken.subject.content(), b"333");
        assert_eq!(given.transfer.giver(), &giver.node_id());
        assert_eq!(given.transfer.receiver(), &asker.node_id());
        assert_eq!(given.transfer.epoch(), Epoch(900));

        // The two sides hold the same two frames, and either one alone can put the
        // asker on a roll from them.
        assert_eq!(given.gave, taken.handover.gave);
        assert_eq!(given.received, taken.handover.received);
        let (roll, read) = Roll::from_halves(&[given.gave, given.received]);
        assert_eq!(read.admitted, 1);
        assert!(roll.member(&asker.public_key()).is_some());
    }

    #[tokio::test]
    async fn what_the_giver_passes_on_is_what_keeps_a_newcomer_from_being_alone() {
        // A roll of one draws no verifiers, so without this a newcomer could never be
        // asked anything and could never answer.
        let (elder, giver, asker) = (identity(3), identity(0), identity(1));
        let (elders_gift, _) = handover(3, 0, Epoch(800), Epoch(800), Vec::new()).await;
        let elders_gift = elders_gift.expect("the elder completed");
        let passed_on = vec![elders_gift.gave, elders_gift.received];

        let (_, taken) = handover(0, 1, Epoch(900), Epoch(900), passed_on).await;
        let taken = taken.expect("the asker completed");
        assert_eq!(taken.tidings.len(), 2);

        let mut held = taken.tidings;
        held.push(taken.handover.gave);
        held.push(taken.handover.received);
        let (roll, _) = Roll::from_halves(&held);
        assert!(roll.member(&asker.public_key()).is_some(), "itself");
        assert!(roll.member(&giver.public_key()).is_some(), "the one who gave");
        assert!(roll.member(&elder.public_key()).is_none(), "no further back");
    }

    #[tokio::test]
    async fn a_giver_who_received_from_nobody_hands_over_nothing() {
        // The one node that was never given the file. Its newcomer starts alone, and
        // stays alone until somebody else joins.
        let (_, taken) = handover(0, 1, Epoch(900), Epoch(900), Vec::new()).await;
        assert!(taken.expect("the asker completed").tidings.is_empty());
    }

    #[tokio::test]
    async fn one_half_alone_puts_nobody_on_a_roll() {
        // The whole reason the round has three messages instead of two.
        let (given, _) = handover(0, 1, Epoch(900), Epoch(900), Vec::new()).await;
        let given = given.expect("the giver completed");
        let (roll, read) = Roll::from_halves(&[given.gave]);
        assert!(roll.is_empty());
        assert_eq!(read.unpaired, 1);
    }

    #[tokio::test]
    async fn a_clock_an_epoch_out_still_completes_and_signs_the_givers_number() {
        // Boundaries are crossed mid-handover every 333 minutes, so one epoch of
        // disagreement has to work.
        let (given, taken) = handover(0, 1, Epoch(900), Epoch(899), Vec::new()).await;
        assert_eq!(given.expect("the giver completed").transfer.epoch(), Epoch(900));
        assert_eq!(
            taken.expect("the asker completed").handover.transfer.epoch(),
            Epoch(900)
        );
    }

    #[tokio::test]
    async fn a_giver_refuses_a_clock_further_out_than_that_before_sending_anything() {
        let (given, taken) = handover(0, 1, Epoch(900), Epoch(800), Vec::new()).await;
        assert!(matches!(given, Err(Error::Apart { .. })));
        // The asker is simply left with nothing, which is all a refusal can look like
        // from the far end of a connection that stopped.
        assert!(taken.is_err(), "no file arrived");
    }

    #[tokio::test]
    async fn an_asker_refuses_a_backdated_record_even_from_a_giver_that_does_not_check() {
        // The asker's own guard, tested against a giver that ignores the rule: without
        // it, a giver could sign a membership into the past and the asker would
        // countersign it.
        let (elder, asker) = (identity(0), identity(1));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("binds");
        let address = listener.local_addr().expect("has an address");

        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accepts");
            let mut stream = stream.compat();
            let _ = take_request(&mut stream).await.expect("a plea");
            let backdated = Record::new(&elder, identity(1).public_key(), Epoch(700), subject::DIGEST)
                .seal(Half::Gave, &elder)
                .expect("seals");
            frame::write_frames(&mut stream, &[the_file().content().as_slice(), &backdated])
                .await
                .expect("writes");
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        });

        let mut stream = tokio::net::TcpStream::connect(address)
            .await
            .expect("connects")
            .compat();
        let refused = ask(&mut stream, &asker, Epoch(900)).await;
        assert!(matches!(
            refused,
            Err(Error::Apart {
                ours: 900,
                theirs: 700
            })
        ));
    }

    #[tokio::test]
    async fn the_cursed_are_turned_away_at_the_door() {
        // Seed 4307 mines a name beginning with 666; see the enrolment tests.
        let (giver, cursed) = (identity(0), Identity::from_seed(&seed(4307)));
        // No stream is needed: the refusal happens before a byte is written, which is
        // the point of doing it first.
        let mut unused = futures::io::Cursor::new(Vec::new());
        let plea = plea_from(&cursed, Epoch(1));
        let refused = give(&mut unused, &giver, Epoch(1), &plea, &the_file(), &[]).await;
        assert!(matches!(refused, Err(Error::Cursed)));
        assert!(unused.into_inner().is_empty(), "nothing was sent");
    }

    /// A numbered seed, written the way the enrolment tests write theirs.
    fn seed(n: u32) -> [u8; 32] {
        let mut seed = [0_u8; 32];
        seed[0..4].copy_from_slice(&n.to_le_bytes());
        seed
    }
}

//! Carrying out what this node was told: typed into the screen, or handed over from
//! another terminal on this machine.
//!
//! The screen reads the keys and draws; it opens nothing. This is where the dialler,
//! the node and the listeners are, so this is where an order becomes a connection. It
//! is the same node whoever asked. Nothing here opens the directory a second time, and
//! nothing starts a second dialler: a second of either inside one vigil would be a
//! second writer to its files and a second Tor client on its state.
//!
//! EVERYTHING SAYS SOMETHING. An order that worked, an order that failed and an order
//! that was refused all put a line in the vigil, because the person typed into a
//! screen and is looking at that screen for the answer. An order handed over from
//! elsewhere is carried out on a task that knows who asked, so the same lines go back
//! to them, and it ends by saying whether it was done.
//!
//! WHAT WAITS FOR WHAT. Reaching somebody who does not answer takes as long as the
//! deadline allows, so those orders run on tasks of their own and the next order is not
//! kept waiting behind them. Saying one of the 333 does not: two of them carried out at
//! once could both find that nothing had been said yet this epoch.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use n333_net::PeerAddress;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{oneshot, watch};
use tokio::task::JoinHandle;

use crate::commands::Common;
use crate::dial::Dialer;
use crate::node::Node;
use crate::orders::Order;

use super::door::Door;

/// An order handed over from another terminal, and where its answer goes.
// Made only where there is a socket to hand one over through; see `told`.
#[cfg_attr(not(unix), allow(dead_code))]
pub(super) struct Ask {
    /// What was asked, already read.
    pub(super) order: Order,
    /// The words whoever asked reads, which every line about it is said in.
    pub(super) words: &'static crate::words::Words,
    /// Where every line said while carrying it out goes.
    pub(super) lines: UnboundedSender<String>,
    /// Whether it was done, sent once at the end.
    pub(super) done: oneshot::Sender<bool>,
}

/// Everything an order can need, held by the vigil.
pub(super) struct Carrier {
    /// The node, open once, for the life of the vigil.
    pub(super) node: Arc<Node>,
    /// The options the vigil was started with.
    pub(super) common: Common,
    /// The vigil's own dialler, and the Tor client inside it if one is up.
    pub(super) dialer: Dialer,
    /// Where the vigil tells others to find it.
    pub(super) found_address: watch::Sender<Option<PeerAddress>>,
    /// The onion listener, when one has been started from here.
    ///
    /// Held so that it can be stopped again: a person who turned it on wants to be able
    /// to turn it off, and a task nobody holds is a task nobody can stop.
    pub(super) unseen: Mutex<Option<JoinHandle<anyhow::Result<()>>>>,
}

/// One order, ready to be carried out.
type Work = Pin<Box<dyn Future<Output = bool> + Send>>;

/// Do what this node is told, for as long as anybody can tell it anything.
pub(super) async fn until_nobody_asks(
    mut typed: UnboundedReceiver<Order>,
    mut told: UnboundedReceiver<Ask>,
    carrier: Carrier,
) {
    let carrier = Arc::new(carrier);
    loop {
        let (order, asker) = tokio::select! {
            Some(order) = typed.recv() => (order, None),
            Some(ask) = told.recv() => (ask.order, Some((ask.words, ask.lines, ask.done))),
            else => return,
        };
        let apart = takes_a_while(&order);
        let work = Arc::clone(&carrier).work(order);
        let carried = async move {
            match asker {
                Some((words, lines, done)) => {
                    let work = crate::words::spoken::spoken_in(words, work);
                    let _ = done.send(crate::aloud::heard_by(lines, work).await);
                }
                None => {
                    work.await;
                }
            }
        };
        if apart {
            tokio::spawn(carried);
        } else {
            carried.await;
        }
    }
}

/// Does this order wait on somebody else, and so go on a task of its own?
const fn takes_a_while(order: &Order) -> bool {
    matches!(
        order,
        Order::Ping(_) | Order::Join(_) | Order::Bootstrap { .. } | Order::TorOn
    )
}

impl Carrier {
    /// The work one order is, saying true if it was done.
    fn work(self: Arc<Self>, order: Order) -> Work {
        match order {
            Order::Ping(address) => Box::pin(async move { self.ping(&address).await }),
            Order::Join(address) => Box::pin(async move { self.join(&address).await }),
            Order::Bootstrap { anyway } => Box::pin(async move { self.bootstrap(anyway).await }),
            Order::Say(which) => Box::pin(async move { self.say(&which).await }),
            Order::Status(show) => Box::pin(async move { self.status(show).await }),
            Order::TorOn => Box::pin(async move { self.tor_on().await }),
            Order::TorOff => Box::pin(async move { self.tor_off() }),
            Order::Bridge(line) => Box::pin(async move { self.bridge(line) }),
            Order::Helper(program) => Box::pin(async move { self.helper(program) }),
            Order::Leave => Box::pin(async { leave() }),
        }
    }

    /// Reach a node and exchange one heartbeat, saying what came back.
    async fn ping(&self, address: &str) -> bool {
        let Some(address) = readable(address) else {
            return false;
        };
        // Into the running node as well as onto the disk, so that the next round
        // knocks there rather than the one after the vigil next reads the disk.
        let typed = address.to_string();
        if let Err(e) = self.node.given_by_hand(&typed, None).await {
            aloud!("failed   writing down the address: {e:#}");
        }
        let knocked = crate::commands::ping::knock(
            self.node.identity(),
            &self.dialer,
            self.common.timeout,
            &address,
        );
        match knocked.await {
            Ok(answered) => {
                if let Err(e) = self.node.given_by_hand(&typed, Some(answered)).await {
                    aloud!("failed   writing down who answered: {e:#}");
                }
                true
            }
            // The failure already names the address when the address is what failed.
            Err(e) => {
                aloud!("unheard  {e:#}");
                false
            }
        }
    }

    /// Ask whoever is there to hand the file over.
    async fn join(&self, address: &str) -> bool {
        let Some(address) = readable(address) else {
            return false;
        };
        let asked =
            crate::commands::join::ask(&self.node, &self.dialer, self.common.timeout, &address);
        asked.await.map_err(|e| aloud!("unheard  {e:#}")).is_ok()
    }

    /// Begin a line of this node's own, if nobody has begun one.
    async fn bootstrap(&self, anyway: bool) -> bool {
        let place = n333_net::meeting::THE_PLACE;
        crate::commands::bootstrap::begin(&self.node, place, anyway)
            .await
            .map_err(|e| aloud!("unbegun  {e:#}"))
            .is_ok()
    }

    /// Say one of the 333 in this epoch.
    async fn say(&self, which: &str) -> bool {
        let Some(index) = crate::words::count::index(which) else {
            aloud!(
                "refused  there are {} of them, numbered 0 to {}. \"{which}\" is not one.",
                n333_core::signal::SIGNAL_COUNT,
                n333_core::signal::SIGNAL_COUNT - 1
            );
            return false;
        };
        // Saying it says its own lines, so there is nothing to add when it works.
        crate::commands::say::speak(&self.node, index)
            .await
            .map_err(|e| aloud!("{}", crate::failed::not_said(&e)))
            .is_ok()
    }

    /// Say what this node is holding.
    ///
    /// Into the vigil, in one line, when the screen asked: the dials beside it already
    /// show the rest. To another terminal, the whole of what `333 status` prints,
    /// because a terminal has no dials, and it goes to that terminal and not into the
    /// vigil, where it would be a page of text in the middle of the log.
    async fn status(&self, show: crate::commands::status::Show) -> bool {
        let Some(asker) = crate::aloud::the_asker() else {
            let roll = self.node.roll().await.len();
            let has = if self.node.subject().await.is_some() {
                "the file is here"
            } else {
                "this node has not been given the file"
            };
            aloud!("holding  {roll} of us on the roll, and {has}");
            return true;
        };
        let mut page = Vec::new();
        let now = n333_core::Epoch::now();
        let written = crate::commands::status::whole(&mut page, &self.node, now, show).await;
        let _ = asker.send(String::from_utf8_lossy(&page).trim_end().to_owned());
        written
            .map_err(|e| aloud!("unread   what this node holds: {e:#}"))
            .is_ok()
    }

    /// Raise an onion address, unless one is already up, and wait until it answers.
    async fn tor_on(&self) -> bool {
        // Subscribed before the listener starts, so the address it publishes cannot
        // arrive before anybody is looking for it.
        let mut raised = self.found_address.subscribe();
        let (fell, fallen) = oneshot::channel();
        {
            // As with the bridges: a poisoned lock still holds the listener.
            let mut unseen = self
                .unseen
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if unseen.as_ref().is_some_and(|task| !task.is_finished()) {
                aloud!("standing the unseen address is already up. `tor off` takes it down.");
                return true;
            }
            let answering = super::onion::answer(
                self.dialer.clone(),
                Arc::clone(&self.node),
                Door::new(),
                self.found_address.clone(),
            );
            let listening = async move {
                let ended = answering.await;
                if let Err(e) = &ended {
                    aloud!("unraised {e:#}");
                }
                let _ = fell.send(());
                ended
            };
            // Whoever asked hears it come up, however long that takes.
            *unseen = Some(match crate::aloud::the_asker() {
                Some(asker) => {
                    let words = crate::words::current();
                    let listening = crate::words::spoken::spoken_in(words, listening);
                    tokio::spawn(crate::aloud::heard_by(asker, listening))
                }
                None => tokio::spawn(listening),
            });
        }
        tokio::select! {
            () = onion_published(&mut raised) => true,
            _ = fallen => false,
        }
    }

    /// Stop answering on the onion address.
    fn tor_off(&self) -> bool {
        let mut unseen = self
            .unseen
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match unseen.take() {
            // One that failed to come up has already said so, and there is nothing to stop.
            Some(task) if !task.is_finished() => {
                task.abort();
                // The address it was published under is not withdrawn from anywhere. It
                // was signed and handed out, and there is no way to unsay a signed
                // statement; it stops answering, and the board forgets it two epochs
                // from now.
                aloud!(
                    "unseen   the onion address stops answering now. What was already said about\n\
                     \x20        it stands until it is forgotten, two epochs from when it was said."
                );
            }
            _ => aloud!("standing there is no unseen address up to take down."),
        }
        true
    }

    /// Add a bridge line for the next time Tor starts.
    fn bridge(&self, line: String) -> bool {
        if self.dialer.tor_is_up() {
            aloud!(
                "too late Tor is already running, and a bridge added now changes nothing about\n\
                 \x20        the connection it already made. Restart the node with it instead."
            );
            return false;
        }
        // A lock another task panicked while holding still holds the list, and the list
        // is what is wanted. Refusing here would say the bridges "could not be reached",
        // which is not a thing that happened to them.
        let mut bridges = self
            .common
            .bridges
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        bridges.lines.push(line);
        aloud!(
            "bridged  {} bridge{} will be used the next time Tor starts.",
            bridges.lines.len(),
            if bridges.lines.len() == 1 { "" } else { "s" }
        );
        true
    }

    /// Name the program that speaks an obfuscated bridge.
    fn helper(&self, program: String) -> bool {
        let mut bridges = self
            .common
            .bridges
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        aloud!("bridged  {program} will be run for any obfuscated bridge.");
        bridges.helper = Some(program);
        true
    }
}

/// Read what was typed as an address, or say why it is not one.
fn readable(typed: &str) -> Option<PeerAddress> {
    n333_net::invite::address_or_invite(typed)
        .map_err(|e| aloud!("unread   {typed} is not an address: {e}"))
        .ok()
}

/// Leaving is the screen's, and it never sends it here. Anybody else is refused.
fn leave() -> bool {
    aloud!(
        "refused  another terminal cannot end this vigil. It ends where it was started:\n\
         \x20        `q` in its screen, Ctrl-C, or the service manager that keeps it."
    );
    false
}

/// Wait until the vigil has an onion address to hand out.
async fn onion_published(raised: &mut watch::Receiver<Option<PeerAddress>>) {
    while raised.changed().await.is_ok() {
        if matches!(*raised.borrow_and_update(), Some(PeerAddress::Onion { .. })) {
            return;
        }
    }
    // The vigil is ending and nothing will be published; the listener says why.
    std::future::pending::<()>().await;
}

//! Carrying out what was typed into the screen.
//!
//! The screen reads the keys and draws; it opens nothing. This is where the dialler,
//! the node and the listeners are, so this is where an order becomes a connection.
//! Each one runs on its own task, because reaching somebody who does not answer takes
//! as long as the deadline allows and the screen must keep drawing throughout.
//!
//! EVERYTHING SAYS SOMETHING. An order that worked, an order that failed and an order
//! that was refused all put a line in the vigil, because the person typed into a
//! screen and is looking at that screen for the answer.

use std::sync::Arc;

use n333_net::PeerAddress;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::watch;
use tokio::task::JoinHandle;

use crate::commands::Common;
use crate::dial::Dialer;
use crate::node::Node;
use crate::orders::Order;

use super::door::Door;

/// Do what the screen was told to do, until the screen is gone.
pub(super) async fn until_the_screen_goes(
    mut orders: UnboundedReceiver<Order>,
    node: Arc<Node>,
    common: Common,
    dialer: Dialer,
    found_address: watch::Sender<Option<PeerAddress>>,
) {
    // The onion listener, when one has been started from here. Held so that it can be
    // stopped again: a person who turned it on wants to be able to turn it off, and a
    // task nobody holds is a task nobody can stop.
    let mut unseen: Option<JoinHandle<anyhow::Result<()>>> = None;
    while let Some(order) = orders.recv().await {
        match order {
            Order::Ping(address) => spawn_ping(&common, address),
            Order::Join(address) => spawn_join(&common, address),
            Order::Bootstrap { anyway } => spawn_bootstrap(&common, anyway),
            Order::Say(which) => say(&node, &which).await,
            Order::Status => status(&node).await,
            Order::TorOn => unseen = tor_on(unseen, &node, &dialer, &found_address),
            Order::TorOff => unseen = tor_off(unseen),
            Order::Bridge(line) => bridge(&common, &dialer, line),
            Order::Helper(program) => helper(&common, program),
            // Read by the screen itself, which stops rather than sending it here.
            Order::Leave => break,
        }
    }
}

/// Reach a node and exchange one heartbeat, saying what came back.
fn spawn_ping(common: &Common, address: String) {
    let common = common.clone();
    tokio::spawn(async move {
        match n333_net::invite::address_or_invite(&address) {
            Ok(address) => match crate::commands::ping::run(&common, &address).await {
                Ok(()) => {}
                Err(e) => aloud!("unheard  {address}: {e:#}"),
            },
            Err(e) => aloud!("unread   {address} is not an address: {e}"),
        }
    });
}

/// Ask whoever is there to hand the file over.
fn spawn_join(common: &Common, address: String) {
    let common = common.clone();
    tokio::spawn(async move {
        match n333_net::invite::address_or_invite(&address) {
            Ok(address) => {
                if let Err(e) = crate::commands::join::run(&common, &address).await {
                    aloud!("unheard  {address}: {e:#}");
                }
            }
            Err(e) => aloud!("unread   {address} is not an address: {e}"),
        }
    });
}

/// Begin a line of this node's own, if nobody has begun one.
fn spawn_bootstrap(common: &Common, anyway: bool) {
    let common = common.clone();
    tokio::spawn(async move {
        let place = n333_net::meeting::THE_PLACE.to_owned();
        if let Err(e) = crate::commands::bootstrap::run(&common, &place, anyway).await {
            aloud!("unbegun  {e:#}");
        }
    });
}

/// Say one of the 333 in this epoch.
async fn say(node: &Arc<Node>, which: &str) {
    let Ok(index) = which.parse::<u16>() else {
        aloud!(
            "refused  there are {} of them, numbered 0 to {}. \"{which}\" is not one.",
            n333_core::signal::SIGNAL_COUNT,
            n333_core::signal::SIGNAL_COUNT - 1
        );
        return;
    };
    // Saying it says its own lines, so there is nothing to add when it works.
    if let Err(e) = crate::commands::say::speak(node, index).await {
        aloud!("refused  {e:#}");
    }
}

/// Put what this node is holding into the vigil, where the person is looking.
async fn status(node: &Arc<Node>) {
    let roll = node.roll().await.len();
    let has = if node.subject().await.is_some() {
        "the file is here"
    } else {
        "this node has not been given the file"
    };
    aloud!("holding  {roll} of us on the roll, and {has}");
}

/// Raise an onion address, unless one is already up.
fn tor_on(
    unseen: Option<JoinHandle<anyhow::Result<()>>>,
    node: &Arc<Node>,
    dialer: &Dialer,
    found_address: &watch::Sender<Option<PeerAddress>>,
) -> Option<JoinHandle<anyhow::Result<()>>> {
    if unseen.as_ref().is_some_and(|task| !task.is_finished()) {
        aloud!("standing the unseen address is already up. `tor off` takes it down.");
        return unseen;
    }
    Some(tokio::spawn(super::onion::answer(
        dialer.clone(),
        Arc::clone(node),
        Door::new(),
        found_address.clone(),
    )))
}

/// Stop answering on the onion address.
fn tor_off(
    unseen: Option<JoinHandle<anyhow::Result<()>>>,
) -> Option<JoinHandle<anyhow::Result<()>>> {
    match unseen {
        Some(task) => {
            task.abort();
            // The address it was published under is not withdrawn from anywhere. It was
            // signed and handed out, and there is no way to unsay a signed statement;
            // it stops answering, and the board forgets it two epochs from now.
            aloud!(
                "unseen   the onion address stops answering now. What was already said about\n\
                 \x20        it stands until it is forgotten, two epochs from when it was said."
            );
            None
        }
        None => {
            aloud!("standing there is no unseen address up to take down.");
            None
        }
    }
}

/// Add a bridge line for the next time Tor starts.
fn bridge(common: &Common, dialer: &Dialer, line: String) {
    if dialer.tor_is_up() {
        aloud!(
            "too late Tor is already running, and a bridge added now changes nothing about\n\
             \x20        the connection it already made. Restart the node with it instead."
        );
        return;
    }
    match common.bridges.lock() {
        Ok(mut bridges) => {
            bridges.lines.push(line);
            aloud!(
                "bridged  {} bridge{} will be used the next time Tor starts.",
                bridges.lines.len(),
                if bridges.lines.len() == 1 { "" } else { "s" }
            );
        }
        Err(_) => aloud!("unheard  the bridges could not be reached to add to"),
    }
}

/// Name the program that speaks an obfuscated bridge.
fn helper(common: &Common, program: String) {
    match common.bridges.lock() {
        Ok(mut bridges) => {
            aloud!("bridged  {program} will be run for any obfuscated bridge.");
            bridges.helper = Some(program);
        }
        Err(_) => aloud!("unheard  the bridges could not be reached to add to"),
    }
}

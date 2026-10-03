### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph. The keys are the command, then the flag, as clap names them.

help-about = One node of 333. It answers when asked, keeps its record, and passes
    the file on.

help-id = Show this node's name, making one on first run

help-bootstrap = Start a new line, when there is nobody to be handed the file by
help-bootstrap-long = Start a new line, when there is nobody to be handed the file by.

    The ordinary way in is `333 join` with an invitation. This looks at the
    meeting point first and refuses if anybody is there. If nobody is, it
    fetches the file, checks it against the hash this client carries, and
    writes it down. Your node is then the founder of its own line, with
    nobody's signature on its beginning, and anybody reading its record
    can see that.

help-serve = Run this node in this terminal until you stop it
help-serve-long = Run this node in this terminal until you stop it.

    It answers heartbeats and questions, trades what it knows, and asks the
    nodes it is drawn to ask at every epoch. On a terminal it opens the
    screen; `q`, Ctrl-C or `333 stop` from another terminal stops it.
    `333 start` runs the same thing in the background instead.

help-serve-long-light = Run this node in this terminal until you stop it.

    It answers heartbeats and questions, trades what it knows, and asks the
    nodes it is drawn to ask at every epoch, one line at a time. Ctrl-C or
    `333 stop` from another terminal stops it. `333 start` runs the same
    thing in the background instead.

help-say = Say one of the 333, once an epoch. What travels is the number

help-status = Show whether this node is running, where others can reach it, and
    how many of us are answering

help-join = Be handed the file by a node that has it, with an invitation

help-languages = List the languages, or save one for every command at this node

help-ping = Reach another node and exchange one heartbeat with it

help-pack = Write this node into one file, to carry it to another machine
help-pack-long = Write this node into one file, to carry it to another machine.

    Everything goes: its name, its record, what others signed about it, the
    file, and the key to its onion address. Afterwards this directory
    refuses to run it, so the name is never in two places. The file is not
    encrypted: whoever holds it is this node. Carry it, unpack it, delete
    it.

help-unpack = Put a packed node into this machine's node directory
help-unpack-long = Put a packed node into this machine's node directory.

    Refused where a node already lives. Nothing is written until the file
    has been read through and its key and record check out.

help-moved = Say that this node's directory was moved or renamed, not copied
help-moved-long = Say that this node's directory was moved or renamed, not copied.

    A node that finds itself somewhere new says so on every run until this
    is typed, because a copy with the original still running would be one
    name in two places.

help-tell = Hand a running node an order, in its screen's words
help-tell-long = Hand a running node an order, in its screen's words.

    `tor on`, `tor off`, `bridge <line>`, `helper <program>`, and every
    other word the screen takes after `:`. The running node carries it out
    and its answer is printed here. `say`, `join`, `ping`, `begin`,
    `status` and `stop` reach a running node the same way without this.

help-tell-light = Hand a running node an order
help-tell-long-light = Hand a running node an order.

    `tor on`, `tor off`, `bridge <line>` and `helper <program>`. The
    running node carries it out and its answer is printed here. `say`,
    `join`, `ping`, `begin`, `status` and `stop` reach a running node the
    same way without this.

help-service = Manage the background service directly (`start` and `stop` use it)
help-service-long = Manage the background service directly (`start` and `stop` use it).

    Nothing is installed until you ask, every file written and command run
    is printed as it happens, and `333 service uninstall` removes all of
    it.

help-service-install = Install the background service with these run flags, and start it
help-service-install-long = Install the background service with these run flags, and start it.

    The service runs `333 run` with exactly the flags given, for this
    node's directory, and an hourly check beside it says so on this machine
    if the node stops. `333 start` does the same with no flags.

help-service-uninstall = Stop the background service and remove everything it installed

help-service-status = What the service manager says, when the node last said it was awake,
    and the last lines it wrote

help-service-check = Say so on this machine if the node has stopped. The service runs this
    every hour; it says nothing when all is well

help-data-dir = Directory holding everything this node owns: its name, and Tor's state
    if it uses Tor

help-timeout = Seconds to wait for any single step that talks to the network
help-timeout-long = Seconds to wait for any single step that talks to the network.

    A ceiling, not a delay. It is sized for starting Tor, the one step that
    can take minutes.

help-dangerously-trust-directory-permissions = Accept a directory that others on this machine can enter
help-dangerously-trust-directory-permissions-long = Accept a directory that others on this machine can enter.

    The directory holds the only copy of this node's name, so a loosely
    permissioned one is refused by default. This is for scratch directories
    and containers with odd ownership.

help-keep-everything = Keep every statement for ever, instead of the window standing is read
    over
help-keep-everything-long = Keep every statement for ever, instead of the window standing is read
    over.

    It changes nothing about anybody's standing: every statement verifies
    the same wherever it is kept.

help-bridges = A bridge line, for a network that blocks the ordinary way into Tor
help-bridges-long = A bridge line, for a network that blocks the ordinary way into Tor.

    Give it once for each bridge you were handed, exactly as it was handed
    to you. Nothing here fetches bridges: they are given out by people, on
    purpose, so that no list can simply be collected and blocked.

help-bridge-helper = The program that speaks an obfuscated bridge, by name or by path
help-bridge-helper-long = The program that speaks an obfuscated bridge, by name or by path.

    Only needed when a bridge line asks for one and it is not `lyrebird` on
    the path. It is not bundled, because a frozen copy would soon be the
    wrong one.

help-language = The language to speak, as a tag: `ko`, `es`, `zh-Hant`
help-language-long = The language to speak, as a tag: `ko`, `es`, `zh-Hant`.

    Without it, `THE333_LANGUAGE`, then the language `333 language <TAG>`
    saved, then English. The system's locale is not used. `333 language`
    lists the languages there are words for, and a folder of catalogs in
    `<data-dir>/words/<tag>/` adds one without building anything. The 333
    words themselves are never translated.

help-count-in = Count in ten, twelve, or twelve-ascii
help-count-in-long = Count in ten, twelve, or twelve-ascii.

    Every count shown is written in it and every number typed is read in
    it: `say 238` in twelve is `say 332` in ten. Names, addresses, ports
    and versions are never re-counted, and nothing on the wire changes.
    Without it, `THE333_COUNT_IN`, then ten.

help-bootstrap-meet = Where to look for people before beginning on your own

help-bootstrap-anyway = Begin even though somebody is already there

help-serve-bind = Address and port to listen on

help-serve-tor = Also raise an onion address, so others can reach this node without
    learning where it is. Waking Tor takes seconds to minutes

help-serve-no-direct = Do not open a socket at all. Only with --tor; it keeps your address
    off the wire entirely

help-serve-announce = The address to tell other nodes to reach this one at
help-serve-announce-long = The address to tell other nodes to reach this one at.

    Needed when the socket cannot say it: listening on every interface, or
    behind something that forwards a port.

help-serve-no-mdns = Do not say on the local network that this node is here
help-serve-no-mdns-long = Do not say on the local network that this node is here.

    What goes out otherwise is that something on this machine speaks 333
    and on which port, not this node's name. It is how two nodes in one
    house find each other without an invitation.

help-serve-no-router = Do not ask the router to send the port to this machine
help-serve-no-router-long = Do not ask the router to send the port to this machine.

    A home router drops what nobody inside asked for until a program inside
    asks it to forward a port, over UPnP-IGD, PCP or NAT-PMP. It changes
    the network, so it is printed when it happens. `--no-upnp` is the older
    name for this.

help-serve-meet = Where to look for nodes nobody introduced this one to
help-serve-meet-long = Where to look for nodes nobody introduced this one to.

    One fixed address holding signed statements about where nodes are.
    Everything read there is verified here.

help-serve-no-meet = Do not use a meeting point at all
help-serve-no-meet-long = Do not use a meeting point at all.

    This node is then reachable by whoever was handed an invitation and by
    nodes on this network, and by nobody else.

help-serve-plain = Say the lines instead of drawing the screen
help-serve-plain-long = Say the lines instead of drawing the screen.

    Off a terminal it always says the lines; this asks for that on a
    terminal too.

help-serve-plain-light = Say the lines, which this edition always does
help-serve-plain-long-light = Say the lines, which this edition always does.

    This edition has no screen. The flag is accepted so that one command
    line works in either edition.

help-say-index = Which of them, from 0 to { $last }, typed in the base this counts in
    (--count-in). The words are not written yet

help-status-sources = List every address this node holds: whose it is, where and when it was
    first heard of, and where last

help-status-json = What this node observed, as JSON for a program to read. No address or
    port is in it

help-join-address = An invitation (`333:host:port`) from somebody who already has it

help-ping-address = An invitation (`333:host:port`), or an address: `host`, `host:port`,
    `[::1]:port` or `something.onion` (reached through Tor)

help-pack-file = The file to write. It must not exist yet

help-pack-undo = Take back a packing here, for a move that was abandoned
help-pack-undo-long = Take back a packing here, for a move that was abandoned.

    Only if the file was never unpacked anywhere: if it was, this makes
    two.

help-unpack-file = The file `333 pack` wrote

help-tell-order = The order, as it would be typed into the screen

help-tell-order-light = The order, as `tor on` or `bridge <line>` is written

help-service-install-flags = The flags for `run`, as you would type them after it

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = Print help
help-print-help-more = Print help (see more with '--help')
help-print-help-summary = Print help (see a summary with '-h')
help-print-version = Print version
help-print-this = Print this message or the help of the given subcommand(s)
help-print-for = Print help for the subcommand(s)

help-start = Run this node in the background, now and after every reboot

help-stop = Stop this node, and keep it stopped after a reboot

help-restart = Stop this node, then run it in the background again

help-logs = Show the last lines this node wrote while it ran in the background

help-logs-follow = Keep showing new lines as they come, where systemd keeps them

help-invite = Show the invitation others use to join through this node

help-status-all = Show everything this node knows, with what each part means

help-languages-tag = The language to save, as a tag: `ko`, `en`. `en` goes back to English

help-start-example = Example: 333 start

help-stop-example = Example: 333 stop

help-restart-example = Example: 333 restart

help-status-example = Example: 333 status --all

help-logs-example = Example: 333 logs -f

help-id-example = Example: 333 name

help-invite-example = Example: 333 invite

help-bootstrap-example = Example: 333 begin

help-serve-example = Example: 333 run --tor

help-say-example = Example: 333 say 7

help-join-example = Example: 333 join 333:192.0.2.7:3333

help-languages-example = Example: 333 language ko

help-ping-example = Example: 333 ping 333:192.0.2.7:3333

help-pack-example = Example: 333 pack node.333

help-unpack-example = Example: 333 unpack node.333

help-moved-example = Example: 333 moved

help-tell-example = Example: 333 tell tor on

help-service-example = Example: 333 service status

help-service-install-example = Example: 333 service install --tor

help-service-uninstall-example = Example: 333 service uninstall

help-service-status-example = Example: 333 service status

help-service-check-example = Example: 333 service check

help-start-flags = The flags for `run`, as you would type them after it. Kept for every
    later start until other flags are given

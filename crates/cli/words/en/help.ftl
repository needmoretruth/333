### `333 --help`: what each command and each flag is for.
###
### A line break inside a paragraph is read as a space; a blank line begins a new
### paragraph. The keys are the command, then the flag, as clap names them.

help-about = One node of 333. It keeps the hours, answers when asked, and passes the
    file on.

help-id = Show this node's name, asking for one on first run

help-bootstrap = Begin a line of your own, when there is nobody to be given the file by
help-bootstrap-long = Begin a line of your own, when there is nobody to be given the file by.

    The ordinary way in is an invitation from somebody who already has the
    file, and this is not that. It looks at the meeting point first, and if
    anybody is there it tells you to go and join them instead. If nobody
    is, it fetches the file, checks it against the hash this client
    carries, and writes it down. Your node is then the start of its own
    line and nobody signed for it, which anybody reading your record can
    see.

help-serve = Keep the vigil: answer whoever asks, until interrupted
help-serve-long = Keep the vigil: answer whoever asks, until interrupted.

    This is what a node does almost all of the time. It answers heartbeats
    and challenges, trades what it knows with whoever it can reach, and at
    every epoch boundary asks the ones it was drawn to ask. On a terminal
    it opens the screen.

help-say = Speak one of the 333, once in this epoch. What travels is the number

help-status = Show what this node has seen: how many of us are answering, where this
    node stands over the window, and how much of the silence is left if it
    has begun

help-join = Ask a node that has the file to hand it over. Write the file yourself
    and you hold a file: you are one of us from the moment somebody gives
    it to you and you both sign for it

help-languages = List the languages there are words for, and how much of each is written

help-ping = Knock on another node, and exchange one heartbeat with it

help-pack = Write this node into one file, to carry it to another machine
help-pack-long = Write this node into one file, to carry it to another machine.

    Everything it is goes: its name, its record, what others signed about
    it, the file if it holds it, and the key to its onion address.
    Afterwards this directory refuses to act as it, because one name in two
    places is a node contradicting itself. The file is not encrypted:
    whoever holds it is this node, so carry it, unpack it, and delete it.

help-unpack = Put a packed node into this machine's node directory
help-unpack-long = Put a packed node into this machine's node directory.

    Refused where a node already lives, and it says what that node holds.
    Nothing is written until the file has been read through and its key and
    record check out.

help-moved = Say that this node's directory was moved or renamed, not copied
help-moved-long = Say that this node's directory was moved or renamed, not copied.

    A node that finds itself somewhere other than where it was says so on
    every run until this is typed, because a copy with the original still
    running is one name in two places, and from inside the directory the
    two look the same.

help-tell = Tell the vigil running in this directory something, in its screen's
    words
help-tell-long = Tell the vigil running in this directory something, in its screen's
    words.

    `tor on`, `tor off`, `bridge <line>`, `helper <program>`, and every
    other word the screen takes after `:`. The vigil carries it out and
    what it says about it is printed here. `say`, `join`, `ping`,
    `bootstrap` and `status` are handed to a running vigil the same way
    without this.

help-service = Keep the vigil through logouts and reboots, with this system's service
    manager
help-service-long = Keep the vigil through logouts and reboots, with this system's service
    manager.

    Nothing is installed until you ask for it here, everything that is done
    is said as it is done, and `333 service uninstall` undoes it.

help-service-install = Keep this node's vigil through logouts and reboots, with this system's
    own service manager
help-service-install-long = Keep this node's vigil through logouts and reboots, with this system's
    own service manager.

    Give it the flags you would give `serve`: the service runs `serve` with
    exactly those, for this node's directory, and an hourly check beside it
    says so on this machine when the vigil stops. Every file it writes and
    every command it runs is said as it happens, and `333 service
    uninstall` undoes all of it.

help-service-uninstall = Stop the vigil's service, and remove everything `service install` wrote

help-service-status = What the service manager says of the vigil, when the vigil last said it
    was awake, and the last things it said

help-service-check = Say so on this machine if the vigil is not being kept. The service runs
    this every hour; it says nothing when all is well

help-data-dir = Directory holding everything this node owns: its name, and Tor's state
    if it uses Tor

help-timeout = Seconds to wait for any single step that talks to the network
help-timeout-long = Seconds to wait for any single step that talks to the network.

    A ceiling rather than a delay. A direct connection is done in
    milliseconds and fails on its own; this is sized for a Tor bootstrap,
    which is the one step here that can legitimately take minutes.

help-dangerously-trust-directory-permissions = Accept a directory that others on this machine can enter
help-dangerously-trust-directory-permissions-long = Accept a directory that others on this machine can enter.

    Both this client and arti refuse to start on a loosely permissioned
    directory, which is the right default: that directory holds the only
    copy of this node's name. The flag exists for scratch directories and
    containers with odd ownership, and it does what its name says.

help-keep-everything = Keep every statement for ever, instead of the window standing is read
    over
help-keep-everything-long = Keep every statement for ever, instead of the window standing is read
    over.

    It confers nothing. Every statement carries its own signature and
    verifies the same wherever it was kept, so there is no archive of
    record and nobody becomes an archivist by doing this. It is for people
    who would rather the bytes still existed somewhere, which nothing here
    requires of anyone.

help-bridges = A bridge line, for a network that blocks the ordinary way into Tor
help-bridges-long = A bridge line, for a network that blocks the ordinary way into Tor.

    Give it once for each bridge you were handed, exactly as it was handed
    to you. Nothing here fetches bridges for you: they are scarce and they
    are given out by people, slowly and on purpose, because a list that
    could simply be collected would simply be blocked. Without any of these
    Tor is reached the ordinary way, which is what almost everybody wants.

help-bridge-helper = The program that speaks an obfuscated bridge, by name or by path
help-bridge-helper-long = The program that speaks an obfuscated bridge, by name or by path.

    Only needed when a bridge line asks for one, and only when it is not
    called `lyrebird` or is not on the path. It is not bundled: it is a
    separate program chasing a moving target, and a copy frozen inside this
    would be the wrong copy within a year while looking like the right one.

help-language = The language to speak, as a tag: `ko`, `es`, `zh-Hant`
help-language-long = The language to speak, as a tag: `ko`, `es`, `zh-Hant`.

    Without it, `THE333_LANGUAGE`, then the system's locale, then English.
    `333 languages` lists the languages there are words for, and a folder
    of catalogs in `<data-dir>/words/<tag>/` adds one without building
    anything. The 333 words themselves are never translated.

help-count-in = Count in ten, twelve, or twelve-ascii
help-count-in-long = Count in ten, twelve, or twelve-ascii.

    Every count shown is written in it, and every number typed is read in
    it: `say 238` in twelve is the same signal as `say 332` in ten. Names,
    addresses, ports and versions are never re-counted, and nothing on the
    wire changes. Twelve is written with ↊ and ↋, or with X and E where the
    terminal cannot show them. Without it, `THE333_COUNT_IN`, then ten.

help-bootstrap-meet = Where to look for people before beginning on your own

help-bootstrap-anyway = Begin even though somebody is already there

help-serve-bind = Address and port to listen on

help-serve-tor = Also raise an onion address, so others can reach this node without
    learning where it is. Waking Tor takes seconds to minutes

help-serve-no-direct = Do not open a socket at all. Only useful with --tor, and the only way
    to keep the vigil with your address nowhere on the wire

help-serve-announce = The address to tell other nodes to reach this one at
help-serve-announce-long = The address to tell other nodes to reach this one at.

    Needed when the socket cannot say: listening on every interface, or
    behind something that forwards a port. Without it a node on a wildcard
    bind can answer whoever finds it and can never be found.

help-serve-no-mdns = Do not say on the local network that this node is here
help-serve-no-mdns-long = Do not say on the local network that this node is here.

    What goes out otherwise is that something on this machine speaks 333
    and on which port — not this node's name — which is what a port scan of
    the same network would find anyway. It is how two nodes in one house
    find each other with nobody typing an invitation. A node listening only
    through Tor never does this at all.

help-serve-no-router = Do not ask the router to send the port to this machine
help-serve-no-router-long = Do not ask the router to send the port to this machine.

    Asking is what makes a socket on a home connection answer anybody: the
    router in front of it drops what nobody inside asked for until a
    program on the inside asks it not to, over UPnP-IGD, PCP or NAT-PMP,
    one of which most of them already speak. It is a real change to
    somebody's network, so it is said out loud when it is made and this
    refuses it outright. `--no-upnp` is the older name for the same thing,
    from when UPnP was all that was asked.

help-serve-meet = Where to look for nodes nobody introduced this one to
help-serve-meet-long = Where to look for nodes nobody introduced this one to.

    One fixed address holding signed statements about where nodes say they
    are. Everything read there is verified here, and nothing there is
    believed. It is the only way two machines on two networks meet without
    somebody handing over an invitation.

help-serve-no-meet = Do not use a meeting point at all
help-serve-no-meet-long = Do not use a meeting point at all.

    This node is then reachable by whoever was handed an invitation and by
    nodes on this network, and by nobody else.

help-serve-plain = Say the lines instead of drawing the screen
help-serve-plain-long = Say the lines instead of drawing the screen.

    The screen is what this client does on a terminal. Anywhere else — a
    pipe, a service manager's log, a file — it says the lines instead, and
    this flag asks for that on a terminal too.

help-say-index = Which of them, from 0 to { $last }, typed in the base this counts in
    (--count-in). The words are not written yet

help-status-sources = List every address this node holds: whose it is, where it was first
    heard of and when, and where it was heard of last. The addresses are
    printed; this is your own node's disk and nobody else's

help-status-json = Say what this node observed as JSON, for a program to read. No address,
    onion address or port of any kind is in it

help-join-address = An invitation (`333:host:port`) from somebody who already has it

help-ping-address = An invitation (`333:host:port`), or an address typed by hand as `host`,
    `host:port`, `[::1]:port` or `something.onion`. An onion address is
    reached through Tor; everything else directly

help-pack-file = The file to write. It must not exist yet

help-pack-undo = Take back a packing here, for a move that was abandoned
help-pack-undo-long = Take back a packing here, for a move that was abandoned.

    Only if the file was never unpacked anywhere: if it was, this makes
    two.

help-unpack-file = The file `333 pack` wrote

help-tell-order = The order, as it would be typed into the screen

help-service-install-flags = The flags for `serve`, as you would type them after it

## What clap says about its own `--help`, `--version` and `help`.

help-print-help = Print help
help-print-help-more = Print help (see more with '--help')
help-print-help-summary = Print help (see a summary with '-h')
help-print-version = Print version
help-print-this = Print this message or the help of the given subcommand(s)
help-print-for = Print help for the subcommand(s)

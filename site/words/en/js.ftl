# What the pages' scripts say. Each page carries all of these as JSON, and the server
# writes the same words wherever it puts the same thing into a page before any script
# runs.
#
# A message here may hold { $variables } and at most one selector on a variable, and
# nothing else: the scripts fill these in themselves, without Fluent.

## Every page

js-copy = Copy
js-copied = Copied
js-selected = Selected
js-state-awake = This site's node is awake
js-state-not-running = This site's node is not running
js-in-hours = in { $h } h { $m } min
js-in-minutes = in { $m } min
# Beside the epoch number: which epoch of this line it is, counted from the epoch the
# line's founder first handed the file on, which is the first.
js-line-epoch = this line's { NUMBER($n, type: "ordinal") ->
    [one] { $n }st
    [two] { $n }nd
    [few] { $n }rd
   *[other] { $n }th
  } epoch

## The network

js-network-state-founder = Not on any roll
js-network-state-ok = Answering this epoch
js-network-state-quiet = Silent this epoch
js-network-state-later = Counted from a later epoch
js-network-state-seen = Seen, not on the roll
js-network-awake = Awake
js-network-not-running = Not running
# Under a node in the graph, once its owner has marked it.
js-network-yours-label = { $name } · yours
js-network-find-bad = A node's name is hexadecimal, at least the first 6 characters of it.
js-network-find-none = This site's node has not seen a node with that name.
js-network-find-many = { $count } nodes begin with that. Type more of the name.
js-network-find-marked = Marked as yours on this device.
js-network-select = Select a node to see what this site's node knows about it.
js-network-role-founder = Founder of this line
js-network-role-site = This site's node
js-network-role-yours = Yours, on this device
js-network-role-none = A node on the network
js-network-col-name = Name
js-network-col-state = This epoch
js-network-col-given = Given the file
js-network-col-counted = Counted from
js-network-col-answered = Last answered
js-network-col-said = Said
js-network-col-reached = Reached
js-network-row-said = Said this epoch
js-network-row-handed = Handed the file to
js-network-row-testimony = Testimony
js-network-given-by = epoch { $epoch }, by { $sponsor }
js-network-given-founder = Nobody. It began this line.
js-network-given-none = Not on the roll
js-network-epoch = epoch { $epoch }
js-network-epoch-now = { $epoch } (this epoch)
js-network-epoch-ago = { $epoch } ({ $ago } ago)
js-network-more = and { $count } more in the table below
js-network-nothing = Nothing
js-network-reach-direct = Directly
js-network-reach-tor = Through Tor
js-network-reach-tor-short = Tor
js-network-reach-unknown = Not known
js-network-testimony = asked by { $asked }, asked { $asking } (last 3 epochs)
js-network-copy-name = Copy name
js-network-select-name = Select the name above
js-network-mine = This is my node
js-network-tag-founder = founder
js-network-tag-site = this site
js-network-tag-yours = yours
js-network-empty = No other node has been seen by this site's node yet.
js-network-this-node = This node
js-network-yes = Yes
js-network-no = No
js-network-none = None

## Where we are

js-map-watch = Watch it live
js-map-stop = Stop watching
js-map-read-at = Read at { $read_at } UTC.
js-map-unreadable = The board could not be read just now.
js-map-tor = Tor
js-map-nowhere = Nowhere the edge could place
js-map-nobody = Nobody is saying where they are.
js-map-all = All of us saying
# Nodes this site's node knows of, on the roll or its founder, that left no statement on
# the board; with the row above, the count the network page shows.
js-map-unsaid = On the network, not saying where
js-map-dot = { $count ->
    [one] { $count } node
   *[other] { $count } nodes
  }

## The board

js-board-said = Said in epoch { $epoch } by { $node }
js-board-site = this site's node
js-board-tor = through Tor

## Take the program

js-start-machine-linux-x86_64 = Linux on x86-64
js-start-machine-linux-aarch64 = Linux on 64-bit ARM
js-start-machine-linux-armv6 = Linux on 32-bit ARM
js-start-machine-macos-aarch64 = a Mac with Apple silicon
js-start-machine-macos-x86_64 = a Mac with an Intel chip
js-start-machine-windows-x86_64 = Windows
js-start-phone = This looks like a phone or a tablet, and the program is for a computer that stays on. Choose that computer here.
js-start-unknown = This browser does not say what it is running on. Choose your machine here.
js-start-sure = This browser says it is on { $machine }, so that is chosen here.
js-start-mac = This browser says it is on a Mac and not which chip, so Apple silicon is chosen here. The installer asks the machine itself.
js-start-linux = This browser says it is on Linux and not which processor, so x86-64 is chosen here. The installer asks the machine itself.
js-start-chosen = Chosen above

## The story on the home page, drawn

js-story-file = 333.txt · 3 bytes
js-story-gave = I gave it to you
js-story-received = I received it from you
js-story-signed = signed
js-story-minutes = 333 min
js-story-epochs = 333 epochs
js-story-now = now
js-story-answering = answering
js-story-roll = on the roll
js-story-years = { $years } years

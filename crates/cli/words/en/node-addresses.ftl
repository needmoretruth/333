### Where the others are, and another copy of this node's name.

node-addresses-reading = reading an address
node-addresses-keeping = keeping an address
node-addresses-reading-own = reading this node's address
node-addresses-keeping-own = keeping this node's address

node-addresses-another-copy = copy of this node's name is out there. A statement signed with this
    node's key, which this node never made, says it is at
    { $address }, in epoch { $said_in }.
    It arrived { $from }. Either this directory was copied and the copy was
    started, or somebody else has the key. Two nodes on one name
    contradict each other in every epoch either is asked about. Stop
    one of them; `333 pack` is how a node moves. This one keeps
    running until you decide which.
    .keyword = another

node-addresses-unread = the note of where each address came from could not be read, so a
    new one begins. Nothing this node decides reads it.
    .keyword = unread

node-addresses-copies = { $copies ->
        [one] a statement
       *[other] { $copies } statements
    } signed with this node's key, which it did not make,
    reached it in the window. Another copy of this name has been
    running. `333 status` says where it said it was.
    .keyword = another

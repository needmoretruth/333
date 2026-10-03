### `333 invite`: the line others use to join through this node.

invite-line = { $invitation }
    .keyword = invite

invite-how = give this line to the other person. They run `333 join { $invitation }`
    on their machine.
    .keyword = how

invite-none = none yet. This node has not found an address others can reach.
    .keyword = invite

invite-none-next = `333 start` runs it; ask again after a few minutes. Behind a router
    nobody opened, `333 service install --tor` runs it with an onion
    address, which needs no router.
    .keyword = next

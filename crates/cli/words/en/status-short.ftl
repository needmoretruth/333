### `333 status` with no flags: where this node can be reached, and the epoch.

status-short-address = { $address }
    .keyword = address

status-short-invite = { $invitation }
    .keyword = invite

status-short-unreachable = none yet. It finds one while it runs; `333 invite` says more.
    .keyword = address

status-short-epoch = { $epoch }, ends { $ends }, in { $left }
    .keyword = epoch

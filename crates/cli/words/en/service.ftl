### `333 service`: keeping the vigil through logouts and reboots, when asked to.
##
## A line here ends in `{` and the next begins with `""}` where the printed line is
## wider than a line of this file may be; the placeable spanning the two prints no
## line break.

service-mind = { $node } is somewhere this system empties, and this node's name is kept
    nowhere else. The service keeps the vigil there until it is emptied.
    .keyword = mind

service-runs = { $command }
    .keyword = vigil

service-undo-partial = `333 service uninstall` removes whatever of this was done.
    .keyword = undo

service-no-receipt-directory = this system names no configuration directory to keep the {
    ""}receipt in

service-wrote-receipt = { $path }, which is how `333 service uninstall` knows what to undo.
    .keyword = wrote

service-undo = `333 service uninstall` stops the vigil and undoes all of the above.
    The node's own directory is not touched by either.
    .keyword = undo

service-uninstalled = no longer kept by a service. { $node } is left as the vigil left it:
    `333 serve` keeps the vigil by hand, and `333 service install` sets
    the service up again.
    .keyword = vigil

service-none-installed = none was installed by `333 service install` for this user.
    .keyword = service

service-state = { $state }
    .keyword = service

service-node = { $node }
    .keyword = node

service-last-awake = said so last at { $at }, { $ago } ago
    .keyword = awake

service-never-awake = never said so, in this directory
    .keyword = awake

service-said-nothing = nothing that was kept
    .keyword = said

service-said-last = { $lines ->
        [one] the last line:
       *[other] the last { $lines } lines:
    }
    .keyword = said

service-no-manager = this system has no service manager `333 service` knows how to ask. {
    ""}`333 serve --plain` keeps the vigil under whatever keeps programs {
    ""}running here.

service-not-installed-here = not installed: there is no service manager here this knows

# Said by every service manager's own file.

service-creating = creating { $path }
service-writing = writing { $path }
service-removing = removing { $path }

service-wrote = { $path }
    .keyword = wrote

service-removed = { $path }
    .keyword = removed

service-left = { $path }. `333 service install` did not write it.
    .keyword = left

service-failed = { $why }
    .keyword = failed

service-not-installed = not installed
service-running = running
service-starting = starting

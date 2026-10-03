### What a command says when another 333 already has this node's directory.

elsewhere-the-vigil = the node running in this directory
elsewhere-the-vigil-by-number = the node running in this directory (process { $pid })
elsewhere-another = another 333
elsewhere-another-by-number = another 333 (process { $pid })

elsewhere-done = carried out by { $vigil }.
    .keyword = done

elsewhere-failed = { $vigil } did not do that.
    .keyword = failed

elsewhere-finding-its-name = { $who } is still finding
    this node's name. Run this again when it has one.
    .keyword = busy

elsewhere-already-keeping = { $who } is already running here, and one directory is one
    node. It can be told things from here: `333 say 7`,
    `333 join <invitation>`, `333 tell 'tor on'`, `333 stop`. A second
    node needs a directory of its own, given with --data-dir.
    .keyword = busy

elsewhere-keeping = { $who } is running here, and
    { $why }
    .keyword = busy

elsewhere-nobody-to-tell = no node is running in this directory, so there is nobody to tell.
    `333 start` or `333 run` starts it, and then this works.
    .keyword = unheard

elsewhere-busy = { $who } has this node's
    directory and is not a running node that can be handed this. Nothing
    here was read or written. Run this again when it has finished.
    .keyword = busy

elsewhere-busy-cannot-be-handed = { $who } has this node's
    directory. On this system a running 333 cannot be handed anything
    from another terminal yet, so nothing here was read or written. Type
    it into that one's screen after `:`, or stop it and run this again.
    .keyword = busy

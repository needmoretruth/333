### `333 pack`: writing this node into one file, to be carried to another machine.

pack-name-the-file = name the file to pack this node into: 333 pack <FILE>

pack-no-node = there is no node in { $root } to pack. Nothing was written.

# A term (a name that starts with `-`) goes on with a line that has to stay one
# line and is wider than a line of a catalog.
pack-already-exists = { $file } already exists. { -pack-never-over }
-pack-never-over = Packing writes a new file and never over an old one; name another.

pack-not-marked = { $file } was written, and this directory could not be marked as packed. { -pack-until-it-is }
-pack-until-it-is = Until it is, this node lives in both: { -pack-delete-that-file }
-pack-delete-that-file = delete that file before anything runs here.

pack-creating = creating { $file }

pack-name = { $name }
    .keyword = name

pack-record-none = none yet
    .keyword = record

pack-record = { $epochs ->
        [one] { $epochs } epoch, going with it
       *[other] { $epochs } epochs, going with them
    }
    .keyword = record

pack-witnessed = { $statements } statements other keys signed about it, going with it
    .keyword = witness

pack-holding = the file, going with it
    .keyword = holding

pack-onion-key = the key to its onion address, so the address goes with it
    .keyword = unseen

pack-carrying = this node, into { $file }.
    That file IS this node: whoever holds it can answer as this name.
    Carry it, unpack it, then delete it; it is not a backup to keep.
    It is not encrypted, because a password would be one more thing to
    lose, and losing it would lose the name as surely as losing the file.
    It is readable by you alone, as this directory is.
    .keyword = carrying

pack-packed = { $bytes } bytes: the files as they are, and half a kilobyte for each.
    nothing in { $root } will act as this node again.
    .keyword = packed

pack-next = on the other machine: 333 unpack { $carried }
    if the move is abandoned: { $undo }
    .keyword = next

pack-not-packed = this node was not packed, so there is nothing to undo in { $root }
    .keyword = here

pack-restored = this node lives in { $root } again.
    The file it was packed into is still this name. If it was unpacked
    anywhere, one of the two has to go before either runs; if it was not,
    delete { $file }
    .keyword = restored

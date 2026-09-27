### `333 unpack`: putting a packed node into this machine's node directory.

# A term (a name that starts with `-`) goes on with a line that has to stay one
# line and is wider than a line of a catalog.
-unpack-nothing = Nothing was unpacked.
-unpack-so-nothing = so nothing was unpacked.

unpack-kept = a node already lives in this directory. To unpack beside it
    instead, give it a directory of its own with --data-dir.

unpack-seed-in = the { $seed } in { $file }

unpack-not-one-node = that file says it holds { $claimed }, and the key inside it is { $name }. { -unpack-not-one }
-unpack-not-one = It is not one node, and nothing was unpacked.

unpack-taken = another 333 took the directory this was unpacking into. { -unpack-nothing }

unpack-elsewhere = 333 --data-dir <another directory> unpack { $file }

unpack-could-not-open = a node already lives in { $target }, and it { -unpack-could-not-be-opened } To unpack beside it instead: { $elsewhere }
-unpack-could-not-be-opened = could not be opened to say what it holds. { -unpack-nothing }

unpack-no-record = no record yet
unpack-epochs-of-record = { $epochs ->
        [one] { $epochs } epoch of record
       *[other] { $epochs } epochs of record
    }
unpack-holding = holding the file
unpack-not-holding = not holding the file

unpack-occupied = a node already lives in { $target }:
    { $name }, { $epochs }, { $holding }.
    Unpacking over it would lose all of that for good, { -unpack-so-nothing }
    To unpack beside it instead, give it a directory of its own:
    { $elsewhere }

unpack-holds-files = { $target } holds files and no node. { -unpack-its-own } To unpack elsewhere: { $elsewhere }
-unpack-its-own = A node is unpacked into a directory of its own, { -unpack-so-nothing }

unpack-opening-the-record = opening the record
unpack-torn = the record in that file is torn, so it is not a whole node. { -unpack-nothing }
unpack-reading-the-record = reading the record
unpack-does-not-verify = the record in that file does not verify. { -unpack-nothing }
unpack-another-key = the record in that file was written by another key. { -unpack-nothing }

unpack-not-a-place = { $target } is not a directory a node can be put in
unpack-making-room = making room at { $target }
unpack-putting = putting the node in { $target }

unpack-name = { $name }
    .keyword = name

unpack-record-none = none yet
    .keyword = record

unpack-record = { $epochs ->
        [one] { $epochs } epoch, verified
       *[other] { $epochs } epochs, verified
    }
    .keyword = record

unpack-holding-the-file = the file
    .keyword = holding

unpack-onion-key = the key to its onion address, so the address came with it
    .keyword = unseen

unpack-unpacked = into { $target },
    from a file packed at { $packed }.
    This is the node now, and so is that file: delete { $file }
    .keyword = unpacked

unpack-next = { $serve }
    .keyword = next

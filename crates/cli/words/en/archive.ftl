### The one file a node becomes while it is carried: reading and writing it.

# A term (a name that starts with `-`) goes on with a line that has to stay one
# line and is wider than a line of a catalog.
-archive-nothing = Nothing was unpacked.

archive-not-a-packed-node = that file is not a packed node: its manifest does not begin `{ $heading }`
archive-no-format = the manifest does not say which format it is
archive-newer = { -archive-packed-by-newer } { $format }. This one reads up to format { $ours }; { -archive-use-the-newer }
-archive-packed-by-newer = that file was packed by a newer client, in format
-archive-use-the-newer = unpack it with the newer one.
archive-no-name = the manifest does not say which name it holds

archive-listing = listing { $dir }
archive-reading = reading { $file }
archive-packing = packing { $file }
archive-finishing = finishing the archive
archive-syncing = writing the archive to disk

archive-opening = opening { $file }
archive-reading-the-archive = reading the archive
archive-not-packed = { $file } is not a packed node: it cannot be read as one
archive-reading-the-manifest = reading the manifest
archive-reading-the-seed = reading the seed
archive-no-manifest = that file has no manifest, so it is not a packed node
archive-no-seed = that file holds no seed, so there is no name in it

archive-making = making { $dir }
archive-writing = writing { $file }
archive-reading-a-name = reading a name in the archive
archive-name-not-text = the archive holds a name that is not text, which no node file has
archive-not-a-node-file = that file holds { $name }, which is not part of a node. { -archive-nothing }

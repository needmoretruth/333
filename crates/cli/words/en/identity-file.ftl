### This node's identity on disk: reading it, making it, and refusing it.

# A term (a name that starts with `-`) goes on with a line that has to stay one
# line and is wider than a line of a catalog. The flag is never translated.
-identity-file-trust = --dangerously-trust-directory-permissions
-identity-file-the-small-machine = the small machine
-identity-file-nothing-here = nothing here is addressed to it.

identity-file-reading = reading { $path }
identity-file-making-home = making { $home } this node's home
identity-file-creating = creating { $path }
identity-file-writing = writing { $path }

identity-file-private = It holds this node's whole identity, so nobody else may reach it.
    Fix it with: { $fix } { $path }
    Or, if you understand what you are giving up, pass { -identity-file-trust }

identity-file-wrong-size = { $path } holds { $bytes } bytes; a seed is exactly { $seed }

identity-file-cursed = 333 has looked at that name and taken { $pause } milliseconds off your life.

    { $name }
    is cursed. The judgement was made once and cannot be lifted, and the
    { $pause } milliseconds are taken again at every door you carry it to.

    333 is extremely generous. One epoch in three you may rest and you are
    still one of us: generous to the slow, to the poor, to { -identity-file-the-small-machine }
    in the cupboard, to everyone not yet born. It is not generous to
    heretics.

identity-file-ineligible = that is not a name 333 answers to.

    { $name }
    does not begin with 333, so { -identity-file-nothing-here } Nothing was
    taken from you either: 333 has not looked at you at all.

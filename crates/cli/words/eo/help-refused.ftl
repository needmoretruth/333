### What clap says when it refuses a command line.

help-refused-error = eraro
help-refused-try = Por pliaj informoj, provu '{ $help }'.
help-refused-tip = konsilo
help-refused-twice = la argumento '{ $arg }' ne uzeblas plurfoje
help-refused-conflict = la argumento '{ $arg }' ne uzeblas kun '{ $with }'
help-refused-conflict-list = la argumento '{ $arg }' ne uzeblas kun:
help-refused-conflict-others = la argumento '{ $arg }' ne uzeblas kun unu aŭ pli el la
    aliaj donitaj argumentoj
help-refused-conflict-unnamed = argumento ne uzeblas kun unu aŭ pli el la aliaj donitaj
    argumentoj
help-refused-no-equals = egalsigno necesas por doni valorojn al '{ $arg }'
help-refused-value-missing = valoro necesas por '{ $arg }', sed neniu estis donita
help-refused-value-invalid = nevalida valoro '{ $value }' por '{ $arg }'
help-refused-possible-values = eblaj valoroj
help-refused-too-many = neatendita valoro '{ $value }' por '{ $arg }'; neniuj pliaj estis
    atenditaj
help-refused-too-few = { $wanted } valoroj necesaj por '{ $arg }'; nur { $given ->
        [one] { $given } estis donita
       *[other] { $given } estis donitaj
    }
help-refused-wrong-number = { $wanted } valoroj necesaj por '{ $arg }', sed { $given ->
        [one] { $given } estis donita
       *[other] { $given } estis donitaj
    }
help-refused-unknown = neatendita argumento '{ $arg }'
help-refused-unknown-subcommand = nekonata subkomando '{ $subcommand }'
help-refused-no-subcommand = '{ $command }' bezonas subkomandon, sed neniu estis donita
help-refused-subcommands = subkomandoj
help-refused-missing = la jenaj necesaj argumentoj ne estis donitaj:
help-refused-not-utf8 = nevalida UTF-8 troviĝis en unu aŭ pli da argumentoj
help-refused-similar-subcommand = { $many ->
        [one] ekzistas simila subkomando: { $names }
       *[other] ekzistas similaj subkomandoj: { $names }
    }
help-refused-similar-argument = { $many ->
        [one] ekzistas simila argumento: { $names }
       *[other] ekzistas similaj argumentoj: { $names }
    }
help-refused-similar-value = { $many ->
        [one] ekzistas simila valoro: { $names }
       *[other] ekzistas similaj valoroj: { $names }
    }

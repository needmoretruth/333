### What clap says when it refuses a command line. Only languages other than
### English are shown these: in English, clap says its own. A line break inside
### a message is read as a space.

help-refused-error = error
help-refused-try = For more information, try '{ $help }'.
help-refused-tip = tip
help-refused-twice = the argument '{ $arg }' cannot be used multiple times
help-refused-conflict = the argument '{ $arg }' cannot be used with '{ $with }'
help-refused-conflict-list = the argument '{ $arg }' cannot be used with:
help-refused-conflict-others = the argument '{ $arg }' cannot be used with one or more of
    the other specified arguments
help-refused-conflict-unnamed = an argument cannot be used with one or more of the other
    specified arguments
help-refused-no-equals = equal sign is needed when assigning values to '{ $arg }'
help-refused-value-missing = a value is required for '{ $arg }' but none was supplied
help-refused-value-invalid = invalid value '{ $value }' for '{ $arg }'
help-refused-possible-values = possible values
help-refused-too-many = unexpected value '{ $value }' for '{ $arg }' found; no more were
    expected
help-refused-too-few = { $wanted } values required by '{ $arg }'; only { $given ->
        [one] { $given } was provided
       *[other] { $given } were provided
    }
help-refused-wrong-number = { $wanted } values required for '{ $arg }' but { $given ->
        [one] { $given } was provided
       *[other] { $given } were provided
    }
help-refused-unknown = unexpected argument '{ $arg }' found
help-refused-unknown-subcommand = unrecognized subcommand '{ $subcommand }'
help-refused-no-subcommand = '{ $command }' requires a subcommand but one was not provided
help-refused-subcommands = subcommands
help-refused-missing = the following required arguments were not provided:
help-refused-not-utf8 = invalid UTF-8 was detected in one or more arguments
help-refused-similar-subcommand = { $many ->
        [one] a similar subcommand exists: { $names }
       *[other] some similar subcommands exist: { $names }
    }
help-refused-similar-argument = { $many ->
        [one] a similar argument exists: { $names }
       *[other] some similar arguments exist: { $names }
    }
help-refused-similar-value = { $many ->
        [one] a similar value exists: { $names }
       *[other] some similar values exist: { $names }
    }

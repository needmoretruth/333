### What clap says when it refuses a command line.

help-refused-error = Fehler
help-refused-try = Mehr dazu mit '{ $help }'.
help-refused-tip = Tipp
help-refused-twice = das Argument '{ $arg }' darf nicht mehrfach angegeben werden
help-refused-conflict = das Argument '{ $arg }' kann nicht mit '{ $with }' verwendet werden
help-refused-conflict-list = das Argument '{ $arg }' kann nicht verwendet werden mit:
help-refused-conflict-others = das Argument '{ $arg }' kann nicht mit einem oder mehreren der
    anderen angegebenen Argumente verwendet werden
help-refused-conflict-unnamed = ein Argument kann nicht mit einem oder mehreren der anderen
    angegebenen Argumente verwendet werden
help-refused-no-equals = für einen Wert von '{ $arg }' ist ein Gleichheitszeichen nötig
help-refused-value-missing = '{ $arg }' braucht einen Wert, aber es wurde keiner angegeben
help-refused-value-invalid = ungültiger Wert '{ $value }' für '{ $arg }'
help-refused-possible-values = mögliche Werte
help-refused-too-many = unerwarteter Wert '{ $value }' für '{ $arg }'; es wurden keine weiteren
    erwartet
help-refused-too-few = '{ $arg }' braucht { $wanted } Werte; { $given ->
        [one] nur { $given } wurde angegeben
       *[other] nur { $given } wurden angegeben
    }
help-refused-wrong-number = '{ $arg }' braucht { $wanted } Werte, aber { $given ->
        [one] { $given } wurde angegeben
       *[other] { $given } wurden angegeben
    }
help-refused-unknown = unerwartetes Argument '{ $arg }'
help-refused-unknown-subcommand = unbekannter Unterbefehl '{ $subcommand }'
help-refused-no-subcommand = '{ $command }' braucht einen Unterbefehl, aber es wurde keiner angegeben
help-refused-subcommands = Unterbefehle
help-refused-missing = diese nötigen Argumente wurden nicht angegeben:
help-refused-not-utf8 = in einem oder mehreren Argumenten steht ungültiges UTF-8
help-refused-similar-subcommand = { $many ->
        [one] es gibt einen ähnlichen Unterbefehl: { $names }
       *[other] es gibt ähnliche Unterbefehle: { $names }
    }
help-refused-similar-argument = { $many ->
        [one] es gibt ein ähnliches Argument: { $names }
       *[other] es gibt ähnliche Argumente: { $names }
    }
help-refused-similar-value = { $many ->
        [one] es gibt einen ähnlichen Wert: { $names }
       *[other] es gibt ähnliche Werte: { $names }
    }

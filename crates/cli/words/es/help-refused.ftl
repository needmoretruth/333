### What clap says when it refuses a command line.

help-refused-error = error
help-refused-try = Para más información, prueba '{ $help }'.
help-refused-tip = consejo
help-refused-twice = el argumento '{ $arg }' no se puede usar varias veces
help-refused-conflict = el argumento '{ $arg }' no se puede usar con '{ $with }'
help-refused-conflict-list = el argumento '{ $arg }' no se puede usar con:
help-refused-conflict-others = el argumento '{ $arg }' no se puede usar con uno o más de los
    otros argumentos indicados
help-refused-conflict-unnamed = un argumento no se puede usar con uno o más de los otros
    argumentos indicados
help-refused-no-equals = hace falta un signo igual para dar valor a '{ $arg }'
help-refused-value-missing = '{ $arg }' necesita un valor y no se dio ninguno
help-refused-value-invalid = valor '{ $value }' no válido para '{ $arg }'
help-refused-possible-values = valores posibles
help-refused-too-many = valor inesperado '{ $value }' para '{ $arg }'; no se esperaban más
help-refused-too-few = '{ $arg }' necesita { $wanted } valores; { $given ->
        [one] solo se dio { $given }
       *[other] solo se dieron { $given }
    }
help-refused-wrong-number = '{ $arg }' necesita { $wanted } valores, pero { $given ->
        [one] se dio { $given }
       *[other] se dieron { $given }
    }
help-refused-unknown = argumento inesperado '{ $arg }'
help-refused-unknown-subcommand = subcomando no reconocido '{ $subcommand }'
help-refused-no-subcommand = '{ $command }' necesita un subcomando y no se dio ninguno
help-refused-subcommands = subcomandos
help-refused-missing = faltan estos argumentos obligatorios:
help-refused-not-utf8 = hay UTF-8 no válido en uno o más argumentos
help-refused-similar-subcommand = { $many ->
        [one] existe un subcomando parecido: { $names }
       *[other] existen subcomandos parecidos: { $names }
    }
help-refused-similar-argument = { $many ->
        [one] existe un argumento parecido: { $names }
       *[other] existen argumentos parecidos: { $names }
    }
help-refused-similar-value = { $many ->
        [one] existe un valor parecido: { $names }
       *[other] existen valores parecidos: { $names }
    }

### What clap says when it refuses a command line.

help-refused-error = erreur
help-refused-try = Pour plus d’informations, essayez '{ $help }'.
help-refused-tip = astuce
help-refused-twice = l’argument '{ $arg }' ne peut pas être utilisé plusieurs fois
help-refused-conflict = l’argument '{ $arg }' ne peut pas être utilisé avec '{ $with }'
help-refused-conflict-list = l’argument '{ $arg }' ne peut pas être utilisé avec :
help-refused-conflict-others = l’argument '{ $arg }' ne peut pas être utilisé avec un ou plusieurs
    des autres arguments donnés
help-refused-conflict-unnamed = un argument ne peut pas être utilisé avec un ou plusieurs des
    autres arguments donnés
help-refused-no-equals = un signe égal est nécessaire pour donner une valeur à '{ $arg }'
help-refused-value-missing = '{ $arg }' demande une valeur, et aucune n’a été donnée
help-refused-value-invalid = valeur '{ $value }' invalide pour '{ $arg }'
help-refused-possible-values = valeurs possibles
help-refused-too-many = valeur inattendue '{ $value }' pour '{ $arg }' ; aucune autre n’était
    attendue
help-refused-too-few = '{ $arg }' demande { $wanted } valeurs ; { $given ->
        [one] seulement { $given } a été donnée
       *[other] seulement { $given } ont été données
    }
help-refused-wrong-number = '{ $arg }' demande { $wanted } valeurs, mais { $given ->
        [one] { $given } a été donnée
       *[other] { $given } ont été données
    }
help-refused-unknown = argument inattendu '{ $arg }'
help-refused-unknown-subcommand = sous-commande inconnue '{ $subcommand }'
help-refused-no-subcommand = '{ $command }' demande une sous-commande, et aucune n’a été donnée
help-refused-subcommands = sous-commandes
help-refused-missing = ces arguments obligatoires n’ont pas été donnés :
help-refused-not-utf8 = de l’UTF-8 invalide a été trouvé dans un ou plusieurs arguments
help-refused-similar-subcommand = { $many ->
        [one] une sous-commande proche existe : { $names }
       *[other] des sous-commandes proches existent : { $names }
    }
help-refused-similar-argument = { $many ->
        [one] un argument proche existe : { $names }
       *[other] des arguments proches existent : { $names }
    }
help-refused-similar-value = { $many ->
        [one] une valeur proche existe : { $names }
       *[other] des valeurs proches existent : { $names }
    }

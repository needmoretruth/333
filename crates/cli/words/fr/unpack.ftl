### `333 unpack`: putting a packed node into this machine's node directory.

-unpack-nothing = Rien n’a été dépaqueté.
-unpack-so-nothing = donc rien n’a été dépaqueté.

unpack-kept = un nœud vit déjà dans ce répertoire. Pour dépaqueter à côté,
    donnez-lui son propre répertoire avec --data-dir.

unpack-seed-in = la { $seed } dans { $file }

unpack-not-one-node = ce fichier dit contenir { $claimed }, et la clé qu’il contient est { $name }. { -unpack-not-one }
-unpack-not-one = Ce n’est pas un seul nœud, et rien n’a été dépaqueté.

unpack-taken = un autre 333 a pris le répertoire où ceci se dépaquetait. { -unpack-nothing }

unpack-elsewhere = 333 --data-dir <un autre répertoire> unpack { $file }

unpack-could-not-open = un nœud vit déjà dans { $target }, et il { -unpack-could-not-be-opened } Pour dépaqueter à côté : { $elsewhere }
-unpack-could-not-be-opened = n’a pas pu être ouvert pour dire ce qu’il contient. { -unpack-nothing }

unpack-no-record = pas encore de registre
unpack-epochs-of-record = { $epochs ->
        [one] { $epochs } époque de registre
       *[other] { $epochs } époques de registre
    }
unpack-holding = détient le fichier
unpack-not-holding = ne détient pas le fichier

unpack-occupied = un nœud vit déjà dans { $target } :
    { $name }, { $epochs }, { $holding }.
    Dépaqueter par-dessus perdrait tout cela pour de bon, { -unpack-so-nothing }
    Pour dépaqueter à côté, donnez-lui son propre répertoire :
    { $elsewhere }

unpack-holds-files = { $target } contient des fichiers et aucun nœud. { -unpack-its-own } Ailleurs : { $elsewhere }
-unpack-its-own = Un nœud se dépaquette dans son propre répertoire, { -unpack-so-nothing }

unpack-opening-the-record = ouverture du registre
unpack-torn = le registre de ce fichier est tronqué : ce n’est pas un nœud entier. { -unpack-nothing }
unpack-reading-the-record = lecture du registre
unpack-does-not-verify = le registre de ce fichier ne se vérifie pas. { -unpack-nothing }
unpack-another-key = le registre de ce fichier a été écrit par une autre clé. { -unpack-nothing }

unpack-not-a-place = { $target } n’est pas un répertoire où l’on peut mettre un nœud
unpack-making-room = on fait de la place à { $target }
unpack-putting = le nœud est mis dans { $target }

unpack-name = { $name }
    .keyword = nom

unpack-record-none = aucun encore
    .keyword = registre

unpack-record = { $epochs ->
        [one] { $epochs } époque, vérifiée
       *[other] { $epochs } époques, vérifiées
    }
    .keyword = registre

unpack-holding-the-file = le fichier
    .keyword = détient

unpack-onion-key = la clé de son adresse onion : l’adresse est venue avec lui
    .keyword = caché

unpack-unpacked = dans { $target },
    depuis un fichier empaqueté le { $packed }.
    C’est désormais le nœud, et ce fichier aussi : supprimez { $file }
    .keyword = fait

unpack-next = { $serve }
    .keyword = ensuite

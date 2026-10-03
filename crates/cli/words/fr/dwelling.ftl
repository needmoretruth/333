### Where this node lives, and whether it still lives here.

-dwelling-would-be-one = ferait un même
-dwelling-anywhere-this = nulle part, ceci

dwelling-made-at = créé à
dwelling-unpacked-at = dépaqueté à
dwelling-moved-to = dit déplacé vers
dwelling-found-at = ouvert pour la première fois par ce client à

dwelling-writing = écriture de { $file }
dwelling-putting-in-place = mise en place de { $file }
dwelling-resolving = résolution de { $dir }

dwelling-elsewhere = ce nœud a été { $how } { $was },
    et il est maintenant à { $now }.
    Si le répertoire a été déplacé ou renommé, ce n’est que cela. S’il a
    été copié et que l’original tourne encore, c’est un même nom en deux
    lieux, et chacun contredira le registre de l’autre. Quand il n’en
    reste qu’un, dites-le ici : { $settle }
    .keyword = demeure

dwelling-unread-moment = un moment illisible
dwelling-unread-file = un fichier dont le nom est illisible

dwelling-packed = ce nœud a été empaqueté pour déménager le { $at }, dans { $into }.
    Il vit là où ce fichier a été dépaqueté. Le lancer ici aussi { -dwelling-would-be-one }
    nom en deux lieux, donc rien dans { $home } n’agira comme lui.

    Si le déménagement est abandonné et que ce fichier n’a été dépaqueté { -dwelling-anywhere-this }
    le remet en place : { $undo }

dwelling-unmarking = retrait de la marque indiquant que ce nœud a été empaqueté

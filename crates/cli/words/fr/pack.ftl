### `333 pack`: writing this node into one file, to be carried to another machine.

pack-name-the-file = nommez le fichier où empaqueter ce nœud : 333 pack <FILE>

pack-no-node = il n’y a aucun nœud à empaqueter dans { $root }. Rien n’a été écrit.

pack-already-exists = { $file } existe déjà. { -pack-never-over }
-pack-never-over = Un nouveau fichier est écrit, jamais sur un ancien ; changez de nom.

pack-not-marked = { $file } a été écrit, mais ce répertoire n’a pu être marqué empaqueté. { -pack-until-it-is }
-pack-until-it-is = D’ici là, ce nœud vit dans les deux : { -pack-delete-that-file }
-pack-delete-that-file = supprimez ce fichier avant que quoi que ce soit tourne ici.

pack-creating = création de { $file }

pack-name = { $name }
    .keyword = nom

pack-record-none = aucun encore
    .keyword = registre

pack-record = { $epochs ->
        [one] { $epochs } époque, qui part avec lui
       *[other] { $epochs } époques, qui partent avec lui
    }
    .keyword = registre

pack-witnessed = { $statements ->
        [one] { $statements } déclaration qu’une autre clé a signée sur lui, qui part avec lui
       *[other] { $statements } déclarations que d’autres clés ont signées sur lui,
            qui partent avec lui
    }
    .keyword = témoin

pack-holding = le fichier, qui part avec lui
    .keyword = détient

pack-onion-key = la clé de son adresse onion : l’adresse part avec lui
    .keyword = caché

pack-carrying = ce nœud, dans { $file }.
    Ce fichier EST ce nœud : qui le détient peut répondre sous ce nom.
    Emportez-le, dépaquetez-le, puis supprimez-le ; ce n’est pas une
    sauvegarde à garder. Il n’est pas chiffré, car un mot de passe serait
    une chose de plus à perdre, et le perdre ferait perdre le nom aussi
    sûrement que perdre le fichier. Vous seul pouvez le lire, comme ce
    répertoire.
    .keyword = emporte

pack-packed = { $bytes } octets.
    rien dans { $root } n’agira plus comme ce nœud.
    .keyword = fait

pack-next = sur l’autre machine : 333 unpack { $carried }
    si le déménagement est abandonné : { $undo }
    .keyword = ensuite

pack-not-packed = ce nœud n’a pas été empaqueté, il n’y a donc rien à annuler dans { $root }
    .keyword = ici

pack-restored = ce nœud vit de nouveau dans { $root }.
    Le fichier où il a été empaqueté est encore ce nom. S’il a été
    dépaqueté quelque part, l’un des deux doit partir avant que l’un ou
    l’autre tourne ; sinon, supprimez { $file }
    .keyword = rétabli

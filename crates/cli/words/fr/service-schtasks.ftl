### `333 service` on Windows: Task Scheduler.

service-schtasks-cannot-pass = `{ $word }` contient un " ou un %, que cmd.exe ne peut pas transmettre tel quel
service-schtasks-no-user = Windows n’a pas dit qui est cet utilisateur
    (%USERDOMAIN% et %USERNAME%)

service-schtasks-logon = Windows fait tourner le nœud tant que vous êtes connecté, dès la
    connexion. Un service lancé au démarrage demanderait un compte à lui,
    et un nœud vit dans votre propre répertoire.
    Ce qu’il dit est dans { $log }.
    .keyword = session

service-schtasks-ready = arrêté, et relancé dans les 333 secondes
service-schtasks-disabled = désactivé : il ne redémarre pas de lui-même

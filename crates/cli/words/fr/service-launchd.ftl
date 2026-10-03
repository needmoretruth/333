### `333 service` on macOS: launchd.

service-launchd-no-home = ce système n’indique aucun répertoire personnel pour cet utilisateur
service-launchd-asking-who = on demande à `id -u` quel utilisateur c’est

service-launchd-login = launchd fait tourner le nœud de votre connexion à votre
    déconnexion, et après un redémarrage il reprend quand vous vous
    connectez. Ce qu’il dit est dans { $log }.
    .keyword = session

service-launchd-ended-with = { $state }, et il s’est terminé la dernière fois avec { $code }
service-launchd-loaded = chargé, et launchd n’a pas dit ce qu’il fait

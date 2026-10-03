### `333 service` on Linux: systemd.

service-systemd-no-configuration = ce système n’indique aucun répertoire de configuration
    pour cet utilisateur

service-systemd-no-session = systemd ne garde pas de session pour cet utilisateur ici ({ $why }).
    Il en ouvre une quand cet utilisateur se connecte, en console ou par
    ssh, et pas par su ou sudo. Connectez-vous comme cet utilisateur et
    relancez ceci.

service-systemd-wrote-over = { $path }, à la place de celui qui y était
    .keyword = écrit

service-systemd-linger-already = déjà actif pour { $user }. Il garde le nœud en marche après votre
    déconnexion, et le lance au démarrage sans personne de connecté.
    .keyword = linger

service-systemd-linger-on = activé pour { $user }. Il garde le nœud en marche après votre
    déconnexion, et le lance au démarrage sans personne de connecté.
    .keyword = linger

service-systemd-linger-not-on = inactif : { $why }. Sans lui, le nœud s’arrête à votre
    déconnexion et attend votre reconnexion après un redémarrage.
    `sudo loginctl enable-linger { $user }` l’active.
    .keyword = linger

service-systemd-linger-off = de nouveau désactivé, comme avant l’installation.
    .keyword = linger

service-systemd-linger-left = laissé tel quel. L’installation ne l’avait pas activé.
    .keyword = linger

service-systemd-not-answering = inconnu : systemd ne répond pas pour cet utilisateur
service-systemd-restarting = arrêté, et relancé 333 secondes après son arrêt
service-systemd-stopping = en cours d’arrêt
service-systemd-failed = en échec ({ $result })
service-systemd-stopped = arrêté
service-systemd-at-every-boot = { $state }, et lancé à chaque démarrage
service-systemd-not-again = { $state }, et { $file_state } : il ne redémarre pas de lui-même

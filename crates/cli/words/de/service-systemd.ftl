### `333 service` on Linux: systemd.

service-systemd-no-configuration = dieses System nennt kein Konfigurationsverzeichnis für diesen Nutzer

service-systemd-no-session = systemd führt hier keine Sitzung für diesen Nutzer ({ $why }).
    Es startet eine, wenn sich dieser Nutzer anmeldet, an der Konsole
    oder über ssh, nicht über su oder sudo. Melde dich als dieser
    Nutzer an und führe das erneut aus.

service-systemd-wrote-over = { $path }, statt der dortigen
    .keyword = schrieb

service-systemd-linger-already = schon an für { $user }. Es lässt den Knoten nach der Abmeldung
    weiterlaufen und startet ihn beim Hochfahren ohne Anmeldung.
    .keyword = linger

service-systemd-linger-on = an für { $user }. Es lässt den Knoten nach der Abmeldung
    weiterlaufen und startet ihn beim Hochfahren ohne Anmeldung.
    .keyword = linger

service-systemd-linger-not-on = nicht an: { $why }. Ohne es stoppt der Knoten bei der Abmeldung
    und wartet nach einem Neustart, bis du dich wieder anmeldest.
    `sudo loginctl enable-linger { $user }` schaltet es an.
    .keyword = linger

service-systemd-linger-off = wieder aus, wie vor der Installation.
    .keyword = linger

service-systemd-linger-left = gelassen, wie es war. Die Installation hat es nicht angeschaltet.
    .keyword = linger

service-systemd-not-answering = unbekannt: systemd antwortet nicht für diesen Nutzer
service-systemd-restarting = gestoppt, und 333 Sekunden nach dem Stopp wieder gestartet
service-systemd-stopping = stoppt
service-systemd-failed = fehlgeschlagen ({ $result })
service-systemd-stopped = gestoppt
service-systemd-at-every-boot = { $state }, und bei jedem Hochfahren gestartet
service-systemd-not-again = { $state }, und { $file_state }: er startet nicht von selbst wieder

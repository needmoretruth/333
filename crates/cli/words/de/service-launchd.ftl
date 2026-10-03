### `333 service` on macOS: launchd.

service-launchd-no-home = dieses System nennt kein Heimverzeichnis für diesen Nutzer
service-launchd-asking-who = frage `id -u`, welcher Nutzer das ist

service-launchd-login = launchd lässt den Knoten von deiner Anmeldung bis zur Abmeldung
    laufen, und nach einem Neustart beginnt er wieder, wenn du dich
    anmeldest. Was er sagt, steht in { $log }.
    .keyword = Anmeld.

service-launchd-ended-with = { $state }, und er endete zuletzt mit { $code }
service-launchd-loaded = geladen, und launchd sagte nicht, was er tut

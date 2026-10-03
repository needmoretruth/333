### `333 service` on macOS: launchd.

service-launchd-no-home = este sistema no indica ningún directorio personal para este usuario
service-launchd-asking-who = preguntando a `id -u` qué usuario es este

service-launchd-login = launchd ejecuta el nodo desde que inicias sesión hasta que la
    cierras, y tras un reinicio vuelve a empezar cuando inicias sesión.
    Lo que dice está en { $log }.
    .keyword = sesión

service-launchd-ended-with = { $state }, y terminó por última vez con { $code }
service-launchd-loaded = cargado, y launchd no dijo qué está haciendo

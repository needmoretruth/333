### `333 service` on macOS: launchd.

service-launchd-no-home = this system names no home directory for this user
service-launchd-asking-who = asking `id -u` which user this is

service-launchd-login = launchd keeps the vigil from the moment you log in until you log out,
    and after a restart it begins again when you log in. What it says is in
    { $log }.
    .keyword = login

service-launchd-ended-with = { $state }, and it last ended with { $code }
service-launchd-loaded = loaded, and launchd did not say what it is doing

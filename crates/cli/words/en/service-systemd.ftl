### `333 service` on Linux: systemd.
##
## A line here ends in `{` and the next begins with `""}` where the printed line is
## wider than a line of this file may be; the placeable spanning the two prints no
## line break.

service-systemd-no-configuration = this system names no configuration directory for this user

service-systemd-no-session = systemd is not keeping a session for this user here {
    ""}({ $why }). It starts one when this user logs in, at the console or {
    ""}over ssh, and not through su or sudo. Log in as this user and run {
    ""}this again.

service-systemd-wrote-over = { $path }, in place of the one there
    .keyword = wrote

service-systemd-linger-already = already on for { $user }. It is what keeps the vigil running after you log
    out, and starts it at boot with nobody logged in.
    .keyword = linger

service-systemd-linger-on = on for { $user }. It is what keeps the vigil running after you log out,
    and starts it at boot with nobody logged in.
    .keyword = linger

service-systemd-linger-not-on = not on: { $why }. Without it the vigil stops when you log out and waits
    for you to log in again after a reboot. `sudo loginctl enable-linger
    { $user }` turns it on.
    .keyword = linger

service-systemd-linger-off = off again, as it was before install.
    .keyword = linger

service-systemd-linger-left = left as it was. Install did not turn it on.
    .keyword = linger

service-systemd-not-answering = unknown: systemd is not answering for this user
service-systemd-restarting = stopped, and starting again 333 seconds after it stopped
service-systemd-stopping = stopping
service-systemd-failed = failed ({ $result })
service-systemd-stopped = stopped
service-systemd-at-every-boot = { $state }, and started at every boot
service-systemd-not-again = { $state }, and { $file_state }: it does not start again by itself

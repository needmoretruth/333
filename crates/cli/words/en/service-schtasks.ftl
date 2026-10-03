### `333 service` on Windows: Task Scheduler.
##
## A line here ends in `{` and the next begins with `""}` where the printed line is
## wider than a line of this file may be; the placeable spanning the two prints no
## line break.

service-schtasks-cannot-pass = `{ $word }` has a " or a % in it, which cmd.exe cannot {
    ""}pass on as it is
service-schtasks-no-user = Windows did not say who this user is {
    ""}(%USERDOMAIN% and %USERNAME%)

service-schtasks-logon = Windows runs the node while you are logged in, from the moment you {
    ""}log
    in. A service run from boot would need an account of its own, and a
    node lives in your own directory.
    What it says is in { $log }.
    .keyword = logon

service-schtasks-ready = stopped, and started again within 333 seconds
service-schtasks-disabled = disabled: it does not start again by itself

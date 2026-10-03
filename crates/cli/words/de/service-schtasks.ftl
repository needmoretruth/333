### `333 service` on Windows: Task Scheduler.

service-schtasks-cannot-pass = `{ $word }` enthält ein " oder ein %, das cmd.exe nicht weitergeben kann
service-schtasks-no-user = Windows sagte nicht, wer dieser Nutzer ist
    (%USERDOMAIN% und %USERNAME%)

service-schtasks-logon = Windows lässt den Knoten laufen, solange du angemeldet bist, ab
    der Anmeldung. Ein Dienst ab dem Systemstart bräuchte ein eigenes
    Konto, und ein Knoten lebt in deinem eigenen Verzeichnis.
    Was er sagt, steht in { $log }.
    .keyword = Anmeld.

service-schtasks-ready = gestoppt, und innerhalb von 333 Sekunden wieder gestartet
service-schtasks-disabled = deaktiviert: er startet nicht von selbst wieder

### `333 service` on Windows: Task Scheduler.

service-schtasks-cannot-pass = `{ $word }` lleva una " o un %, que cmd.exe no puede pasar tal cual
service-schtasks-no-user = Windows no dijo quién es este usuario
    (%USERDOMAIN% y %USERNAME%)

service-schtasks-logon = Windows ejecuta el nodo mientras tienes la sesión iniciada, desde
    que la inicias. Un servicio desde el arranque necesitaría una cuenta
    propia, y un nodo vive en tu propio directorio.
    Lo que dice está en { $log }.
    .keyword = sesión

service-schtasks-ready = detenido, y vuelve a iniciarse en 333 segundos
service-schtasks-disabled = desactivado: no vuelve a iniciarse solo

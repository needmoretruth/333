### `333 start` and `333 restart`: running this node in the background.

start-no-node = in { $home } gibt es noch keinen Knoten

start-no-node-next = `333 join <invitation>` tritt mit der Einladung von jemandem bei,
    der 333 betreibt. `333 begin` beginnt allein.

start-in-a-terminal = schon, in einem Terminal. Stoppe ihn dort oder mit `333 stop`,
    dann lässt `333 start` ihn im Hintergrund laufen.
    .keyword = läuft

start-already = schon, im Hintergrund.
    .keyword = läuft

start-started = im Hintergrund, jetzt und nach jedem Neustart. `333 status` zeigt,
    wie es ihm geht; `333 stop` stoppt ihn.
    .keyword = läuft

start-elsewhere = der Hintergrunddienst dieses Rechners lässt den Knoten in { $other }
    laufen. `333 service uninstall` entfernt ihn, dann wieder `333 start`.

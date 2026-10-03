### `333 start` and `333 restart`: running this node in the background.

start-no-node = ankoraŭ ne estas nodo en { $home }

start-no-node-next = `333 join <invitation>` aliĝas per invito de iu, kiu funkciigas
    333. `333 begin` komencas sola.

start-in-a-terminal = jam, en terminalo. Haltigu ĝin tie, aŭ per `333 stop`, kaj tiam
    `333 start` funkciigas ĝin fone.
    .keyword = funkcias

start-already = jam, fone.
    .keyword = funkcias

start-started = fone, nun kaj post ĉiu restartigo. `333 status` montras, kiel ĝi
    fartas; `333 stop` haltigas ĝin.
    .keyword = startis

start-elsewhere = la fona servo sur ĉi tiu maŝino funkciigas la nodon en { $other }.
    `333 service uninstall` forigas ĝin, kaj poste denove `333 start`.

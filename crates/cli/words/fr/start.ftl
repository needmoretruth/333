### `333 start` and `333 restart`: running this node in the background.

start-no-node = il n’y a pas encore de nœud dans { $home }

start-no-node-next = `333 join <invitation>` rejoint avec l’invitation de quelqu’un qui
    fait tourner 333. `333 begin` commence seul.

start-in-a-terminal = déjà, dans un terminal. Arrêtez-le là, ou avec `333 stop`, puis
    `333 start` le lance en arrière-plan.
    .keyword = marche

start-already = déjà, en arrière-plan.
    .keyword = marche

start-started = en arrière-plan, maintenant et après chaque redémarrage.
    `333 status` montre comment il va ; `333 stop` l’arrête.
    .keyword = lancé

start-elsewhere = le service d’arrière-plan de cette machine fait tourner le nœud de
    { $other }. `333 service uninstall` le retire, puis `333 start` à
    nouveau.

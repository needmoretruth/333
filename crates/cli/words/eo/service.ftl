### `333 service`: running the node through logouts and reboots, when asked to.

service-mind = { $node } estas ie, kion ĉi tiu sistemo malplenigas, kaj la nomo de ĉi tiu
    nodo estas tenata nenie alie. La servo funkciigas ĝin tie, ĝis ĝi estos
    malplenigita.
    .keyword = atentu

service-runs = { $command }
    .keyword = nodo

service-undo-partial = `333 service uninstall` forigas ĉion, kio el ĉi tio estis farita.
    .keyword = malfari

service-no-receipt-directory = ĉi tiu sistemo nomas neniun agordan dosierujon por la kvito

service-wrote-receipt = { $path }, per kiu `333 service uninstall` scias, kion malfari.
    .keyword = skribis

service-undo = `333 service uninstall` haltigas la nodon kaj malfaras ĉion supre.
    La propra dosierujo de la nodo estas tuŝita de neniu el ili.
    .keyword = malfari

service-uninstalled = ne plu funkciigata de servo. { $node } restas, kiel la nodo lasis ĝin:
    `333 run` funkciigas ĝin mane, kaj `333 start` restarigas la servon.
    .keyword = nodo

service-none-installed = nenio estis instalita per `333 service install` por ĉi tiu uzanto.
    .keyword = servo

service-state = { $state }
    .keyword = servo

service-node = { $node }
    .keyword = nodo

service-last-awake = diris tion laste ĉe { $at }, antaŭ { $ago }
    .keyword = maldorma

service-never-awake = neniam diris tion, en ĉi tiu dosierujo
    .keyword = maldorma

service-said-nothing = nenio tenita
    .keyword = diris

service-said-last = { $lines ->
        [one] la lasta linio:
       *[other] la lastaj { $lines } linioj:
    }
    .keyword = diris

service-no-manager = ĉi tiu sistemo havas neniun servoadministrilon, kiun
    `333 service` scias demandi. `333 run --plain` funkciigas la nodon
    sub kio ajn tenas programojn funkciantaj ĉi tie.

service-not-installed-here = ne instalita: ne estas servoadministrilo ĉi tie, kiun ĉi tio konas

# Said by every service manager's own file.

service-creating = kreante { $path }
service-writing = skribante { $path }
service-removing = forigante { $path }

service-wrote = { $path }
    .keyword = skribis

service-removed = { $path }
    .keyword = forigis

service-left = { $path }. `333 service install` ne skribis ĝin.
    .keyword = lasita

service-failed = { $why }
    .keyword = fiaskis

service-not-installed = ne instalita
service-running = funkcianta
service-starting = startanta

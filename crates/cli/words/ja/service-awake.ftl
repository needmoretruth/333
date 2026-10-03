### `333 service`: the line a running node writes to say it is still awake.

service-awake-failed = ノードが起きていることを { $root } に書いています: { $why }。
    これが直るまで、このマシンの何もノードが動いているとわかりません。
    .keyword = 失敗

service-awake-never-kept = 動いていません。サービスは入っていますが、ノードは一度も起きて
    いると告げていません。理由は `333 service status` でわかります。
    .keyword = ノード

service-awake-not-kept-since = { $at } ({ $ago } 前) から動いていません。理由は
    `333 service status` でわかります。
    .keyword = ノード

service-awake-under-a-minute = 1 分未満

service-awake-minutes = { $minutes ->
       *[other] { $minutes } 分
    }

service-awake-epochs = { $epochs ->
       *[other] { $epochs } エポック
    }

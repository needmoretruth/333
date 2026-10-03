### `333 service`: running the node through logouts and reboots, when asked to.

service-mind = { $node } はこのシステムが空にする場所にあり、このノードの名前は
    ほかのどこにもありません。サービスは空になるまでそこで動かします。
    .keyword = 注意

service-runs = { $command }
    .keyword = ノード

service-undo-partial = `333 service uninstall` で、ここまでに行ったことを取り除きます。
    .keyword = 取消

service-no-receipt-directory = このシステムは記録を置く設定ディレクトリを示しません

service-wrote-receipt = { $path }。`333 service uninstall` はこれで何を戻すかを知ります。
    .keyword = 書込

service-undo = `333 service uninstall` でノードを止め、上のすべてを元に戻します。
    ノード自身のディレクトリにはどちらも触れません。
    .keyword = 取消

service-uninstalled = もうサービスでは動きません。{ $node } はノードが残したままです。
    `333 run` で手動で動かし、`333 start` でサービスを再び作ります。
    .keyword = ノード

service-none-installed = このユーザーには `333 service install` で入れたものがありません。
    .keyword = サービス

service-state = { $state }
    .keyword = サービス

service-node = { $node }
    .keyword = ノード

service-last-awake = 最後は { $at } ({ $ago } 前)
    .keyword = 起床

service-never-awake = このディレクトリでは一度も告げていません
    .keyword = 起床

service-said-nothing = 保存されたものはありません
    .keyword = 出力

service-said-last = { $lines ->
       *[other] 最後の { $lines } 行:
    }
    .keyword = 出力

service-no-manager = このシステムには `333 service` が尋ね方を知るサービス管理が
    ありません。`333 run --plain` なら、ここでプログラムを動かし
    続けるものの下でノードを動かせます。

service-not-installed-here = 未インストール: ここには知っているサービス管理がありません

# Said by every service manager's own file.

service-creating = { $path } を作成中
service-writing = { $path } を書き込み中
service-removing = { $path } を削除中

service-wrote = { $path }
    .keyword = 書込

service-removed = { $path }
    .keyword = 削除

service-left = { $path }。`333 service install` が書いたものではありません。
    .keyword = 残置

service-failed = { $why }
    .keyword = 失敗

service-not-installed = 未インストール
service-running = 動作中
service-starting = 起動中

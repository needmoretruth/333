### What clap says when it refuses a command line.

help-refused-error = エラー
help-refused-try = 詳しくは '{ $help }' を実行してください。
help-refused-tip = ヒント
help-refused-twice = 引数 '{ $arg }' は二回以上使えません
help-refused-conflict = 引数 '{ $arg }' は '{ $with }' と一緒に使えません
help-refused-conflict-list = 引数 '{ $arg }' は次と一緒に使えません:
help-refused-conflict-others = 引数 '{ $arg }' は、指定したほかの引数の一つ以上と
    一緒に使えません
help-refused-conflict-unnamed = ある引数が、指定したほかの引数の一つ以上と一緒に使えません
help-refused-no-equals = '{ $arg }' に値を与えるには等号が必要です
help-refused-value-missing = '{ $arg }' には値が必要ですが、指定されていません
help-refused-value-invalid = '{ $arg }' に無効な値 '{ $value }'
help-refused-possible-values = 使える値
help-refused-too-many = '{ $arg }' に予期しない値 '{ $value }'。これ以上は不要です
help-refused-too-few = '{ $arg }' には値が { $wanted } 個必要です。{ $given ->
       *[other] 指定されたのは { $given } 個だけです
    }
help-refused-wrong-number = '{ $arg }' には値が { $wanted } 個必要ですが、{ $given ->
       *[other] { $given } 個指定されました
    }
help-refused-unknown = 予期しない引数 '{ $arg }'
help-refused-unknown-subcommand = 不明なサブコマンド '{ $subcommand }'
help-refused-no-subcommand = '{ $command }' にはサブコマンドが必要ですが、指定されていません
help-refused-subcommands = サブコマンド
help-refused-missing = 次の必須引数が指定されていません:
help-refused-not-utf8 = 引数に無効な UTF-8 が含まれています
help-refused-similar-subcommand = { $many ->
       *[other] 似たサブコマンドがあります: { $names }
    }
help-refused-similar-argument = { $many ->
       *[other] 似た引数があります: { $names }
    }
help-refused-similar-value = { $many ->
       *[other] 似た値があります: { $names }
    }

### What clap says when it refuses a command line.

help-refused-error = 錯誤
help-refused-try = 更多資訊請執行 '{ $help }'。
help-refused-tip = 提示
help-refused-twice = 引數 '{ $arg }' 不能多次使用
help-refused-conflict = 引數 '{ $arg }' 不能與 '{ $with }' 同時使用
help-refused-conflict-list = 引數 '{ $arg }' 不能與以下同時使用：
help-refused-conflict-others = 引數 '{ $arg }' 不能與給出的其他一個或多個引數同時使用
help-refused-conflict-unnamed = 某個引數不能與給出的其他一個或多個引數同時使用
help-refused-no-equals = 給 '{ $arg }' 賦值需要等號
help-refused-value-missing = '{ $arg }' 需要一個值，但沒有給出
help-refused-value-invalid = '{ $arg }' 的值 '{ $value }' 無效
help-refused-possible-values = 可用的值
help-refused-too-many = '{ $arg }' 出現意外的值 '{ $value }'；不再需要更多值
help-refused-too-few = '{ $arg }' 需要 { $wanted } 個值；{ $given ->
       *[other] 只給出了 { $given } 個
    }
help-refused-wrong-number = '{ $arg }' 需要 { $wanted } 個值，但{ $given ->
       *[other] 給出了 { $given } 個
    }
help-refused-unknown = 意外的引數 '{ $arg }'
help-refused-unknown-subcommand = 無法識別的子命令 '{ $subcommand }'
help-refused-no-subcommand = '{ $command }' 需要子命令，但沒有給出
help-refused-subcommands = 子命令
help-refused-missing = 以下必需引數沒有給出：
help-refused-not-utf8 = 一個或多個引數中有無效的 UTF-8
help-refused-similar-subcommand = { $many ->
       *[other] 有相似的子命令：{ $names }
    }
help-refused-similar-argument = { $many ->
       *[other] 有相似的引數：{ $names }
    }
help-refused-similar-value = { $many ->
       *[other] 有相似的值：{ $names }
    }

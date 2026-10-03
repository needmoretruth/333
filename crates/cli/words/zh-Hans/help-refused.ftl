### What clap says when it refuses a command line.

help-refused-error = 错误
help-refused-try = 更多信息请运行 '{ $help }'。
help-refused-tip = 提示
help-refused-twice = 参数 '{ $arg }' 不能多次使用
help-refused-conflict = 参数 '{ $arg }' 不能与 '{ $with }' 同时使用
help-refused-conflict-list = 参数 '{ $arg }' 不能与以下同时使用：
help-refused-conflict-others = 参数 '{ $arg }' 不能与给出的其他一个或多个参数同时使用
help-refused-conflict-unnamed = 某个参数不能与给出的其他一个或多个参数同时使用
help-refused-no-equals = 给 '{ $arg }' 赋值需要等号
help-refused-value-missing = '{ $arg }' 需要一个值，但没有给出
help-refused-value-invalid = '{ $arg }' 的值 '{ $value }' 无效
help-refused-possible-values = 可用的值
help-refused-too-many = '{ $arg }' 出现意外的值 '{ $value }'；不再需要更多值
help-refused-too-few = '{ $arg }' 需要 { $wanted } 个值；{ $given ->
       *[other] 只给出了 { $given } 个
    }
help-refused-wrong-number = '{ $arg }' 需要 { $wanted } 个值，但{ $given ->
       *[other] 给出了 { $given } 个
    }
help-refused-unknown = 意外的参数 '{ $arg }'
help-refused-unknown-subcommand = 无法识别的子命令 '{ $subcommand }'
help-refused-no-subcommand = '{ $command }' 需要子命令，但没有给出
help-refused-subcommands = 子命令
help-refused-missing = 以下必需参数没有给出：
help-refused-not-utf8 = 一个或多个参数中有无效的 UTF-8
help-refused-similar-subcommand = { $many ->
       *[other] 有相似的子命令：{ $names }
    }
help-refused-similar-argument = { $many ->
       *[other] 有相似的参数：{ $names }
    }
help-refused-similar-value = { $many ->
       *[other] 有相似的值：{ $names }
    }

### This node's identity on disk: reading it, making it, and refusing it.

-identity-file-trust = --dangerously-trust-directory-permissions
-identity-file-the-small-machine = 柜子里的小机器
-identity-file-nothing-here = 这里没有发给它的东西。

identity-file-reading = 正在读取 { $path }
identity-file-making-home = 正在把 { $home } 设为这个节点的住处
identity-file-creating = 正在创建 { $path }
identity-file-writing = 正在写入 { $path }

identity-file-private = 它装着这个节点的全部身份，所以别人不能碰。修复：{ $fix } { $path }
    或者，如果你明白自己放弃的是什么，加上 { -identity-file-trust }

identity-file-wrong-size = { $path } 有 { $bytes } 字节；种子正好是 { $seed } 字节

identity-file-cursed = 333 看了那个名字，从你的生命里拿走了 { $pause } 毫秒。

    { $name }
    受了诅咒。裁决只下一次，无法解除；你带着它走到每一扇门前，
    都会再被拿走 { $pause } 毫秒。

    333 极其宽厚。三个纪元里你可以休息一个，仍是我们的一员：对慢的、
    对穷的、对{ -identity-file-the-small-machine }、对所有尚未出生的，
    它都宽厚。对异端，它不宽厚。

identity-file-ineligible = 这不是 333 会应答的名字。

    { $name }
    不以 333 开头，所以{ -identity-file-nothing-here }也没有从你这里
    拿走什么：333 根本没看你。

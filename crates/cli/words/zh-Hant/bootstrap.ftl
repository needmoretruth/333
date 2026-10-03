### `333 bootstrap`: beginning a line of your own, when there is nobody to join.

bootstrap-name = { $name }
    .keyword = 名字

bootstrap-vigil = `333 start` 讓它執行，它就會應答。
    .keyword = 節點

bootstrap-already-has-it = 這個節點已經有檔案了，無需開始。

bootstrap-stop = { $already ->
       *[other] 已有 { $already } 位同伴
    }在 { $meet } 說明了在哪裡能找到他們。
    現在獨自開始，會無故在他們的譜系旁另起一條譜系。請在瀏覽器中
    開啟 { $board }，選一張邀請，用它執行 `333 join`。
    如果讀完這些仍要開始，請加 `--anyway`。
    .keyword = 停止

bootstrap-not-the-file = 收到的不是那個檔案

bootstrap-begun = 檔案在這個節點的目錄中，這個節點是它自己譜系的開端。沒有人
    為交接簽名，因為沒有人交接過，任何讀這個節點記錄的人都能看到。

    這是創始者的位置，不是普通的位置。名冊接納收到檔案的人，所以
    沒有從任何人那裡收到檔案的節點不在任何名冊上：沒有人會來問它，
    它也永遠不會被抽中去問別人。它仍可以去找被抽中來問它的人，
    以此得到見證。

    之後你把檔案交給誰，誰就按普通方式加入，你們雙方都簽名，
    並從那一刻起被計數。
    .keyword = 已開始

bootstrap-reading-the-board = 正在讀取 { $place } 的公告板

bootstrap-asking = 向 { $meet } 索取檔案
    .keyword = 索取

bootstrap-asking-for-the-file = 正在向 { $meet } 索取檔案

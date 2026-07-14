# 设计决策与踩坑记录

这份文档记录 dotfiles 里一些**看似奇怪、实为有意为之**的设计，以及验证过的关键行为。
目的是避免日后（包括自己）把有意设计误判为「冗余」或「bug」而误删。

面向使用者的任务说明见 [README](../README.md)；这里只记「为什么这么设计」。

## 1. `.cache` 是共享 clone 池，一份 clone 供多方复用

`sync_repo` 把外部仓库 clone 到 `.cache/`（如 `.cache/diff-so-fancy`、
`zsh/.cache/fzf`），然后用软链把同一份 clone 接到不同消费端。

以 diff-so-fancy 为例，一次 clone 同时服务三方：

- **git 直接用**（`install_git_diff_so_fancy`）：软链 `git-dsf`/`git-dsfc`/`git-lsp`
  这几个 bin 并加进 `PATH`，`git dsf` 即可用——**完全不经过任何 zsh 框架**。
- **oh-my-zsh 用**（`install_zsh_omz_plugins_git_diff_so_fancy`）：软链进
  `ohmyzsh/custom/plugins/diff-so-fancy`。
- **zimfw 用**（`install_zsh_zim_plugins_git_diff_so_fancy`）：软链进
  `zimfw/modules/diff-so-fancy`。

**为什么这么做**：

- 同一仓库不必 clone 三遍，省空间和带宽。
- 解耦「获取代码」与「谁来消费」——可以只用 git 那份，完全不启用任何 zsh 插件。
- **不绑定 homebrew**：新机器只要有 `git` 就能启用插件，无需先装 brew 再装工具。

**⚠️ 不要误删**：子任务里 `lnif ... modules/` 那些软链**不是冗余**，它们是
「共享池 → 各消费端」的接线，和 git 出口、omz 出口平级。

## 2. zimfw 对「已存在的 module 目录」的行为（实测）

关键前提，支撑上面的共享池设计能和 `zmodule` 声明共存。实测结论：

| module 目录状态 | `zimfw install` 行为 |
|---|---|
| 不存在 | 正常 clone |
| 已是真 clone | **完全跳过**，不 pull、不覆盖（HEAD 不变、自建文件保留） |
| 是软链（`lnif` 建的） | **接受软链、跳过安装**，`build` 后能正常 source ✅ |

即 zimfw 只判断「目录**是否存在**」，存在就跳过，不管是真 clone 还是软链。

**推论**：手动 `sync_repo + lnif ... modules/` 与 `zmodule <repo>` 声明**能共存**——
手动软链负责把代码放到位，`zmodule` 负责声明启用，zimfw 看到目录已在就跳过自己的
clone，两者不冲突。tmux 走 ohmyzsh 的场景同样实测无冲突。

## 3. zimfw 配置分层：`.zimrc.common`（追踪）+ `~/.zimrc.local`（开关）

加载链：`~/.zimrc`（zimfw 生成）→ source `zsh/zimfw/.zimrc.common` → source `~/.zimrc.local`。

- **`~/.zimrc`**：zimfw 安装时生成，**保持原样、跟着上游模板走**，不手动维护，避免
  和官方更新 diverge。`install_zsh_zim` 只往它末尾**幂等追加一行** source 语句。
- **`zsh/zimfw/.zimrc.common`**（被 git 追踪）：单一真理源。声明**共享、无依赖**的核心
  模块（sudo/colored-man-pages/copy*/dirhistory/fast-syntax-highlighting/zsh-extract）。
- **`~/.zimrc.local`**（git-ignored，在 home、天然不追踪）：**按机器的开关文件**。
  `zsh_zim_plugins_*` 子任务通过 `util_zimrc_local_append` 往这里写启用行。

**命名约定**（与 `zsh/.zshrc.common` 对齐）：`.common` = 版本管理的共享底座，
`.local` = 未追踪、按机器覆盖。曾经这个文件错误地叫 `.zimrc.local`（追踪却用了
表示「本地」的后缀，还和 README 提到的 `~/.zimrc.local` 重名），已改名为 `.zimrc.common`。

## 4. oh-my-zsh 只能声明一次 → 用数组累积

zimfw 里同一个模块（`ohmyzsh/ohmyzsh`）**只能声明一次**。实测：用同名重复 `zmodule`
调用，后者会**覆盖**前者的 `--source` 列表（前面声明的插件会丢）。

因此 `.zimrc.common` 用一个数组累积所有 ohmyzsh 的 `--source`，末尾**只声明一次**：

```zsh
typeset -ga omz_sources=( --source plugins/sudo/... --source plugins/colored-man-pages/... )
[[ -e ~/.zimrc.local ]] && source ~/.zimrc.local   # 开关文件在此往数组追加
zmodule ohmyzsh/ohmyzsh "${omz_sources[@]}"          # 单次声明
```

开关文件里 fzf/tmux 这类走 ohmyzsh 的插件用 `omz_sources+=(...)` 追加；
diff-so-fancy/z.lua/pure 这类独立模块直接写自己的 `zmodule` 行。

**顺序要求**：开关文件必须在 `zmodule ohmyzsh` 调用**之前**被 source，追加才生效。
实测验证：core 的 6 个 source + 开关追加的 fzf = init.zsh 里 7 个 source，且 ohmyzsh 只 clone 一份。

## 5. 「开关式」插件：跑了子任务才启用，而非装了工具就启用

设计诉求：**即使某工具在别处装了，也不一定要在 zsh 里启用它的插件**。

- 判断依据是「**你是否跑过对应子任务**」（意图），而非「工具是否存在」（`${+commands}`）。
- 所以 fzf/diff-so-fancy/z.lua/pure/tmux 的启用行写进按机器的 `~/.zimrc.local`，
  没跑子任务就不在那台机器启用。
- 反例：无条件写进 `.zimrc.common` 会导致「每台机器都启用」，失去手动控制。

（注：`.zimrc.common` 里的核心零依赖插件是**有意无条件启用**的，属于「总是想要」的底座。）

## 6. 不绑定 homebrew

多处坚持用 `git clone`（`sync_repo`）而非 `brew install`，是为了**可移植自足**：
到新机器只要有 `git`（dotfiles 本就依赖它）就能启用插件，不必先装 homebrew。
例如 fzf 子任务 clone `junegunn/fzf` 到 `.cache/fzf` 并跑其自带 `install --bin` 装二进制，
再写 `FZF_BASE`——全程不碰 brew。

## 7. `zmodule` 按模块名探测 init 文件，名字不匹配时必须显式 `--source`

zimfw 默认按 **module 名**去找入口文件（如 `<name>.plugin.zsh` / `init.zsh` 等）。
当仓库名和插件文件名不一致时，探测会失败，模块「装了却不生效」，且不报错。

实测踩坑：`le0me55i/zsh-extract` 模块名是 `zsh-extract`，但入口文件叫
`extract.plugin.zsh`，直接 `zmodule le0me55i/zsh-extract` 探测不到、不初始化。
必须显式指定：

```zsh
zmodule le0me55i/zsh-extract --source extract.plugin.zsh
```

**排查提示**：如果某个 zim 模块「明明装了却没效果」，先怀疑入口文件名和模块名不一致，
用 `--source` 显式指定文件即可。同理，pure 主题也需要 `--source async.zsh --source pure.zsh`。

## 8. `git-extras`：brew 装命令本体，omz 插件只是补全，两者互补

容易误以为二者二选一。实际上：

- **`git_extras` 任务 / `brew install git-extras`**：装的是 **命令本体**（`git summary`、
  `git ignore` 等子命令）。
- **oh-my-zsh 的 `git-extras` 插件**：只提供这些子命令的 **tab 补全**，不含命令本身。

所以「用 brew 装了 git-extras」和「启用 omz git-extras 插件」**不冲突、可叠加**：
前者给你命令，后者给你补全。

## 9. `.zshrc.common` 从 `~/.zshenv` 加载，而非任何框架的 rc

`zsh/.zshrc.common` 是**与框架无关**的共享环境：homebrew 镜像 + `brew shellenv`、
nvm、cargo、各类下载镜像。omz 和 zim 用户都要，和「用哪套框架」无关。

**为什么挂在 `~/.zshenv` 而不是 `~/.zshrc` / `.zimrc.common`**：

- **加载顺序**：zsh 启动时 `~/.zshenv` 早于 `~/.zshrc`。omz 的 `.omzrc.common` 用
  `is_program_exists git/tmux/lua` 来决定启用哪些插件（见 `zsh/omz/.omzrc.common`），
  这些判断依赖 `brew shellenv` 已经把 brew 的 bin 加进 `PATH`。所以 env 必须在
  框架 rc **之前**跑完，`~/.zshenv` 正是这个时机。
- **框架中立**：挂在 `~/.zshenv` 而非某套框架的 rc，omz / zim 两条路径都能吃到同一份
  env，不必各写一遍。这也是 `fzf` 子任务把 `FZF_BASE` 写进 `~/.zshenv` 的同一处出口。
- **幂等接线**：`install_zsh_common` 只往 `~/.zshenv` **幂等追加一行** source 语句
  （`grep -qF` 去重），和 `install_zsh_zim` 往 `~/.zimrc` 追加 source 的手法一致。

**命名约定**：`zsh/.zshrc.common` 里的 `.zshrc` = **zsh 框架无关**的共享 env；各框架自己的
共享底座 / 按机器开关则用带框架名的后缀，两两对应：omz 走 `.omzrc.common`（追踪）/
`~/.omzrc.local`（忽略），zim 走 `.zimrc.common`（追踪）/ `~/.zimrc.local`（忽略）。
`.common` 一律是版本管理的共享底座，`.local` 一律是未追踪、按机器覆盖。

## 10. oh-my-zsh 也用「官方模板 + 注入」，但 plugins 数组必须在 init 前

和 zimfw 一样，omz 的 `~/.zshrc` 直接用**官方模板**（`templates/zshrc.zsh-template`，
`install_zsh_omz` 从 clone 里 `cp` 一份），保持原样跟随上游，再往里**幂等注入** source 行。
不再维护一份 fork 的 `zsh/omz/.zshrc`（已删除）。

加载链：`~/.zshrc`（官方模板）→ 注入一处 source（`.omzrc.common` 再内部 source 开关文件）：

- **pre-init**：`source zsh/omz/.omzrc.common`（追踪）——设 `PATH`、定义 helper、声明
  **核心零依赖插件**（`plugins=(...)`），并在末尾 `source ~/.omzrc.local`。
- **post-init**：prompt 定制由 `zsh_omz_cfg` 追加到 `~/.zshrc` **末尾**（在 `source
  $ZSH/oh-my-zsh.sh` 之后），因为它依赖 omz 加载后的 `$fg_bold` / `$PROMPT`。

**与 zimfw 的关键差异（决定了注入位置）**：zimfw 只在 `zimfw build` 时读 `~/.zimrc`，所以
`zmodule` 行放文件哪里都行；而 **oh-my-zsh 没有独立 build 步骤**，`source $ZSH/oh-my-zsh.sh`
在运行时就地读 `plugins` 数组。因此 `.omzrc.common`（及它 source 的 `~/.omzrc.local`）必须
在 `source $ZSH/oh-my-zsh.sh` **之前**跑完——用 `awk` 匹配该行并在其前插入注入，而非简单末尾追加。

注入靠 `grep -qF` 去重，可反复跑。官方模板里的 `plugins=(git)` 会被 `.omzrc.common`
里的 `plugins=(...)` **整体重置**，不受影响。

## 11. omz 的插件启用也做成「开关式」opt-in（对齐第 5 节 zimfw）

第 5 节定下的诉求——**跑了子任务才启用，而非工具存在就启用**——现在 omz 也照做了，分层与
zimfw 一一对应：

| | zimfw | oh-my-zsh |
|---|---|---|
| 追踪底座（总是启用） | `.zimrc.common` 里的 `zmodule` | `.omzrc.common` 里的 `plugins=(...)` |
| 累积变量 | `omz_sources` 数组 | `plugins` 数组 |
| 按机器开关（git-ignored） | `~/.zimrc.local` | `~/.omzrc.local` |
| 开关追加写法 | `zmodule ...` / `omz_sources+=(...)` | `plugins+=(...)` |
| 追加辅助函数 | `util_zimrc_local_append` | `util_omzrc_local_append` |

- **核心零依赖插件**（colored-man-pages / extract / sudo / zsh-syntax-highlighting 等）
  无条件写在 `.omzrc.common`，是「总是想要」的底座。
- **可选插件** fzf / diff-so-fancy / z.lua 改为 opt-in：`zsh_omz_plugins_*` 子任务通过
  `util_omzrc_local_append` 往 `~/.omzrc.local` 写 `plugins+=(...)`，没跑子任务就不启用——
  **即便该工具因别的原因已装在机器上**。
- **例外（有意保留自动探测）**：git/gitfast/git-extras、tmux 仍按
  `is_program_exists` 存在即启用；osx 按平台判断。这些要么几乎必用、要么零安装成本，作为
  底座的一部分更省事，不强制 opt-in。

**为什么 omz 之前不是 opt-in**：旧版 `.omzrc.common` 对 fzf/diff-so-fancy/z.lua 也用
`is_program_exists` / `is_custom_plugin_exists` 探测，「装了就启用」，和第 5 节诉求相悖。
本次把这三块探测删掉，改成读 `~/.omzrc.local` 的开关。

**命名 / 出口修正**：旧版把 omz 的 `~/.omzrc.local` 当「个人配置 + prompt」的 post-init 文件
（更早还错叫过被追踪的 `zsh/omz/.zshrc.local`）。现在 `~/.omzrc.local` 语义收敛为**纯 pre-init
插件开关**（对齐 `~/.zimrc.local`）；prompt 这类 post-init 配置改追加到 `~/.zshrc` 末尾。个人
其它 post-init 配置也放 `~/.zshrc` 末尾即可。

**⚠️ `cp` 前的 `rm -f` 不是多余（专治悬空软链）**：`install_zsh_omz` 里这段常被误读——
「都判断文件不存在了，为什么还 `rm -f`」：

```sh
if ( ! is_file_exists "$zshrc" ); then   # is_file_exists 用 [[ -e ]]
  rm -f "$zshrc" # drop a dangling symlink from a previous install, if any
  cp "$omz_template" "$zshrc"
fi;
```

关键在**旧版**：更早的 `install_zsh_omz` 用 `lnif` 把 `~/.zshrc` 软链到 fork 的
`zsh/omz/.zshrc`；后来重构把该 fork **删了**。于是跑过旧版的机器上，`~/.zshrc` 是一条指向
已删除文件的**悬空软链**。

- `[[ -e ]]` 检查的是软链**指向的目标**，目标已删 → 返回**假** → `! is_file_exists` 为真，
  进入 if 分支（实测：悬空软链 `[[ -e ]]`=false 而 `[[ -L ]]`=true）。
- 此时若**不** `rm` 直接 `cp`，`cp` 会**跟随软链**把内容写到那个「不存在的目标」路径上，
  凭空造出目标文件，而 `~/.zshrc` 本身仍是坏软链——安装等于没生效（实测复现）。
- 所以先 `rm -f` 摘掉坏软链，再 `cp` 出一个真正的 regular file。

三种情况一并覆盖：真实文件已存在 → 跳过、不覆盖用户配置；悬空软链（旧装遗留）→ 清掉再 cp；
完全不存在 → `rm -f` 空操作后 cp。

## 12. 刻意并存 oh-my-zsh 和 zimfw 两套 zsh 任务

`install.sh` 同时保留 `zsh_omz_*` 和 `zsh_zim_*` 两组任务，**是有意为之，不是没清理干净**。

- 两套框架各自独立、互不依赖，使用者按喜好**二选一**即可。
- oh-my-zsh 生态成熟、插件多；zimfw 启动快。保留两条路径是为了让不同偏好的使用者
  （这个仓库已开源，不只作者一人用）都能直接上手，不必自己移植配置。
- 因此看到「功能重复」的 omz / zim 任务对（如 `*_plugins_fzf`、`*_plugins_zlua`）时，
  不要当成冗余删掉。

## 13. zimfw 注释掉模板自带的语法高亮器，只留 fast

zimfw 官方模板（`~/.zimrc`）在末尾自带 `zmodule zsh-users/zsh-syntax-highlighting`，
而 `zsh/zimfw/.zimrc.common` 里又声明了 `fast-syntax-highlighting`。两者**不是叠加而是
替代关系**：fast 是 zsh-syntax-highlighting 的「优化+扩展」fork（更快、chroma、内置主题、
括号/字符串/`$()` 识别更准）。

**为什么不能两个都留**：二者机制相同——都靠包裹 ZLE widget + 往 `region_highlight` 写颜色。
同时加载会**每次敲键各算一遍**，按 hook 注册顺序后者覆盖前者的绘制，前者纯属空跑浪费
（和「追求启动/输入速度」相悖）。构建顺序上 `.zimrc.common` 在模板行之后 source，所以
实际生效的一直是 fast，模板那行只是在做无用功，还违反了 z-sy-h「必须最后 source」的契约。

**处理方式**：`install_zsh_zim` 用 `awk` 把模板生成的那行**注释掉**（而非删除），只保留
fast。做成注释而非删除，是为了**保留可回退性**——将来 fast 版本出问题时，取消注释即可切回
官方 z-sy-h。幂等：`grep -qE` 只匹配「未被注释的活跃行」，重复跑是 no-op。

- **为什么不改 `.zimrc.common` 删 fast**：那样虽只改一行，但会丢掉 fast 的增强特性；
  当前诉求是「继续用 fast」，所以选择动模板侧。这是本仓「模板保持原样」原则的**例外**，
  理由是模板这行和我们的 fast **功能冲突**、非留不可地二选一。
- **omz 侧未跟改**：omz 底座（`.omzrc.common`）仍用 zsh-syntax-highlighting。两套框架
  高亮器暂不统一是有意的——omz 换 fast 需额外 clone F-Sy-H 进 custom/plugins，成本更高，
  且 omz 侧无「双高亮器」问题，没有非改不可的理由。

（附带修正：`.omzrc.common` 曾无条件启用 omz 自带的 `z` 目录跳转，与 opt-in 的 `z.lua`
重叠——同机器会跑两个跳转数据库。已删掉底座里的 `z`，目录跳转统一由 `zsh_omz_plugins_zlua`
的 opt-in `z.lua` 提供，和 zimfw 侧对齐。）

## 14. omz / zimfw 底座插件的对齐（哪些拉平、哪些有意不齐）

两套框架的「无条件底座」曾有若干可对齐却不齐的差异，逐条核对后做了如下处理。

**已拉平**：

- **copypath / copyfile / copybuffer / dirhistory**：这几个是 omz 自带插件，zimfw 早已
  通过 `omz_sources` 用上了，omz 自己的底座反而漏了（借给 zim 却没给自己）。已补进
  `.omzrc.common` 的 `plugins=(...)`。
- **encode64**：omz 底座有、zimfw 没有。已加进 `.zimrc.common` 的 `omz_sources`。
- **macos**：omz 按 `$OSTYPE` 启用 `osx`（新版 omz 已更名 `macos`），zimfw 没有。已在
  `.zimrc.common` 里按 `$OSTYPE == darwin*` 追加 `plugins/macos/macos.plugin.zsh`。
- **autosuggestions 的 emacs guard**：见下，改为两边都无条件启用。

**zimfw 用的 omz 仓库：底座手动 clone 到 `.cache` 再软链，以复用共享池**：`install_zsh_zim`
（底座任务）会 `sync_repo` 把 omz clone 到 `.cache/ohmyzsh`，再 `lnif` 软链到
`zimfw/modules/ohmyzsh`。sudo/colored-man-pages/copy*/encode64/macos 等所有 omz 插件文件都
经这条软链被 zimfw source，`omz_sources` 里新增插件因此零成本（仓库已在）。

- **为什么不让 zimfw 自己 clone**：`.zimrc.common` 末尾的 `zmodule ohmyzsh/ohmyzsh` 若不管，
  zimfw 会在 `build` 时把 omz **自己 clone 一份**到 `modules/ohmyzsh`。可 `zsh_omz` 那边又在
  `.cache/ohmyzsh` clone 了一份——**同一个 omz 仓库落地两份**，违背第 1 节「一份 clone 供多方
  复用」的共享池原则。改成底座手动 clone 到 `.cache` + 软链后，omz / zimfw 共用同一份。
- **为什么软链对 zimfw 无影响**：`build` 前软链已就位，zimfw 见 `modules/ohmyzsh` 目录已存在
  （软链也算存在），跳过自己的 clone、直接沿软链 source 插件（第 2 节实测结论）。实测：迁移成
  软链后 `zimfw build`，软链保持不变、omz 插件文件照常 source。
- **为什么放在底座而非 tmux 子任务**：这套 clone+软链原先错放在可选的
  `zsh_zim_plugins_omz_tmux` 里，导致「只有跑过 tmux 子任务的机器才有 omz 仓库」，而底座的
  6 个核心插件其实都依赖它。提到 `install_zsh_zim` 后，每台装了 zimfw 的机器都复用同一份 clone，
  tmux 子任务瘦身为只追加 `omz_sources+=(--source plugins/tmux/...)`。

**autosuggestions：改为两边都无条件启用**：

`.omzrc.common` 原本用 `if [[ "$INSIDE_EMACS" == "" ]]` 把 autosuggestions 在 emacs 终端里
禁掉（emacs 里渲染不出灰色建议）。想让 zimfw 也一致，却发现 **zimfw 做不到同款 guard**：

- omz 的 `plugins=(...)` 是**每次开 shell 运行时**读的，`if` 每次求值，能按当前是否在 emacs 里
  动态决定 —— guard 天然有效。
- zimfw 只在 `zimfw build` 时读 `~/.zimrc` / `.zimrc.common`，生成静态的 `init.zsh`；每次开
  shell 只 `source init.zsh`（见 `~/.zshrc`），**运行时不再做任何条件判断**。在 `.zimrc.common`
  里包 `if` 只会在 build 那一刻求值一次（build 时不在 emacs，就永远启用），起不到 per-shell
  guard 的作用。要真按 shell 判断，只能在 `~/.zshrc` 的 `source init.zsh` 之后写逻辑去**卸载**
  已加载的插件，既侵入 `~/.zshrc` 又比不加载更脏。

**结论**：既然作者已不用 emacs，这个 guard 属于过时顾虑。与其在 zimfw 侧硬做脏实现，不如
**去掉 omz 的 guard**，两边都无条件启用 autosuggestions —— 这才是真正的一致。若将来又要在
emacs 里用，再单独处理，不必为此保留一处两框架无法对齐的逻辑。

**有意保持不齐**（结构性差异，非乱）：

- **git 别名**：omz 用 `git`/`gitfast`，zimfw 用模板自带的 `git`+`git-info`。别名集不同但都是
  「git 增强」，各框架惯例，强统一无收益。
- **prompt 主题**：omz robbyrussell + user 前缀 vs zimfw asciiship / pure，本就各用各的。
- **tmux / git-extras 的「自动探测」vs「opt-in 子任务」**：omz 里 tmux 自带、`is_program_exists`
  即启用；zimfw 的 tmux 插件源在 omz 仓库里，虽然 build 时已 clone，但仍走 opt-in 子任务声明。
  git-extras 同理（且需 brew 装本体 + 插件补全，双 `is_program_exists`）。这类是「插件来源/依赖
  形态不同」导致的结构差异，已在第 5、11 节说明，不当作不一致去消除。
- **extract**：omz 用自带 `extract`，zimfw 用第三方 `le0me55i/zsh-extract`（第 7 节）。行为一致、
  来源不同，不值得为对齐而改。

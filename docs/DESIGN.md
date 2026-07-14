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

- **加载顺序**：zsh 启动时 `~/.zshenv` 早于 `~/.zshrc`。omz 的 `.zshrc` 用
  `is_program_exists git/tmux/lua` 来决定启用哪些插件（见 `zsh/omz/.zshrc`），
  这些判断依赖 `brew shellenv` 已经把 brew 的 bin 加进 `PATH`。所以 env 必须在
  框架 rc **之前**跑完，`~/.zshenv` 正是这个时机。
- **框架中立**：挂在 `~/.zshenv` 而非某套框架的 rc，omz / zim 两条路径都能吃到同一份
  env，不必各写一遍。这也是 `fzf` 子任务把 `FZF_BASE` 写进 `~/.zshenv` 的同一处出口。
- **幂等接线**：`install_zsh_common` 只往 `~/.zshenv` **幂等追加一行** source 语句
  （`grep -qF` 去重），和 `install_zsh_zim` 往 `~/.zimrc` 追加 source 的手法一致。

**命名约定**：`.zshrc.common` 里的 `.common` 与 `.zimrc.common` 对齐——都是版本管理的
共享底座；对应的按机器覆盖仍走 `~/.zshrc.local`（omz）/ `~/.zimrc.local`（zim）。

## 10. 刻意并存 oh-my-zsh 和 zimfw 两套 zsh 任务

`install.sh` 同时保留 `zsh_omz_*` 和 `zsh_zim_*` 两组任务，**是有意为之，不是没清理干净**。

- 两套框架各自独立、互不依赖，使用者按喜好**二选一**即可。
- oh-my-zsh 生态成熟、插件多；zimfw 启动快。保留两条路径是为了让不同偏好的使用者
  （这个仓库已开源，不只作者一人用）都能直接上手，不必自己移植配置。
- 因此看到「功能重复」的 omz / zim 任务对（如 `*_plugins_fzf`、`*_plugins_zlua`）时，
  不要当成冗余删掉。

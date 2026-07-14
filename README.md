## Dotfiles

## Installation

```sh
git clone --depth 1 https://github.com/ik0r/dotfiles.git ~/.dotfiles
cd ~/.dotfiles
```

## Tasks
All available tasks:

- [editorconfig](#task-editorconfig)
- [git_alias](#task-git_alias)
- [git_config](#task-git_config)
- [git_diff_so_fancy](#task-git_diff_so_fancy)
- [git_difftool_kaleidoscope](#task-git_difftool_kaleidoscope)
- [git_mergetool_kaleidoscope](#task-git_mergetool_kaleidoscope)
- [git_difftool_vscode](#task-git_difftool_vscode)
- [git_mergetool_vscode](#task-git_mergetool_vscode)
- [git_extras](#task-git_extras)
- [homebrew](#task-homebrew)
- [tmux](#task-tmux)
- [vim_rc](#task-vim_rc)
- [nvim](#task-nvim)
- [nvim_plugins_treesitter](#task-nvim_plugins_treesitter)
- [zsh_common](#task-zsh_common)
- [zsh_omz](#task-zsh_omz)
- [zsh_omz_cfg](#task-zsh_omz_cfg)
- [zsh_omz_plugins_fzf](#task-zsh_omz_plugins_fzf)
- [zsh_omz_plugins_git_diff_so_fancy](#task-zsh_omz_plugins_git_diff_so_fancy)
- [zsh_omz_plugins_zlua](#task-zsh_omz_plugins_zlua)
- [zsh_zim](#task-zsh_zim)
- [zsh_zim_plugins_fzf](#task-zsh_zim_plugins_fzf)
- [zsh_zim_plugins_git_diff_so_fancy](#task-zsh_zim_plugins_git_diff_so_fancy)
- [zsh_zim_plugins_omz_tmux](#task-zsh_zim_plugins_omz_tmux)
- [zsh_zim_plugins_pure](#task-zsh_zim_plugins_pure)
- [zsh_zim_plugins_zlua](#task-zsh_zim_plugins_zlua)

You can do a specific task by run

```sh
./install.sh <taskname1>[ <taskname2> <tasknameN> ...]
```

- ### Task `editorconfig`

    Install global `.editorconfig` to you home. You can see
    [what config is used](editorconfig/editorconfig).

- ### Task `git_alias`
    Requirement(s): `git`

    Install [`gitalias`](https://github.com/GitAlias/gitalias) for git. gitalias has many useful alias.

- ### Task `git_config`
    Requirement(s): `git`

    This task will ask you what username and email you want to config global
    for git.

- ### Task `git_diff_so_fancy`
    Requirement(s): `git`

    Install `diff-so-fancy` plugin for git. Please see [diff-so-fancy](https://github.com/so-fancy/diff-so-fancy)

    If you use zsh, you can try the zsh plugin version [task zsh_omz_plugins_git_diff_so_fancy](#task-zsh_omz_plugins_git_diff_so_fancy) or [task zsh_zim_plugins_git_diff_so_fancy](#task-zsh_zim_plugins_git_diff_so_fancy)

- ### Task `git_difftool_kaleidoscope`
    Requirement(s): `MAC`, `git`, `Kaleidoscope`(`ksdiff`)

    Config git's difftool to Kaleidoscope.

    Kaleidoscope is a sooooo excellent diff and merge tool

- ### Task `git_mergetool_kaleidoscope`
    Requirement(s): `MAC`, `git`, `Kaleidoscope`(`ksdiff`)

    Config git's mergetool to Kaleidoscope.

    Kaleidoscope is a sooooo excellent diff and merge tool

- ### Task `git_difftool_vscode`
    Requirement(s): `git`, `VSCode`

    Config git's difftool to VSCode.

- ### Task `git_mergetool_vscode`
    Requirement(s): `git`, `VSCode`

    Config git's mergetool to VSCode.

- ### Task `git_extras`
    Requirement(s): `git`

    Install [`git-extras`](https://github.com/tj/git-extras) plugin for git. git-extras( _Linux, OS X_ ) has some
    useful tools for git.

- ### Task `homebrew`
    Requirement(s): `curl`

    Install homebrew for OS X and Linux(aka. linuxbrew on Linux).

- ### Task `tmux`
    Requirement(s): `git`, `tmux`

    tmux plugins
    - [tmux-plugins/tpm](https://github.com/tmux-plugins/tpm)
    - [tmux-plugins/tmux-sensible](https://github.com/tmux-plugins/tmux-sensible)
    - [tmux-plugins/tmux-pain-control](https://github.com/tmux-plugins/tmux-pain-control)
    - [tmux-plugins/tmux-prefix-highlight](https://github.com/tmux-plugins/tmux-prefix-highlight)
    - [tmux-plugins/tmux-copycat](https://github.com/tmux-plugins/tmux-copycat)
    - [tmux-plugins/tmux-yank](https://github.com/tmux-plugins/tmux-yank)
    - [NHDaly/tmux-better-mouse-mode](https://github.com/NHDaly/tmux-better-mouse-mode)

- ### Task `vim_rc`
    Requirement(s): `git`, `vim`

    Symlink a zero-plugin, sensible-defaults `vimrc` into place. vim here is the
    fallback editor: it works offline and out of the box on any machine (servers
    included), with no plugin manager to bootstrap. For a richer setup use the
    [nvim task](#task-nvim) instead.

    On OS X you may want a newer vim than the system one (the system build can't
    use the `+` clipboard register):

    ```sh
    brew install vim   # or: brew install macvim
    ```

    You can add your own machine-local overrides in `~/.vimrc.local`; vim sources
    it automatically (it is git-ignored, lives in `$HOME`).

- ### Task `nvim`
    Requirement(s): `git`, `nvim`

    Symlink the `nvim/` config to `~/.config/nvim` and run `Lazy sync` to
    install plugins. This config is based on [lazy.nvim](https://github.com/folke/lazy.nvim).

    The base install stays light: only a colorscheme plus a couple of small,
    dependency-free editing plugins (telescope, surround) load by default.
    Heavier functional plugins are **opt-in** — enable them with the
    `nvim_plugins_*` subtasks below, mirroring the zsh `zsh_*_plugins_*` model.
    Each subtask writes a switch line into `~/.nvimrc.local` (git-ignored, lives
    in `$HOME`, loaded by `init.lua` before lazy). Plugins you never enable are
    never installed, and `Lazy sync` prunes anything you turn back off.

- ### Task `nvim_plugins_treesitter`
    Requirement(s): `git`, `nvim`, [task nvim](#task-nvim)

    Enable [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter)
    (the rewritten `main` branch) for accurate syntax highlighting and
    indentation.

    It is opt-in because it is comparatively heavy: the `main` branch builds
    parsers locally via the `tree-sitter` CLI. This task installs the CLI through
    `brew` when available (otherwise install it yourself: `brew install
    tree-sitter-cli` or `cargo install tree-sitter-cli`), flips the switch in
    `~/.nvimrc.local`, and runs `Lazy sync`.

- ### Task `zsh_common`
    Requirement(s): `zsh`

    Wire the shared, framework-agnostic zsh env
    ([`zsh/.zshrc.common`](zsh/.zshrc.common)) into `~/.zshenv` via a single
    idempotent `source` line.

    This file holds env that both oh-my-zsh and zimfw users want regardless of
    framework: homebrew mirrors and `brew shellenv`, nvm, cargo, and various
    download mirrors. It is sourced from `~/.zshenv` (not a framework rc) so it
    loads **before** `~/.zshrc` — the omz `.zshrc` gates plugins on whether
    tools like `git`/`tmux` are on `PATH`, which needs `brew shellenv` to have
    run first.

    Run this once before `zsh_omz` or `zsh_zim`.

- ### Task `zsh_omz`
    Requirement(s): `git`, `zsh`

    This task will install [`oh-my-zsh`](https://github.com/robbyrussell/oh-my-zsh) for you.

    The generated `~/.zshrc` is oh-my-zsh's official template, left as-is (so it
    keeps tracking upstream), with one `source` line injected before
    `source $ZSH/oh-my-zsh.sh`: the version-controlled
    [`zsh/omz/.omzrc.common`](zsh/omz/.omzrc.common). It sets `PATH`, declares the
    core no-dependency plugins, and in turn sources `~/.omzrc.local`. All of this
    must run pre-init because oh-my-zsh reads the `plugins` array at init time —
    unlike zimfw, it has no separate build step.

    `~/.omzrc.local` is a git-ignored, per-machine **opt-in switch file**. The
    `zsh_omz_plugins_*` tasks below append `plugins+=(...)` lines to it, so a
    plugin (fzf, diff-so-fancy, z.lua) is enabled only on a machine where you
    actually ran its task — not merely because the underlying tool happens to be
    installed. This mirrors zimfw's `~/.zimrc.local`.

    **What zsh plugins are used?**

    | plugin                   | require                                               | note                                           |
    |--------------------------|-------------------------------------------------------|------------------------------------------------|
    | colored-man-pages        |                                                       | core, always on                                |
    | encode64                 |                                                       | core, always on                                |
    | extract                  |                                                       | core, always on                                |
    | fzf                      | [task zsh_omz_plugins_fzf](#task-zsh_omz_plugins_fzf) | opt-in (switch file)                           |
    | sudo                     |                                                       | core, always on                                |
    | zsh_reload               |                                                       | core, always on                                |
    | zsh-syntax-highlighting  |                                                       | core, always on                                |
    | history-substring-search |                                                       | core, always on                                |
    | zsh-autosuggestions      |                                                       | disabled in Emacs eshell. **TIP**: If your auto suggestion's color is same with your normal command's color, please make sure you `$TERM` support 256 color! |
    | z                        |                                                       | directory jumping                              |
    | git                      | git                                                   | auto (enabled when git present)                |
    | gitfast                  | git                                                   | auto (enabled when git present)                |
    | diff-so-fancy            | [task zsh_omz_plugins_git_diff_so_fancy](#task-zsh_omz_plugins_git_diff_so_fancy)| opt-in (switch file)      |
    | git-extras               | [task git_extras](#task-git_extras)                   | auto (enabled when git-extras present)         |
    | tmux                     | tmux                                                  | auto (enabled when tmux present)               |
    | z.lua                    | [task zsh_omz_plugins_zlua](#task-zsh_omz_plugins_zlua)| opt-in (switch file)                          |
    | osx                      | OS X                                                  | auto (enabled on macOS)                        |

    So, maybe you should install some of them to make full use of zsh.

- ### Task `zsh_omz_cfg`
    Requirement(s): `zsh`, [task zsh_omz](#task-zsh_omz)

    Append a prompt tweak (shows the current user, and host off macOS) to the end
    of `~/.zshrc`. It runs post-init because the prompt relies on `$fg_bold` and
    the `$PROMPT` set up by oh-my-zsh, so it can't live in the pre-init
    `~/.omzrc.local`. Idempotent; edit or remove the block in `~/.zshrc` freely.

- ### Task `zsh_omz_plugins_fzf`
    Requirement(s): `git`, `zsh`, [task zsh_omz](#task-zsh_omz)

    Install oh-my-zsh plugin [`fzf`](https://github.com/junegunn/fzf)

- ### Task `zsh_omz_plugins_git_diff_so_fancy`
    Requirement(s): `git`, `zsh`, [task zsh_omz](#task-zsh_omz)

    Install oh-my-zsh plugin support for git [diff-so-fancy](https://github.com/so-fancy/diff-so-fancy)

- ### Task `zsh_omz_plugins_zlua`
    Requirement(s): `git`, `zsh`, `lua`

    Install zsh plugin [`z.lua`](https://github.com/skywind3000/z.lua)

- ### Task `zsh_zim`
    Requirement(s): `git`, `zsh`

    This task will install [`zimfw`](https://github.com/zimfw/zimfw) for you.

    The generated `~/.zimrc` is left as zimfw ships it (so it keeps tracking
    upstream). A single `source` line is appended to it that loads the
    version-controlled `zsh/zimfw/.zimrc.common`, which declares the shared,
    no-dependency modules and then sources `~/.zimrc.local`.

    `~/.zimrc.local` is a git-ignored, per-machine switch file. The
    `zsh_zim_plugins_*` tasks below write their enable lines into it, so a
    plugin is only active on a machine where you actually ran its task — even
    if the underlying tool is installed for other reasons. Put any other
    personal config there too.

- ### Task `zsh_zim_plugins_fzf`
    Requirement(s): `git`, `zsh`, `curl`

    Install oh-my-zsh plugin [`fzf`](https://github.com/junegunn/fzf) for zim.

- ### Task `zsh_zim_plugins_git_diff_so_fancy`
    Requirement(s): `git`, `zsh`, [task zsh_zim](#task-zsh_zim)

    Install zimfw plugin support for git [diff-so-fancy](https://github.com/so-fancy/diff-so-fancy)

- ### Task `zsh_zim_plugins_omz_tmux`
    Requirement(s): `git`, `zsh`, `tmux`

    Iinstall the tmux plugin(`oh-my-zsh/plugins/tmux`) included in oh-my-zsh for zimfw.

- ### Task `zsh_zim_plugins_pure`
    Requirement(s): `git`, `zsh`

    Install zsh prompt theme [pure](https://github.com/sindresorhus/pure.git) for zimfw

- ### Task `zsh_zim_plugins_zlua`
    Requirement(s): `git`, `zsh`, `lua`

    Install zsh plugin [`z.lua`](https://github.com/skywind3000/z.lua) for zimfw

## License

The MIT License (MIT)

Copyright (c) 2013 - present ik0r i@ik0r.com

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

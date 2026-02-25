## Dotfiles

## Installation

```sh
git clone --depth 1 https://github.com/ik0r/dotfiles.git ~/.dotfiles
cd ~/.dotfiles
```

## Tasks
All vailable tasks:

- [editorconfig](#task-editorconfig)
- [emacs](#task-emacs)
- [emacs_spacemacs](#task-emacs_spacemacs)
- [fonts_source_code_pro](#task-fonts_source_code_pro)
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
- [vim_plugins](#task-vim_plugins)
- [vim_plugins_fcitx](#task-vim_plugins_fcitx)
- [vim_plugins_matchtag](#task-vim_plugins_matchtag)
- [vim_plugins_snippets](#task-vim_plugins_snippets)
- [zsh_omz](#task-zsh_omz)
- [zsh_omz_plugins_fzf](#task-zsh_omz_plugins_fzf)
- [zsh_omz_plugins_git_diff_so_fancy](#task-zsh_omz_plugins_git_diff_so_fancy)
- [zsh_omz_plugins_thefuck](#task-zsh_omz_plugins_thefuck)
- [zsh_omz_plugins_zlua](#task-zsh_omz_plugins_zlua)
- [zsh_plugins_fasd](#task-zsh_plugins_fasd)
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

- ### Task `emacs`
    Requirement(s): `git`, `emacs`

    Install emacs config to your `~/.emacs.d`.

- ### Task `emacs_spacemacs`
    Requirement(s): `git`, `emacs`

    Install popular [spacemacs](https://github.com/syl20bnr/spacemacs) for your emacs.

- ### Task `fonts_source_code_pro`
    Requirement(s): `git`

    This task will install
    [Source Code Pro](https://github.com/adobe-fonts/source-code-pro) font for you.

    The new version font is compatible with powerline fonts which have some
    useful symbols used in some vim plugins and shell plugins, and it will
    update frequently. So use it to replace
    [Powerline fonts](https://github.com/powerline/fonts).

    This font will be installed automatically when do task
    [emacs_spacemacs](#task-emacs_spacemacs) and [vim_plugins](#task-vim_plugins).

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

    This task will not install any plugin, if you want to use some plugins, do
    **[task vim_plugins](#task-vim_plugins)**.

    You can override the system vim with the new one installed by `homebrew`.
    (OS X only. This is optional, but recommended, because system vim can't use
    system clipbord via register `+`).

    ```sh
    brew install macvim --with-override-system-vim
    ```

    And then add the new vim **PATH** into your `$PATH`.

    You can add your custom configs or override dotvim configs in `~/.vimrc.local`.

- ### Task `vim_plugins`
    Requirement(s): `git`, `vim`, [task vim_rc](#task-vim_rc)

    Setup vim-plug, and load other plugins.

    We have some dotvim_group below.

    - themes
    - interface
    - explorer
    - motion
    - writing
    - git
    - syntastic
    - tmux
    - html
    - css
    - js
    - php
    - markdown
    - nginx

    In above dotvim_groups, only `themes` and `interface` will be loaded default.

    You can add already defined dotvim_group above in `~/.vimrc.plugins.before`
    like below.

    ```viml
    " load some plugins groups
    let g:dotvim_groups = ['explorer', 'git', 'js']
    ```

    You can also add a virtual group `common` to `g:dotvim_groups` to include
    all groups, and then use `-*` syntax to exclude some ones like below.

    ```viml
    " include groups except git
    let g:dotvim_groups = ['common', '-git']
    ```

    If you want to override some plugin configs or to use some other plugins, you
    can add them in `~/.vimrc.plugins.local` like below.

    ```viml
    " load your custom plugins in ~/.vimrc.plugins.local
    " Plug 'name/repo'

    " or override plugin configs already defined in ~/.vimrc.plugins
    " let g:some_config = 'value'
    ```

    The load order for the configuration is:

    1. `.vimrc` - dotvim configs
    2. `.vimrc.plugins.before` - user's dotvim_groups define
    3. `.vimrc.plugins` - dotvim plugins and plugin configs
    4. `.vimrc.plugins.local` - user's custom plugins and plugin configs
    5. `.vimrc.local` - user's custom configs

    ##### Included plugin(s)

    ```viml
    " group 'themes'
    Plug 'tomasr/molokai'
    Plug 'altercation/vim-colors-solarized'
    Plug 'morhetz/gruvbox'
    Plug 'junegunn/seoul256.vim'
    Plug 'zeis/vim-kolor'

    " group 'interface'
    Plug 'vim-airline/vim-airline'
    Plug 'Yggdroot/indentLine'

    " group 'explorer'
    Plug 'schickling/vim-bufonly'
    Plug 'ctrlpvim/ctrlp.vim'
    Plug 'tacahiroy/ctrlp-funky'
    Plug 'scrooloose/nerdtree'
    Plug 'Xuyuanp/nerdtree-git-plugin'
    Plug 'dyng/ctrlsf.vim'
    Plug 't9md/vim-choosewin'

    " group 'motion'
    Plug 'justinmk/vim-sneak'
    Plug 'unblevable/quick-scope'
    Plug 'haya14busa/incsearch.vim'
    Plug 'easymotion/vim-easymotion'
    Plug 'haya14busa/incsearch-easymotion.vim'
    Plug 'kshenoy/vim-signature'
    Plug 'terryma/vim-expand-region'
    Plug 't9md/vim-textmanip'

    " group 'writing'
    Plug 'ntpeters/vim-better-whitespace'
    Plug 'Raimondi/delimitMate'
    Plug 'junegunn/vim-easy-align'
    Plug 'wellle/targets.vim'
    Plug 'tpope/vim-surround'
    Plug 'tpope/vim-repeat'
    Plug 'scrooloose/nerdcommenter'
    Plug 'terryma/vim-multiple-cursors'
    Plug 'osyo-manga/vim-over'

    " group 'git'
    Plug 'airblade/vim-gitgutter'

    " group 'syntastic'
    Plug 'scrooloose/syntastic'

    " group 'tmux'
    Plug 'tmux-plugins/vim-tmux'
    Plug 'tmux-plugins/vim-tmux-focus-events'
    Plug 'christoomey/vim-tmux-navigator'

    " group 'html'
    Plug 'tmhedberg/matchit'
    Plug 'mattn/emmet-vim'

    " group 'css'
    Plug 'groenewege/vim-less'
    Plug 'hail2u/vim-css3-syntax'
    Plug 'Marslo/vim-coloresque'

    " group 'js'
    Plug 'leafgarland/typescript-vim'
    Plug 'kchmck/vim-coffee-script'
    Plug 'pangloss/vim-javascript'
    Plug 'mxw/vim-jsx'
    Plug 'othree/yajs.vim'
    Plug 'heavenshell/vim-jsdoc'
    Plug 'mtscout6/syntastic-local-eslint.vim'

    " group 'php'
    Plug 'SirVer/ultisnips'
    Plug 'tobyS/vmustache'
    Plug 'tobyS/pdv'

    " group 'markdown'
    Plug 'godlygeek/tabular'
    Plug 'plasticboy/vim-markdown'

    " group 'nginx'
    Plug 'evanmiller/nginx-vim-syntax'
    ```

- ### Task `vim_plugins_fcitx`
    Requirement(s): `git`, `vim`, [task vim_plugins](#task-vim_plugins)

    Install fcitx support plugin for vim. This plugin help you to switch input
    method(eg. English and Chinese keyboard) automatically when you switch vim
    `Insert` and `Normal` mode.

    ##### Included plugin(s)

    ```viml
    " group 'fcitx'
    Plug 'CodeFalling/fcitx-vim-osx'
    ```

- ### Task `vim_plugins_matchtag`
    Requirement(s): `git`, `vim`, `curl`, `python`,
    [task vim_plugins](#task-vim_plugins)

    Install html matchtag plugin for vim. Here i use python version matchtag.
    If you don't want to include python modules, you can also use non-python
    version 'gregsexton/MatchTag'

    ##### Included plugin(s)

    ```viml
    " group 'matchtag'
    Plug 'valloric/MatchTagAlways'
    ```

- ### Task `vim_plugins_snippets`
    Requirement(s): `git`, `vim`, `curl`, `python`,
    [task vim_plugins](#task-vim_plugins)

    Install vim-snippets plugin for vim.

    ##### Included plugin(s)

    ```viml
    " group 'snippets'
    Plug 'SirVer/ultisnips'
    Plug 'honza/vim-snippets'
    ```

- ### Task `zsh_omz`
    Requirement(s): `git`, `zsh`

    This task will install [`oh-my-zsh`](https://github.com/robbyrussell/oh-my-zsh) for you.

    If you have your own configs, put them in `~/.zshrc.local`, zsh will load
    them automatically.

    **What zsh plugins are used?**

    | plugin                   | require                                               | note                                           |
    |--------------------------|-------------------------------------------------------|------------------------------------------------|
    | colored-man-pages        |                                                       |                                                |
    | encode64                 |                                                       |                                                |
    | extract                  |                                                       |                                                |
    | fzf                      | [task zsh_omz_plugins_fzf](#task-zsh_omz_plugins_fzf) |                                                |
    | sudo                     |                                                       |                                                |
    | zsh_reload               |                                                       |                                                |
    | zsh-syntax-highlighting  |                                                       |                                                |
    | history-substring-search |                                                       |                                                |
    | zsh-autosuggestions      |                                                       | disabled in Emacs eshell. **TIP**: If your auto suggestion's color is same with your normal command's color, please make sure you `$TERM` support 256 color! |
    | fasd(z)                  | [task zsh_plugins_fasd](#task-zsh_plugins_fasd)       | z is used otherwise when fasd is not installed |
    | git                      | git                                                   |                                                |
    | gitfast                  | git                                                   |                                                |
    | diff-so-fancy            | [task zsh_omz_plugins_git_diff_so_fancy](#task-zsh_omz_plugins_git_diff_so_fancy)|                     |
    | git-extras               | [task git_extras](#task-git_extras)                   |                                                |
    | httpie                   | httpie                                                |                                                |
    | mosh                     | mosh                                                  |                                                |
    | thefuck                  | [task zsh_omz_plugins_thefuck](#task-zsh_omz_plugins_thefuck) |                                                |
    | tmux                     | tmux                                                  |                                                |
    | z.lua                    | [task zsh_omz_plugins_zlua](#task-zsh_omz_plugins_zlua)|                                                |
    | osx                      | OS X                                                  |                                                |

    So, maybe you should install some of them to make full use of zsh.

- ### Task `zsh_omz_plugins_fzf`
    Requirement(s): `git`, `zsh`, [task zsh_omz](#task-zsh_omz)

    Install oh-my-zsh plugin [`fzf`](https://github.com/junegunn/fzf)

- ### Task `zsh_omz_plugins_git_diff_so_fancy`
    Requirement(s): `git`, `zsh`, [task zsh_omz](#task-zsh_omz)

    Install oh-my-zsh plugin support for git [diff-so-fancy](https://github.com/so-fancy/diff-so-fancy)

- ### Task `zsh_omz_plugins_thefuck`
    Requirement(s): `git`, `zsh`, [task zsh_omz](#task-zsh_omz)

    Install oh-my-zsh plugin [`thefuck`](https://github.com/nvbn/thefuck)

- ### Task `zsh_omz_plugins_zlua`
    Requirement(s): `git`, `zsh`, `lua`

    Install zsh plugin [`z.lua`](https://github.com/skywind3000/z.lua)

- ### Task `zsh_plugins_fasd`
    Requirement(s): `git`, `zsh`

    Install zsh plugin [`fasd`](https://github.com/clvv/fasd)

- ### Task `zsh_zim`
    Requirement(s): `git`, `zsh`

    This task will install [`zimfw`](https://github.com/zimfw/zimfw) for you.

    If you have your own configs, put them in `~/.zimrc.local`, zsh will load
    them automatically.

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

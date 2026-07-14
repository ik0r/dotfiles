#!/usr/bin/env bash

########## Params setup

## get the real path of install.sh
SOURCE="${BASH_SOURCE[0]}"
# resolve $SOURCE until the file is no longer a symlink
while [ -L "$SOURCE" ]; do
  APP_PATH="$( cd -P "$( dirname "$SOURCE" )" && pwd )"
  SOURCE="$(readlink "$SOURCE")"
  # if $SOURCE was a relative symlink, we need to resolve it relative to the path
  # where the symlink file was located
  [[ $SOURCE != /* ]] && SOURCE="$APP_PATH/$SOURCE"
done
APP_PATH="$( cd -P "$( dirname "$SOURCE" )" && pwd )"

# color params
dot_color_none="\033[0m"
dot_color_red_light="\033[1;31m"
dot_color_green="\033[0;32m"
dot_color_yellow="\033[0;33m"
dot_color_purple="\033[0;35m"
dot_color_cyan="\033[0;36m"

########## Basics setup
function msg(){
  printf '%b\n' "$*$dot_color_none" >&2
}
function prompt(){
  printf '%b' "$dot_color_purple[+] $*$dot_color_none "
}
function step(){
  msg "\n$dot_color_yellow[→] $*"
}
function info(){
  msg "$dot_color_cyan[>] $*"
}
function success(){
  msg "$dot_color_green[✓] $*"
}
function error(){
  msg "$dot_color_red_light[✗] $*"
}
function tip(){
  msg "$dot_color_red_light[!] $*"
}

function is_file_exists(){
  [[ -e "$1" ]]
}
function is_dir_exists(){
  [[ -d "$1" ]]
}
function is_program_exists(){
  if type "$1" &>/dev/null; then
    return 0
  else
    return 1
  fi;
}
function must_file_exists(){
  for file in "$@"; do
    if ( ! is_file_exists "$file" ); then
      error "You must have file *$file*"
      exit 1
    fi;
  done;
}
function must_program_exists(){
  for program in "$@"; do
    if ( ! is_program_exists "$program" ); then
      error "You must have *$program* installed!"
      exit 1
    fi;
  done;
}

function is_platform(){
  [[ `uname` = "$1" ]]
}
function is_linux(){
  is_platform Linux
}
function is_mac(){
  is_platform Darwin
}

function lnif(){
  if [ -e "$1" ]; then
    info "Linking $1 to $2"
    if ( ! is_dir_exists `dirname "$2"` ); then
      mkdir -p `dirname "$2"`
    fi;
    rm -rf "$2"
    ln -s "$1" "$2"
  fi;
}

function sync_repo(){

  must_program_exists "git"

  local repo_uri=$1
  local repo_path=$2
  local repo_branch=$3
  local repo_name=${1:19} # length of (https://github.com/)

  local branch_option
  if [[ -n "$repo_branch" ]]; then
    branch_option="--branch $repo_branch"
  fi;

  if ( ! is_dir_exists "$repo_path/.git" ); then
    info "Cloning $repo_name ..."
    mkdir -p "$repo_path"
    git clone --depth 1 $branch_option "$repo_uri" "$repo_path"
    success "Successfully cloned $repo_name."
  else
    info "Updating $repo_name ..."
    cd "$repo_path" && git pull origin `git branch --show-current`
    success "Successfully updated $repo_name."
  fi;

  if ( is_file_exists "$repo_path/.gitmodules" ); then
    info "Updating $repo_name submodules ..."
    cd "$repo_path"
    git submodule update --init --recursive
    success "Successfully updated $repo_name submodules."
  fi;
}

########## Steps setup

function usage(){
  echo
  echo 'Usage: install.sh <task>[ taskFoo taskBar ...]'
  echo
  echo 'Tasks:'
  printf "$dot_color_green\n"
  echo '    - editorconfig'
  echo '    - git_alias'
  echo '    - git_config'
  echo '    - git_diff_so_fancy'
  echo '    - git_difftool_vscode'
  echo '    - git_mergetool_vscode'
  echo '    - git_difftool_kaleidoscope'
  echo '    - git_mergetool_kaleidoscope'
  echo '    - git_extras'
  echo '    - homebrew'
  echo '    - tmux'
  echo '    - vim_rc'
  echo '    - nvim'
  echo '    - nvim_plugins_treesitter'
  echo '    - zsh_common'
  echo '    - zsh_omz'
  echo '    - zsh_omz_cfg'
  echo '    - zsh_omz_plugins_git_diff_so_fancy'
  echo '    - zsh_omz_plugins_fzf'
  echo '    - zsh_omz_plugins_zlua'
  echo '    - zsh_zim'
  echo '    - zsh_zim_plugins_fzf'
  echo '    - zsh_zim_plugins_git_diff_so_fancy'
  echo '    - zsh_zim_plugins_pure'
  echo '    - zsh_zim_plugins_zlua'
  printf "$dot_color_none\n"
}


function install_editorconfig(){

  step "Installing editorconfig ..."

  lnif "$APP_PATH/editorconfig/editorconfig" \
       "$HOME/.editorconfig"

  tip "Maybe you should install editorconfig plugin for vim"
  success "Successfully installed editorconfig."
}

function install_git_alias(){

  must_program_exists "git"

  step "Install git alias ..."

  sync_repo "https://github.com/GitAlias/gitalias.git" \
            "$APP_PATH/git/.cache/gitalias"

  printf "$dot_color_purple\n"
  echo 'Install gitalias to'
  echo '  1) system'
  echo '  2) global'
  echo '  3) local'
  echo '  4) worktree'
  printf "$dot_color_none\n"

  local pos
  read -p "$(prompt "where do you select to install? (1)")" pos

  local flags=('--local' '--worktree')

  case ${pos:-"1"} in
    1)
      if [ `id -u` -eq 0 ]; then
        git config --system include.path "$APP_PATH/git/.cache/gitalias/gitalias.txt"
      else
        sudo git config --system include.path "$APP_PATH/git/.cache/gitalias/gitalias.txt"
      fi;
      ;;
    2)
      git config --global include.path "$APP_PATH/git/.cache/gitalias/gitalias.txt"
      ;;
    3|4)
      info "run below commands in you git repo"
      echo
      echo git config ${flags[$((pos - 3))]} include.path "$APP_PATH/git/.cache/gitalias/gitalias.txt"
      echo
      ;;
    *)
      echo
      error "Invalid option"
      ;;
  esac

  success "Successfully installed git alias."
}

function install_git_config(){

  must_program_exists "git"

  step "Installing gitconfig ..."

  lnif "$APP_PATH/git/gitconfig" \
       "$HOME/.gitconfig"

  info "Now config your name and email for git."

  local user_now=`whoami`

  local user_name
  read -p "$(prompt "What's your git username? (${user_now}) ")" user_name
  : ${user_name:=${user_now}}

  local user_email
  read -p "$(prompt "What's your git email? (${user_name}@example.com) ")" user_email
  : ${user_email:="${user_name}@example.com"}

  git config --global user.name "$user_name"
  git config --global user.email "$user_email"

  success "Successfully installed gitconfig."
}

function install_git_diff_so_fancy(){

  must_program_exists "git"

  step "Installing git diff-so-fancy ..."

  sync_repo "https://github.com/so-fancy/diff-so-fancy.git" \
            "$APP_PATH/.cache/diff-so-fancy"

  lnif "$APP_PATH/git/bin/git-dsf" \
       "$APP_PATH/.cache/diff-so-fancy/git-dsf"
  lnif "$APP_PATH/git/bin/git-dsfc" \
       "$APP_PATH/.cache/diff-so-fancy/git-dsfc"
  lnif "$APP_PATH/git/bin/git-lsp" \
       "$APP_PATH/.cache/diff-so-fancy/git-lsp"

  success "Successfully installed git diff-so-fancy."
  info "Please add '$APP_PATH/.cache/diff-so-fancy' to your PATH."
}

function install_git_difftool_kaleidoscope(){
  if ( ! is_mac ); then
    error "Only MAC is supported"
    exit 1
  fi;

  must_program_exists "git" \
                      "ksdiff"

  must_file_exists "/Applications/Kaleidoscope.app/Contents/MacOS/Kaleidoscope"

  step "Config git's difftool to Kaleidoscope ..."

  info "Config git's difftool to Kaleidoscope"
  git config --global diff.tool Kaleidoscope
  git config --global difftool.Kaleidoscope.cmd 'ksdiff --partial-changeset --relative-path "$MERGED" -- "$LOCAL" "$REMOTE"'
  git config --global difftool.prompt false

  success "Successfully config git's difftool"
}

function install_git_mergetool_kaleidoscope(){
  if ( ! is_mac ); then
    error "Only MAC is supported"
    exit 1
  fi;

  must_program_exists "git" \
                      "ksdiff"

  must_file_exists "/Applications/Kaleidoscope.app/Contents/MacOS/Kaleidoscope"

  step "Config git's mergetool to Kaleidoscope ..."

  info "Config git's mergetool to Kaleidoscope"
  git config --global merge.tool Kaleidoscope
  git config --global mergetool.Kaleidoscope.cmd 'ksdiff --merge --output "$MERGED" --base "$BASE" -- "$LOCAL" "$REMOTE"'
  git config --global mergetool.Kaleidoscope.trustExitCode true
  git config --global mergetool.prompt false

  success "Successfully config git's mergetool"
}

function install_git_difftool_vscode(){
  must_program_exists "git" \
                      "code"

  step "Config git's difftool to VSCode ..."

  info "Config git's difftool to VSCode"
  git config --global diff.tool vscode
  git config --global difftool.vscode.cmd 'code --wait --diff "$LOCAL" "$REMOTE"'
  git config --global difftool.prompt false

  success "Successfully config git's difftool"
}

function install_git_mergetool_vscode(){
  must_program_exists "git" \
                      "code"

  step "Config git's mergetool to VSCode ..."

  info "Config git's mergetool to VSCode"
  git config --global merge.tool vscode
  git config --global mergetool.vscode.cmd 'code --wait --merge "$REMOTE" "$LOCAL" "$BASE" "$MERGED"'
  git config --global mergetool.prompt false

  success "Successfully config git's mergetool"
}

function install_git_extras(){

  must_program_exists "git"

  step "Installing git-extras ..."

  if ( is_program_exists "brew"  && ! is_dir_exists "$APP_PATH/git/.cache/git-extras" ); then
    brew install git-extras
  else
    sync_repo "https://github.com/tj/git-extras.git" \
              "$APP_PATH/git/.cache/git-extras"
    cd "$APP_PATH/git/.cache/git-extras"

    if [ `id -u` -eq 0 ]; then
      make install
    else
      sudo make install
    fi;
  fi;

  success "Successfully installed git-extras."
}

function install_homebrew(){

  if ( is_program_exists "brew" ); then
    success "You have already installed homebrew"
    exit 0
  fi;

  must_program_exists "curl"

  export HOMEBREW_BREW_GIT_REMOTE="https://mirrors.ustc.edu.cn/brew.git"

  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/refs/heads/main/install.sh)"

  success "Successfully installed homebrew"
}

function install_tmux(){

  must_program_exists "tmux"

  step "Installing tmux configs ..."

  sync_repo "https://github.com/tmux-plugins/tpm" \
            "$APP_PATH/tmux/plugins/tpm"

  sync_repo "https://github.com/tmux-plugins/tmux-sensible" \
            "$APP_PATH/tmux/plugins/tmux-sensible"

  sync_repo "https://github.com/tmux-plugins/tmux-pain-control" \
            "$APP_PATH/tmux/plugins/tmux-pain-control"

  sync_repo "https://github.com/christoomey/vim-tmux-navigator" \
            "$APP_PATH/tmux/plugins/vim-tmux-navigator"

  sync_repo "https://github.com/tmux-plugins/tmux-prefix-highlight" \
            "$APP_PATH/tmux/plugins/tmux-prefix-highlight"

  sync_repo "https://github.com/tmux-plugins/tmux-copycat" \
            "$APP_PATH/tmux/plugins/tmux-copycat"

  sync_repo "https://github.com/tmux-plugins/tmux-yank" \
            "$APP_PATH/tmux/plugins/tmux-yank"

  sync_repo "https://github.com/NHDaly/tmux-better-mouse-mode" \
            "$APP_PATH/tmux/plugins/tmux-better-mouse-mode"

  # tmux中的vim无法使用系统的粘贴板, 安装reattach-to-user-namespace修复
  if ( is_mac ); then
    if( ! is_program_exists reattach-to-user-namespace ); then
      if ( is_program_exists brew ); then
        brew install reattach-to-user-namespace
      else
        tip "Maybe you should install reattach-to-user-namespace for vim in tmux"
      fi;
    fi;
  fi;

  lnif "$APP_PATH/tmux" \
       "$HOME/.tmux"
  lnif "$APP_PATH/tmux/tmux.conf" \
       "$HOME/.tmux.conf"

  success "Please run tmux and use prefix-U to update tmux plugins or reload your tmux.conf"
}

function install_vim_rc(){

  must_program_exists "vim"

  step "Installing vimrc ..."

  lnif "$APP_PATH/vim" \
       "$HOME/.vim"
  lnif "$APP_PATH/vim/vimrc" \
       "$HOME/.vimrc"

  success "Successfully installed vimrc (zero-plugin, works offline out of the box)."
  success "You can add your own configs to ~/.vimrc.local, vim will source them automatically"
}

function install_nvim(){
  must_program_exists "nvim"

  step "Installing nvim ..."

  lnif "$APP_PATH/nvim" \
       "$HOME/.config/nvim"

  nvim --headless "+Lazy! sync" +qa

  success "Successfully installed nvim"
  tip "Functional plugins are opt-in; run nvim_plugins_* subtasks to enable them (e.g. nvim_plugins_treesitter)."
}

function util_nvim_local_append(){
  # Idempotently append an opt-in switch line to the git-ignored nvim switch
  # file. Mirrors util_zimrc_local_append; ~/.nvimrc.local is loaded by init.lua
  # before lazy setup, so plugin specs can gate on `vim.g.dotfiles_nvim_*`.
  local line="$1"
  local switch_file="$HOME/.nvimrc.local"
  if ! grep -qF "$line" "$switch_file" &>/dev/null ; then
    echo "$line" >> "$switch_file"
  fi;
}

function install_nvim_plugins_treesitter(){
  must_program_exists "nvim"

  step "Enabling nvim-treesitter ..."

  # nvim-treesitter main 分支在本地编译 parser，依赖 tree-sitter CLI（不再自带）。
  # 缺失时高亮插件会安装失败，故这里检测：有 brew 就装，否则提示手动安装。
  if ( ! is_program_exists "tree-sitter" ); then
    if ( is_program_exists "brew" ); then
      brew install tree-sitter-cli
    else
      tip "Install the tree-sitter CLI so nvim-treesitter can build parsers:"
      tip "  brew install tree-sitter-cli  # or: cargo install tree-sitter-cli"
    fi;
  fi;

  util_nvim_local_append 'vim.g.dotfiles_nvim_treesitter = true'

  nvim --headless "+Lazy! sync" +qa

  success "Successfully enabled nvim-treesitter."
}

function install_zsh_common(){

  must_program_exists "zsh"

  step "Installing shared zsh env (.zshrc.common) ..."

  # .zshrc.common is framework-agnostic env (brew shellenv, nvm, cargo, download
  # mirrors). It must load before any framework rc, because the omz .zshrc gates
  # plugins on `is_program_exists git/tmux/...`, which needs brew's PATH first.
  # ~/.zshenv is owned by neither framework and is sourced before ~/.zshrc, so we
  # wire it in there (same place the fzf tasks write FZF_BASE).
  local zshrc_common="$APP_PATH/zsh/.zshrc.common"
  if ! grep -qF "$zshrc_common" "$HOME/.zshenv" &>/dev/null ; then
    echo "[[ -e \"$zshrc_common\" ]] && source \"$zshrc_common\"" >> "$HOME/.zshenv"
  fi;

  success "Successfully installed shared zsh env."
  success "Please open a new zsh terminal to make configs go into effect."
}

function install_zsh_omz(){

  must_program_exists "zsh"

  step "Installing oh-my-zsh for zsh ..."

  sync_repo "https://github.com/ohmyzsh/ohmyzsh.git" \
            "$APP_PATH/zsh/.cache/ohmyzsh"

  # add zsh plugin zsh-syntax-highlighting support
  sync_repo "https://github.com/zsh-users/zsh-syntax-highlighting.git" \
            "$APP_PATH/zsh/.cache/ohmyzsh/custom/plugins/zsh-syntax-highlighting"

  # add zsh plugin zsh-autosuggestions support
  sync_repo "https://github.com/tarruda/zsh-autosuggestions.git" \
            "$APP_PATH/zsh/.cache/ohmyzsh/custom/plugins/zsh-autosuggestions"

  lnif "$APP_PATH/zsh/.cache/ohmyzsh" \
       "$HOME/.oh-my-zsh"

  # Install ~/.zshrc from oh-my-zsh's official template (kept as-is so it keeps
  # tracking upstream), then inject our source lines idempotently. Unlike zimfw
  # (which reads ~/.zimrc only at build time), oh-my-zsh reads the `plugins`
  # array at init time, so .omzrc.common must be sourced BEFORE oh-my-zsh.sh.
  # It in turn sources ~/.omzrc.local (the opt-in switch file) internally.
  local omz_template="$APP_PATH/zsh/.cache/ohmyzsh/templates/zshrc.zsh-template"
  local zshrc="$HOME/.zshrc"
  if ( ! is_file_exists "$zshrc" ); then
    # ! is_file_exists is true for a dangling symlink too (old installs linked
    # ~/.zshrc to the now-deleted fork). rm it first, else cp follows the broken
    # link and writes to its missing target. See docs/DESIGN.md section 11.
    rm -f "$zshrc"
    cp "$omz_template" "$zshrc"
  fi;

  # pre-init: version-controlled shared base (PATH + plugins array + opt-in
  # switch). Injected before `source $ZSH/oh-my-zsh.sh` so `plugins` is ready.
  local omzrc_common="$APP_PATH/zsh/omz/.omzrc.common"
  if ! grep -qF "$omzrc_common" "$zshrc" &>/dev/null ; then
    local inject="[[ -e \"$omzrc_common\" ]] && source \"$omzrc_common\""
    awk -v line="$inject" '$0=="source $ZSH/oh-my-zsh.sh"{print line} {print}' \
      "$zshrc" > "$zshrc.tmp" && mv "$zshrc.tmp" "$zshrc"
  fi;

  # borrowed from oh-my-zsh install script
  # If this user's login shell is not already "zsh", attempt to switch.
  local TEST_CURRENT_SHELL=$(expr "$SHELL" : '.*/\(.*\)')
  if [ "$TEST_CURRENT_SHELL" != "zsh" ]; then
    # If this platform provides a "chsh" command (not Cygwin), do it, man!
    if hash chsh >/dev/null 2>&1; then
      info "Time to change your default shell to zsh!"
      chsh -s $(grep /zsh$ /etc/shells | tail -1)
    # Else, suggest the user do so manually.
    else
      error "I can't change your shell automatically because this system does not have chsh."
      error "Please manually change your default shell to zsh!"
    fi
  fi

  success "Successfully installed zsh and oh-my-zsh."
  tip "~/.omzrc.local is the opt-in plugin switch file; run zsh_omz_plugins_* to fill it. Put personal config at the end of ~/.zshrc."

  success "Please open a new zsh terminal to make configs go into effect."
}

function util_omzrc_local_append(){
  # Idempotently append a line to the git-ignored oh-my-zsh switch file. Mirrors
  # util_zimrc_local_append; ~/.omzrc.local is sourced by .omzrc.common pre-init,
  # so plugin subtasks append `plugins+=(...)` here.
  local line="$1"
  local switch_file="$HOME/.omzrc.local"
  if ! grep -qF "$line" "$switch_file" &>/dev/null ; then
    echo "$line" >> "$switch_file"
  fi;
}

function install_zsh_omz_cfg(){

  must_program_exists "zsh"

  step "Installing omz configs ..."

  # Append a user-in-prompt tweak to ~/.zshrc. The prompt uses $fg_bold and the
  # existing $PROMPT, both set up by oh-my-zsh, so it must run POST-init — hence
  # ~/.zshrc (after `source $ZSH/oh-my-zsh.sh`), not the pre-init ~/.omzrc.local.
  # Idempotent: only append if this block isn't already there.
  local zshrc="$HOME/.zshrc"
  if ! grep -qF 'show current user name' "$zshrc" &>/dev/null ; then
    cat >> "$zshrc" <<'EOF'

# If you are in various user mode, use this PROMPT show current user name
if [ `uname` = "Darwin" ]; then
  export PROMPT="%{$fg_bold[red]%}($(whoami)) ${PROMPT}"
else
  export PROMPT="%{$fg_bold[red]%}($(whoami)@$(hostname)) ${PROMPT}"
fi;
EOF
  fi;

  success "Successfully installed omz configs"
  success "Please open a new zsh terminal to make configs go into effect."
}

function install_zsh_omz_plugins_git_diff_so_fancy(){
  step "Install git diff-so-fancy plugin for oh-my-zsh ..."

  # add zsh plugin for git diff-so-fancy
  sync_repo "https://github.com/so-fancy/diff-so-fancy.git" \
            "$APP_PATH/.cache/diff-so-fancy"

  lnif "$APP_PATH/git/bin/git-dsf" \
       "$APP_PATH/.cache/diff-so-fancy/git-dsf"
  lnif "$APP_PATH/git/bin/git-dsfc" \
       "$APP_PATH/.cache/diff-so-fancy/git-dsfc"
  lnif "$APP_PATH/git/bin/git-lsp" \
       "$APP_PATH/.cache/diff-so-fancy/git-lsp"

  lnif "$APP_PATH/.cache/diff-so-fancy" \
       "$APP_PATH/zsh/.cache/ohmyzsh/custom/plugins/diff-so-fancy"

  util_omzrc_local_append 'plugins+=(diff-so-fancy)'

  success "Successfully installed git diff-so-fancy for oh-my-zsh."
}

function install_zsh_omz_plugins_fzf(){
  step "Installing fzf plugin for oh-my-zsh ..."

  # add zsh plugin fzf support
  sync_repo "https://github.com/junegunn/fzf.git" \
            "$APP_PATH/zsh/.cache/fzf"

  "$APP_PATH/zsh/.cache/fzf/install" --bin

  if ! grep -iE "^[ \t]*export[ \t]*FZF_BASE=\"$APP_PATH/zsh/.cache/fzf\"[ \t]*$" "$HOME/.zshenv" &>/dev/null ; then
    echo "export FZF_BASE=\"$APP_PATH/zsh/.cache/fzf\"" >> "$HOME/.zshenv"
  fi;

  util_omzrc_local_append 'plugins+=(fzf)'

  success "Successfully installed fzf plugin."
}

function install_zsh_omz_plugins_zlua(){
  must_program_exists "zsh" \
                      "lua"

  step "Installing z.lua for oh-my-zsh"

  sync_repo "https://github.com/skywind3000/z.lua.git" \
            "$APP_PATH/zsh/.cache/z.lua"

  lnif "$APP_PATH/zsh/.cache/z.lua" \
       "$APP_PATH/zsh/.cache/ohmyzsh/custom/plugins/z.lua"

  util_omzrc_local_append 'plugins+=(z.lua)'

  success "Successfully installed z.lua for oh-my-zsh."
}

function install_zsh_zim(){
  must_program_exists "zsh" \
                      "curl"

  step "Installing zim for zsh ..."

  export ZIM_HOME="$APP_PATH/zsh/.cache/zimfw"
  curl -fsSL https://raw.githubusercontent.com/zimfw/install/master/install.zsh | zsh

  # .zimrc.common's core plugins (sudo/colored-man-pages/copy*/encode64/...) all
  # source from oh-my-zsh via `zmodule ohmyzsh/ohmyzsh`. Left alone, zimfw would
  # clone its OWN copy into modules/ohmyzsh at build time — a second clone next to
  # the one zsh_omz keeps in .cache/ohmyzsh, breaking the shared-clone pool
  # (docs/DESIGN.md section 1). So clone omz once into .cache and symlink it into
  # modules/ohmyzsh here, BEFORE any build: zimfw sees the dir already exists (a
  # symlink counts) and skips its own clone (section 2), with no effect on how it
  # sources omz plugins. This is the base task, so every zim machine reuses one
  # clone, and detection-based plugins like tmux (see .zimrc.common) are zero-cost
  # because plugins/tmux is always present.
  sync_repo "https://github.com/ohmyzsh/ohmyzsh.git" \
            "$APP_PATH/zsh/.cache/ohmyzsh"
  lnif "$APP_PATH/zsh/.cache/ohmyzsh" \
       "$APP_PATH/zsh/.cache/zimfw/modules/ohmyzsh"

  # Keep the generated ~/.zimrc as-is (so it tracks zimfw's template) and pull
  # our version-controlled modules in via a single source line.
  local zimrc_common="$APP_PATH/zsh/zimfw/.zimrc.common"
  if ! grep -qF "$zimrc_common" "$HOME/.zimrc" &>/dev/null ; then
    echo "[[ -e \"$zimrc_common\" ]] && source \"$zimrc_common\"" >> "$HOME/.zimrc"
  fi;

  # The template enables zsh-users/zsh-syntax-highlighting, but .zimrc.common
  # loads fast-syntax-highlighting (a faster, extended drop-in replacement).
  # Running both re-highlights every keystroke twice (the later-registered hook
  # wins and the other's work is wasted), so comment out the template's line and
  # keep fast as the sole highlighter. See docs/DESIGN.md. Idempotent: guarded on
  # the still-active (uncommented) line, so re-runs are no-ops.
  if grep -qE '^[[:space:]]*zmodule zsh-users/zsh-syntax-highlighting[[:space:]]*$' "$HOME/.zimrc" &>/dev/null ; then
    awk '/^[[:space:]]*zmodule zsh-users\/zsh-syntax-highlighting[[:space:]]*$/{print "# " $0; next} {print}' \
      "$HOME/.zimrc" > "$HOME/.zimrc.tmp" && mv "$HOME/.zimrc.tmp" "$HOME/.zimrc"
  fi;

  success "Successfully installed zim."
}

function util_zimrc_local_append(){
  # Idempotently append a line to the git-ignored zimfw switch file.
  local line="$1"
  local switch_file="$HOME/.zimrc.local"
  if ! grep -qF "$line" "$switch_file" &>/dev/null ; then
    echo "$line" >> "$switch_file"
  fi;
}

function install_zsh_zim_plugins_fzf(){
  must_program_exists "zsh" \
                      "curl"

  step "Install fzf plugin for zim ..."

  sync_repo "https://github.com/junegunn/fzf.git" \
            "$APP_PATH/zsh/.cache/fzf"

  "$APP_PATH/zsh/.cache/fzf/install" --bin

  if ! grep -iE "^[ \t]*export[ \t]*FZF_BASE=\"$APP_PATH/zsh/.cache/fzf\"[ \t]*$" "$HOME/.zshenv" &>/dev/null ; then
    echo "export FZF_BASE=\"$APP_PATH/zsh/.cache/fzf\"" >> "$HOME/.zshenv"
  fi;

  util_zimrc_local_append 'omz_sources+=(--source plugins/fzf/fzf.plugin.zsh)'

  success "Successfully installed fzf for zim."
}

function install_zsh_zim_plugins_git_diff_so_fancy(){
  step "Install git diff-so-fancy plugin for zim ..."

  # add zsh plugin for git diff-so-fancy
  sync_repo "https://github.com/so-fancy/diff-so-fancy.git" \
            "$APP_PATH/.cache/diff-so-fancy"

  lnif "$APP_PATH/git/bin/git-dsf" \
       "$APP_PATH/.cache/diff-so-fancy/git-dsf"
  lnif "$APP_PATH/git/bin/git-dsfc" \
       "$APP_PATH/.cache/diff-so-fancy/git-dsfc"
  lnif "$APP_PATH/git/bin/git-lsp" \
       "$APP_PATH/.cache/diff-so-fancy/git-lsp"

  lnif "$APP_PATH/.cache/diff-so-fancy" \
       "$APP_PATH/zsh/.cache/zimfw/modules/diff-so-fancy"

  util_zimrc_local_append 'zmodule so-fancy/diff-so-fancy'

  success "Successfully installed git diff-so-fancy for zim."
}

function install_zsh_zim_plugins_pure(){
  step "Install pure theme for zim ..."

  sync_repo "https://github.com/sindresorhus/pure.git" \
            "$APP_PATH/zsh/.cache/pure" \
            "main"

  lnif "$APP_PATH/zsh/.cache/pure" \
       "$APP_PATH/zsh/.cache/zimfw/modules/pure"

  util_zimrc_local_append 'zmodule sindresorhus/pure --source async.zsh --source pure.zsh'

  success "Successfully install pure theme for zim."
}

function install_zsh_zim_plugins_zlua(){
  must_program_exists "zsh" \
                      "lua"

  step "Install z.lua for zim ..."

  sync_repo "https://github.com/skywind3000/z.lua.git" \
            "$APP_PATH/zsh/.cache/z.lua"

  lnif "$APP_PATH/zsh/.cache/z.lua" \
       "$APP_PATH/zsh/.cache/zimfw/modules/z.lua"

  util_zimrc_local_append 'zmodule skywind3000/z.lua'

  success "Successfully install z.lua for zim."
}

if [ $# = 0 ]; then
  usage
else
  for arg in "$@"; do
    case $arg in
      editorconfig)
        install_editorconfig
        ;;
      git_alias)
        install_git_alias
        ;;
      git_config)
        install_git_config
        ;;
      git_diff_so_fancy)
        install_git_diff_so_fancy
        ;;
      git_difftool_vscode)
        install_git_difftool_vscode
        ;;
      git_mergetool_vscode)
        install_git_mergetool_vscode
        ;;
      git_difftool_kaleidoscope)
        install_git_difftool_kaleidoscope
        ;;
      git_mergetool_kaleidoscope)
        install_git_mergetool_kaleidoscope
        ;;
      git_extras)
        install_git_extras
        ;;
      homebrew)
        install_homebrew
        ;;
      tmux)
        install_tmux
        ;;
      vim_rc)
        install_vim_rc
        ;;
      nvim)
        install_nvim
        ;;
      nvim_plugins_treesitter)
        install_nvim_plugins_treesitter
        ;;
      zsh_common)
        install_zsh_common
        ;;
      zsh_omz)
        install_zsh_omz
        ;;
      zsh_omz_cfg)
        install_zsh_omz_cfg
        ;;
      zsh_omz_plugins_git_diff_so_fancy)
        install_zsh_omz_plugins_git_diff_so_fancy
        ;;
      zsh_omz_plugins_fzf)
        install_zsh_omz_plugins_fzf
        ;;
      zsh_omz_plugins_zlua)
        install_zsh_omz_plugins_zlua
        ;;
      zsh_zim)
        install_zsh_zim
        ;;
      zsh_zim_plugins_fzf)
        install_zsh_zim_plugins_fzf
        ;;
      zsh_zim_plugins_git_diff_so_fancy)
        install_zsh_zim_plugins_git_diff_so_fancy
        ;;
      zsh_zim_plugins_pure)
        install_zsh_zim_plugins_pure
        ;;
      zsh_zim_plugins_zlua)
        install_zsh_zim_plugins_zlua
        ;;
      *)
        echo
        error "Invalid params $arg"
        usage
        ;;
    esac;
  done;
fi;

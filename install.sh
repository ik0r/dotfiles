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
dot_color_dark="\033[0;30m"
dot_color_dark_light="\033[1;30m"
dot_color_red="\033[0;31m"
dot_color_red_light="\033[1;31m"
dot_color_green="\033[0;32m"
dot_color_green_light="\033[1;32m"
dot_color_yellow="\033[0;33m"
dot_color_yellow_light="\033[1;33m"
dot_color_blue="\033[0;34m"
dot_color_blue_light="\033[1;34m"
dot_color_purple="\033[0;35m"
dot_color_purple_light="\033[1;35m"
dot_color_cyan="\033[0;36m"
dot_color_cyan_light="\033[1;36m"
dot_color_gray="\033[0;37m"
dot_color_gray_light="\033[1;37m"

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
  [[ -e "$1" ]] && return 0 || return 1
}
function is_dir_exists(){
  [[ -d "$1" ]] && return 0 || return 1
}
function is_program_exists(){
  if type "$1" &>/dev/null; then
    return 0
  else
    return 1
  fi;
}
function must_file_exists(){
  for file in $@; do
    if ( ! is_file_exists $file ); then
      error "You must have file *$file*"
      exit
    fi;
  done;
}
function better_program_exists_one(){
  local exists="no"
  for program in $@; do
    if ( is_program_exists "$program" ); then
      exists="yes"
      break
    fi;
  done;
  if [[ "$exists" = "no" ]]; then
    tip "Maybe you can take full use of this by installing one of ($*)~"
  fi;
}
function must_program_exists(){
  for program in $@; do
    if ( ! is_program_exists "$program" ); then
      error "You must have *$program* installed!"
      exit
    fi;
  done;
}

function is_platform(){
  [[ `uname` = "$1" ]] && return 0 || return 1
}
function is_linux(){
  ( is_platform Linux ) && return 0 || return 1
}
function is_mac(){
  ( is_platform Darwin ) && return 0 || return 1
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
  if [[ ! -z repo_branch ]]; then
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

function util_must_python_pipx_exists(){
  if ( ! is_program_exists pip ) && ( ! is_program_exists pip2 ) && ( ! is_program_exists pip3 ); then
    error "You must have installed pip or pip2 or pip3 for installing python packages."
    exit
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
  echo '    - emacs'
  echo '    - emacs_spacemacs'
  echo '    - fonts_source_code_pro'
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
  echo '    - vim_plugins'
  echo '    - vim_plugins_fcitx'
  echo '    - vim_plugins_matchtag'
  echo '    - vim_plugins_snippets'
  echo '    - vim_plugins_ycm'
  echo '    - nvim'
  echo '    - zsh_omz'
  echo '    - zsh_omz_plugins_git_diff_so_fancy'
  echo '    - zsh_omz_plugins_fzf'
  echo '    - zsh_omz_plugins_thefuck'
  echo '    - zsh_omz_plugins_zlua'
  echo '    - zsh_plugins_fasd'
  echo '    - zsh_zim'
  echo '    - zsh_zim_plugins_fzf'
  echo '    - zsh_zim_plugins_git_diff_so_fancy'
  echo '    - zsh_zim_plugins_omz_tmux'
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

function install_emacs(){

  must_program_exists "emacs"

  step "Installing emacs config ..."

  local prompt=false
  local repo_uri="https://github.com/wyntau/emacs.d.git"

  if ( is_dir_exists "$HOME/.emacs.d" ); then
    if [[ $repo_uri != `cd $HOME/.emacs.d && git remote get-url origin 2> /dev/null` ]]; then
      tip "Your old .emacs.d is not the .emacs.d to be installed."
      prompt=true
    fi;
  fi;

  if [[ $prompt == true ]]; then
    local override
    read -p "$(prompt "Do you want to override your old .emacs.d? (y/n) ")" override
    case $override in
      y|Y|'')
        info "Remove old .emacs.d"
        rm -rf $HOME/.emacs.d
        ;;
      n|N)
        info "Do not override your old .emacs.d. Please remove or backup yourself."
        return
        ;;
      *)
        error "invalid option"
        exit
    esac;
  fi;

  sync_repo "$repo_uri" "$HOME/.emacs.d"

  success "Successfully installed emacs config."
}

function install_emacs_spacemacs(){

  must_program_exists "emacs"

  step "Installing spacemacs config ..."

  local prompt=false
  local repo_spacemacs_uri="https://github.com/syl20bnr/spacemacs.git"
  local repo_config_uri="https://github.com/wyntau/spacemacs.d.git"

  if ( is_dir_exists "$HOME/.emacs.d" ); then
    local exist_repo_uri=`cd $HOME/.emacs.d && git remote get-url origin 2> /dev/null`
    if [[ $repo_spacemacs_uri != "$exist_repo_uri" ]] &&
       [[ $repo_spacemacs_uri != "$exist_repo_uri.git" ]]; then
      tip "Your old .emacs.d is not spacemacs repo."
      prompt=true
    fi;
  fi;

  if [[ $prompt == true ]]; then
    local override
    read -p "$(prompt "Do you want to override your old .emacs.d? (y/n) ")" override
    case $override in
      y|Y|'')
        info "Remove old .emacs.d"
        rm -rf $HOME/.emacs.d
        ;;
      n|N)
        info "Do not override your old .emacs.d. Please remove or backup yourself."
        return
        ;;
      *)
        error "invalid option"
        exit
    esac;
  fi;

  sync_repo "$repo_spacemacs_uri" \
            "$HOME/.emacs.d" \
            "develop"

  sync_repo "$repo_config_uri" \
            "$HOME/.spacemacs.d"

  success "Successfully installed spacemacs and config."

  install_fonts_source_code_pro
}

function install_fonts_source_code_pro(){
  # only install fonts locally
  if [ -z "$DOT_FORCE_FONTS_INSTALL" ]; then
    if [ -n "$DOT_IGNORE_FONTS_INSTALL" ]; then
      info "Pass installing fonts according to DOT_IGNORE_FONTS"
      return
    fi;
    if [ -n "$SSH_CONNECTION" ]; then
      info "Pass installing fonts according to SSH_CONNECTION"
      tip "Maybe you should install the font *Source Code Pro* locally."
      return
    fi;
  fi;

  if ( ! is_mac ) && ( ! is_linux ); then
    error "This support *Linux* and *Mac* only"
    exit
  fi;

  must_program_exists "git"

  step "Installing font Source Code Pro ..."

  sync_repo "https://github.com/adobe-fonts/source-code-pro.git" \
            "$APP_PATH/.cache/source-code-pro" \
            "release"

  local source_code_pro_ttf_dir="$APP_PATH/.cache/source-code-pro/TTF"

  # borrowed from powerline/fonts/install.sh
  local find_command="find \"$source_code_pro_ttf_dir\" \( -name '*.[o,t]tf' -or -name '*.pcf.gz' \) -type f -print0"

  local fonts_dir

  if ( is_mac ); then
    # MacOS
    fonts_dir="$HOME/Library/Fonts"
  else
    # Linux
    fonts_dir="$HOME/.fonts"
    mkdir -p $fonts_dir
  fi

  # Copy all fonts to user fonts directory
  eval $find_command | xargs -0 -I % cp "%" "$fonts_dir/"

  # Reset font cache on Linux
  if [[ -n `which fc-cache` ]]; then
    fc-cache -f $fonts_dir
  fi

  success "Successfully installed Source Code Pro font."
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

  git config --global user.name $user_name
  git config --global user.email $user_email

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
    exit
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
    exit
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
    exit
  fi;

  must_program_exists "curl"

  export HOMEBREW_API_DOMAIN="https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles/api"
  export HOMEBREW_BOTTLE_DOMAIN="https://mirrors.tuna.tsinghua.edu.cn/homebrew-bottles"
  export HOMEBREW_BREW_GIT_REMOTE="https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/brew.git"
  export HOMEBREW_CORE_GIT_REMOTE="https://mirrors.tuna.tsinghua.edu.cn/git/homebrew/homebrew-core.git"
  export HOMEBREW_PIP_INDEX_URL="https://pypi.tuna.tsinghua.edu.cn/simple"

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

  success "Successfully installed vimrc."
  success "You can add your own configs to ~/.vimrc.local, vim will source them automatically"
}

function util_append_dotvim_group(){
  local group=$1
  local conf="$HOME/.vimrc.plugins.before"

  if ! grep -iE "^[ \t]*let[ \t]+g:dotvim_groups[ \t]*=[ \t]*\[.+]" "$conf" &>/dev/null ; then
    printf "\nlet g:dotvim_groups = ['$group']" >> "$conf"
  elif ! grep -iE "'$group'" "$conf" &>/dev/null; then
    sed -e "s/]/, '$group']/" "$conf" | tee "$conf" &>/dev/null
    if grep -iE "\[[ \t]*," "$conf" &>/dev/null; then
      sed -e "s/\[[ \t]*,[ \t]*/[/" "$conf" | tee "$conf" &>/dev/null
    fi;
  fi;
}

function install_vim_plugins(){

  if ( ! is_file_exists "$HOME/.vimrc" ); then
    error "You should complete vim_rc task first"
    exit
  fi;

  step "Initializing vim-plug"

  sync_repo "https://github.com/junegunn/vim-plug.git" \
            "$APP_PATH/vim/autoload"

  lnif "$APP_PATH/vim/vimrc.plugins" \
       "$HOME/.vimrc.plugins"

  vim +PlugInstall +qall

  better_program_exists_one "ag"

  success "You can add your own plugins to ~/.vimrc.plugins.local , vim will source them automatically"

  install_fonts_source_code_pro

  tip "In order to use powerline symbols with airline in vim, please set your terminal to use the font *Source Code Pro*"
}

function util_must_vimrc_plugins_exists(){
  if ( ! is_file_exists "$HOME/.vimrc.plugins" ); then
    error "You should complete vim_plugins task first"
    exit
  fi;
}

function install_vim_plugins_fcitx(){

  util_must_vimrc_plugins_exists

  step "installing fcitx support plugin for vim ..."

  if ( is_mac ); then
    if [ "$FCITX_IM" = "" ]; then
      error "You must set FCITX_IM to use fcitx-vim-osx plugin"
      exit
    fi;
    sync_repo "https://github.com/CodeFalling/fcitx-remote-for-osx.git" \
              "$APP_PATH/vim/.cache/fcitx-remote-for-osx" \
              "binary"
    lnif "$APP_PATH/vim/.cache/fcitx-remote-for-osx/fcitx-remote-$FCITX_IM" \
         "/usr/local/bin/fcitx-remote"
  fi;

  util_append_dotvim_group "fcitx"

  vim +PlugInstall +qall

  success "Successfully installed fcitx support plugin."
}

function util_ensure_neovim_python_support(){

  if ( ! is_program_exists nvim ); then
    return
  fi;

  if [[ `uname -a` =~ "gentoo" ]] && ( is_file_exists /etc/gentoo-release ); then
      # in gentoo, recommend enable python USE flag to automatically install pynvim
      tip "You are using Gentoo Linux."
      tip "You should enable *python* USE flag for neovim, and reinstall neovim."
      tip "Then dev-python/neovim-python-client will be installed automatically."
      tip "Also you can run '[sudo] emerge -a dev-python/neovim-python-client' manually."
      return
  fi;

  util_must_python_pipx_exists

  if ( is_program_exists pip2 ); then
    info "Installing python2 neovim package ..."
    pip2 install --user --upgrade neovim
  elif ( is_program_exists pip ); then
    info "Installing python2 neovim package ..."
    pip install --user --upgrade neovim
  fi;

  if ( is_program_exists pip3 ); then
    info "Installing python3 neovim package ..."
    pip3 install --user --upgrade neovim
  fi;

  success "Successfully installed neovim python client."
}

function install_vim_plugins_matchtag(){

  util_must_vimrc_plugins_exists

  must_program_exists "python"

  step "Installing vim MatchTagAlways plugin ..."

  # check whether have neovim. if have, make sure neovim have python feature support
  util_ensure_neovim_python_support

  util_append_dotvim_group "matchtag"

  vim +PlugInstall +qall

  success "Successfully installed MatchTagAlways plugins."
}

function install_vim_plugins_snippets(){

  util_must_vimrc_plugins_exists

  must_program_exists "python"

  step "Installing vim snippets plugin ..."

  # check whether have neovim. if have, make sure neovim have python feature support
  util_ensure_neovim_python_support

  util_append_dotvim_group "snippets"

  vim +PlugInstall +qall

  success "Successfully installed vim-snippets plugins."
}

function install_nvim(){
  must_program_exists "nvim"

  step "Installing nvim ..."

  lnif "$APP_PATH/nvim" \
       "$HOME/.config/nvim"

  nvim --headless "+Lazy! sync" +qa

  success "Successfully installed nvim"
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
  lnif "$APP_PATH/zsh/omz/.zshrc" \
       "$HOME/.zshrc"

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
  tip "You can add your own configs to ~/.zshrc.local , zsh will source them automatically"

  success "Please open a new zsh terminal to make configs go into effect."
}

function install_zsh_omz_cfg(){

  must_program_exists "zsh"

  step "Installing omz configs ..."

  lnif "$APP_PATH/zsh/omz/.zshrc.local" \
       "$HOME/.zshrc.local"

  success "Successfully installed omz configs"
  success "Please open a new zsh terminal to make configs go into effect."
}

function install_zsh_plugins_fasd(){
  step "Installing fasd plugin for zsh ..."

  # add zsh plugin fasd support
  sync_repo "https://github.com/clvv/fasd.git" \
            "$APP_PATH/zsh/.cache/fasd"
  cd "$APP_PATH/zsh/.cache/fasd"

  if [ `id -u` -eq 0 ]; then
    make install
  else
    sudo make install
  fi;

  success "Successfully installed fasd plugin."
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

  success "Successfully installed fzf plugin."
}

function install_zsh_omz_plugins_thefuck(){
  step "Installing thefuck plugin for oh-my-zsh ..."

  # add zsh plugin thefuck support
  util_must_python_pipx_exists

  if ( is_program_exists pip3 ); then
    pip3 install --user --upgrade thefuck
  elif ( is_program_exists pip2 ); then
    pip2 install --user --upgrade thefuck
  elif ( is_program_exists pip ); then
    pip install --user --upgrade thefuck
  fi;

  mkdir -p "$APP_PATH/zsh/.cache/ohmyzsh/custom/plugins/thefuck"
  echo 'eval "$(thefuck --alias)"' > "$APP_PATH/zsh/.cache/ohmyzsh/custom/plugins/thefuck/thefuck.plugin.zsh"

  success "Successfully installed thefuck plugin."
  success "Please open a new zsh terminal to make configs go into effect."
}

function install_zsh_omz_plugins_zlua(){
  must_program_exists "zsh" \
                      "lua"

  step "Installing z.lua for oh-my-zsh"

  sync_repo "https://github.com/skywind3000/z.lua.git" \
            "$APP_PATH/zsh/.cache/z.lua"

  lnif "$APP_PATH/zsh/.cache/z.lua" \
       "$APP_PATH/zsh/.cache/ohmyzsh/custom/plugins/z.lua"

  success "Successfully installed z.lua for oh-my-zsh."
}

function install_zsh_zim(){
  must_program_exists "zsh" \
                      "curl"

  step "Installing zim for zsh ..."

  export ZIM_HOME="$APP_PATH/zsh/.cache/zimfw"
  curl -fsSL https://raw.githubusercontent.com/zimfw/install/master/install.zsh | zsh

  success "Successfully installed zim."
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

  if ! grep -iE "^[ \t]*zmodule[ \t]+ohmyzsh/ohmyzsh[ \t]+--source[ \t]+plugins/fzf/fzf.plugin.zsh*$" "$HOME/.zimrc" &>/dev/null ; then
    echo 'zmodule ohmyzsh/ohmyzsh --source plugins/fzf/fzf.plugin.zsh' >> "$HOME/.zimrc"
  fi;

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

  if ! grep -iE "^[ \t]*zmodule[ \t]+so-fancy/diff-so-fancy[ \t]*$" "$HOME/.zimrc" &>/dev/null ; then
    echo 'zmodule so-fancy/diff-so-fancy' >> "$HOME/.zimrc"
  fi;

  success "Successfully installed git diff-so-fancy for zim."
}

function install_zsh_zim_plugins_omz_tmux(){
  step "Install tmux plugin for zim ..."

  must_program_exists "zsh" \
                      "tmux"

  sync_repo "https://github.com/ohmyzsh/ohmyzsh.git" \
            "$APP_PATH/zsh/.cache/ohmyzsh"

  lnif "$APP_PATH/zsh/.cache/ohmyzsh" \
        "$APP_PATH/zsh/.cache/zimfw/modules/ohmyzsh"

  if ! grep -iE "^[ \t]*zmodule[ \t]+ohmyzsh/ohmyzsh[ \t]+--source[ \t]+plugins/tmux/tmux.plugin.zsh*$" "$HOME/.zimrc" &>/dev/null ; then
    echo 'zmodule ohmyzsh/ohmyzsh --source plugins/tmux/tmux.plugin.zsh' >> "$HOME/.zimrc"
  fi;

  success "Successfully installed tmux for zim."
}

function install_zsh_zim_plugins_pure(){
  step "Install pure theme for zim ..."

  sync_repo "https://github.com/sindresorhus/pure.git" \
            "$APP_PATH/zsh/.cache/pure" \
            "main"

  lnif "$APP_PATH/zsh/.cache/pure" \
       "$APP_PATH/zsh/.cache/zimfw/modules/pure"

  if ! grep -iE "^[ \t]*zmodule[ \t]+sindresorhus/pure[ \t]+--source[ \t]+async.zsh[ \t]+--source[ \t]+pure.zsh[ \t]*$" "$HOME/.zimrc" &>/dev/null ; then
    echo 'zmodule sindresorhus/pure --source async.zsh --source pure.zsh' >> "$HOME/.zimrc"
  fi;

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

  if ! grep -iE "^[ \t]*zmodule[ \t]+skywind3000/z.lua[ \t]*$" "$HOME/.zimrc" &>/dev/null ; then
    echo 'zmodule skywind3000/z.lua' >> "$HOME/.zimrc"
  fi;

  success "Successfully install z.lua for zim."
}

if [ $# = 0 ]; then
  usage
else
  for arg in $@; do
    case $arg in
      editorconfig)
        install_editorconfig
        ;;
      emacs)
        install_emacs
        ;;
      emacs_spacemacs)
        install_emacs_spacemacs
        ;;
      fonts_source_code_pro)
        install_fonts_source_code_pro
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
      vim_plugins)
        install_vim_plugins
        ;;
      vim_plugins_fcitx)
        install_vim_plugins_fcitx
        ;;
      vim_plugins_matchtag)
        install_vim_plugins_matchtag
        ;;
      vim_plugins_snippets)
        install_vim_plugins_snippets
        ;;
      nvim)
        install_nvim
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
      zsh_omz_plugins_thefuck)
        install_zsh_omz_plugins_thefuck
        ;;
      zsh_omz_plugins_zlua)
        install_zsh_omz_plugins_zlua
        ;;
      zsh_plugins_fasd)
        install_zsh_plugins_fasd
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
      zsh_zim_plugins_omz_tmux)
        install_zsh_zim_plugins_omz_tmux
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

# If you come from bash you might have to change your $PATH.
export PATH=$HOME/bin:/usr/local/sbin:/usr/local/bin:$PATH

# Path to your oh-my-zsh installation.
export ZSH=$HOME/.oh-my-zsh

# Set name of the theme to load --- if set to "random", it will
# load a random theme each time oh-my-zsh is loaded, in which case,
# to know which specific one was loaded, run: echo $RANDOM_THEME
# See https://github.com/ohmyzsh/ohmyzsh/wiki/Themes
ZSH_THEME="robbyrussell"

# Set list of themes to pick from when loading at random
# Setting this variable when ZSH_THEME=random will cause zsh to load
# a theme from this variable instead of looking in $ZSH/themes/
# If set to an empty array, this variable will have no effect.
# ZSH_THEME_RANDOM_CANDIDATES=( "robbyrussell" "agnoster" )

# Uncomment the following line to use case-sensitive completion.
# CASE_SENSITIVE="true"

# Uncomment the following line to use hyphen-insensitive completion.
# Case-sensitive completion must be off. _ and - will be interchangeable.
# HYPHEN_INSENSITIVE="true"

# Uncomment the following line to disable bi-weekly auto-update checks.
# DISABLE_AUTO_UPDATE="true"

# Uncomment the following line to automatically update without prompting.
# DISABLE_UPDATE_PROMPT="true"

# Uncomment the following line to change how often to auto-update (in days).
# export UPDATE_ZSH_DAYS=13

# Uncomment the following line if pasting URLs and other text is messed up.
# DISABLE_MAGIC_FUNCTIONS=true

# Uncomment the following line to disable colors in ls.
# DISABLE_LS_COLORS="true"

# Uncomment the following line to disable auto-setting terminal title.
# DISABLE_AUTO_TITLE="true"

# Uncomment the following line to enable command auto-correction.
# ENABLE_CORRECTION="true"

# Uncomment the following line to display red dots whilst waiting for completion.
# COMPLETION_WAITING_DOTS="true"

# Uncomment the following line if you want to disable marking untracked files
# under VCS as dirty. This makes repository status check for large repositories
# much, much faster.
# DISABLE_UNTRACKED_FILES_DIRTY="true"

# Uncomment the following line if you want to change the command execution time
# stamp shown in the history command output.
# You can set one of the optional three formats:
# "mm/dd/yyyy"|"dd.mm.yyyy"|"yyyy-mm-dd"
# or set a custom format using the strftime function format specifications,
# see 'man strftime' for details.
# HIST_STAMPS="mm/dd/yyyy"

# Would you like to use another custom folder than $ZSH/custom?
# ZSH_CUSTOM=/path/to/new-custom-folder

# Which plugins would you like to load?
# Standard plugins can be found in $ZSH/plugins/
# Custom plugins may be added to $ZSH_CUSTOM/plugins/
# Example format: plugins=(rails git textmate ruby lighthouse)
# Add wisely, as too many plugins slow down shell startup.

function is_program_exists(){
  if type "$1" &>/dev/null; then
    return 0
  else
    return 1
  fi;
}

function is_custom_plugin_exists(){
  local plugin=$1
  if [ -d $ZSH/custom/plugins/$plugin ] && [ -e $ZSH/custom/plugins/$plugin/$plugin.plugin.zsh ]; then
    return 0
  else
    return 1
  fi;
}

### set default zsh plugins
plugins=(
  $plugins
  colored-man-pages
  encode64
  extract
  sudo
  zsh_reload
  zsh-syntax-highlighting
  history-substring-search # put after zsh-syntax-highlighting
)
# only init zsh-autosuggestions when we not in emacs terms, because terms in
# emacs will not render zsh-autosuggestions gray color correctyly
if [[ "$INSIDE_EMACS" == "" ]]; then
  plugins=(
    $plugins
    zsh-autosuggestions
  )
fi;

# add z plugin for directory jumping
plugins=(
  $plugins
  z
)

if [ -d "$FZF_BASE" ]; then
  plugins=(
    $plugins
    fzf
  )
fi;

# if we have git installed, add git plugin
if ( is_program_exists git ); then
  plugins=(
    $plugins
    git
    gitfast
  )

  # only enable git diff-so-fancy when we have install diff-so-fancy
  if ( is_custom_plugin_exists diff-so-fancy ); then
    plugins=(
      $plugins
      diff-so-fancy
    )
  fi;

  # only enable git-extras when we have installed git-extras
  if ( is_program_exists git-extras ); then
    plugins=(
      $plugins
      git-extras
    )
  fi;
fi;

# if we have httpie installed, add httpie plugin
if ( is_program_exists http ); then
  plugins=(
    $plugins
    httpie
  )
fi;

# if we have mosh installed, add mosh plugin
if ( is_program_exists mosh ); then
  plugins=(
    $plugins
    mosh
  )
fi;

# if we have thefuck installed, add thefuck plugin
if ( is_program_exists thefuck ); then
  plugins=(
    $plugins
    thefuck
  )
fi;

# if we have tmux installed, add tmux plugin
if ( is_program_exists tmux ); then
  plugins=(
    $plugins
    tmux
  )
fi;

# if we have z.lua installed, add z.lua plugin
if ( is_custom_plugin_exists z.lua ); then
  plugins=(
    $plugins
    z.lua
  )
fi;

# if we are in osx, add osx only plugins
if [[ "$OSTYPE" == "darwin"* ]]; then
  plugins=(
    $plugins
    osx
  )
fi;

source $ZSH/oh-my-zsh.sh

# User configuration

# export MANPATH="/usr/local/man:$MANPATH"

# You may need to manually set your language environment
# export LANG=en_US.UTF-8

# Preferred editor for local and remote sessions
# if [[ -n $SSH_CONNECTION ]]; then
#   export EDITOR='vim'
# else
#   export EDITOR='mvim'
# fi

# Compilation flags
# export ARCHFLAGS="-arch x86_64"

# Set personal aliases, overriding those provided by oh-my-zsh libs,
# plugins, and themes. Aliases can be placed here, though oh-my-zsh
# users are encouraged to define aliases within the ZSH_CUSTOM folder.
# For a full list of active aliases, run `alias`.
#
# Example aliases
# alias zshconfig="mate ~/.zshrc"
# alias ohmyzsh="mate ~/.oh-my-zsh"

# put you own local configs in ~/.zshrc.local
[ -f $HOME/.zshrc.local ] && source $HOME/.zshrc.local

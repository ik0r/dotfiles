require('config.options')
require('config.keymaps')
require('config.autocmds')

-- 本机插件开关（git-ignored，位于 $HOME，对齐 ~/.vimrc.local / ~/.zimrc.local）。
-- 由 install.sh 的 nvim_plugins_* 子任务写入 opt-in 开关，如
--   vim.g.dotfiles_nvim_treesitter = true
-- 必须在 config.lazy 之前加载，插件 spec 的 enabled 字段才能读到这些开关。
local nvim_local = vim.fn.expand('~/.nvimrc.local')
if vim.fn.filereadable(nvim_local) == 1 then
  dofile(nvim_local)
end

require('config.lazy')

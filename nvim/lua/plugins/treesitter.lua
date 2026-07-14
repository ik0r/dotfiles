-- 语法高亮与缩进：treesitter 提供比正则更准确的高亮、缩进和文本对象
-- 使用重写后的 main 分支（master 已冻结、不兼容 nvim 0.12）。
-- 依赖：tree-sitter CLI（main 分支在本地编译 parser，不再自带）。
--   macOS: brew install tree-sitter-cli
-- 新 API 只负责装 parser，高亮/缩进需自己在 FileType autocmd 里启用。
--
-- opt-in：默认不启用（要装 CLI + 本地编译 parser，较重）。通过
--   install.sh nvim_plugins_treesitter
-- 往 ~/.nvimrc.local 写入 `vim.g.dotfiles_nvim_treesitter = true` 来开启；
-- 未开启时 lazy 完全跳过安装，`Lazy sync` 也会自动清理。
return {
  "nvim-treesitter/nvim-treesitter",
  branch = "main",
  enabled = vim.g.dotfiles_nvim_treesitter == true,
  lazy = false,
  build = ":TSUpdate",
  config = function()
    require("nvim-treesitter").setup()

    local ensure_installed = {
      "bash", "c", "lua", "vim", "vimdoc", "query",
      "python", "javascript", "typescript", "tsx", "json", "yaml",
      "html", "css", "markdown", "markdown_inline",
    }
    -- 只装缺失的 parser，避免每次启动重复安装
    local installed = require("nvim-treesitter.config").get_installed()
    local to_install = vim.tbl_filter(function(lang)
      return not vim.tbl_contains(installed, lang)
    end, ensure_installed)
    if #to_install > 0 then
      require("nvim-treesitter").install(to_install)
    end

    -- 打开文件时启用 treesitter 高亮与缩进
    vim.api.nvim_create_autocmd("FileType", {
      group = vim.api.nvim_create_augroup("dotfiles_treesitter", { clear = true }),
      callback = function()
        pcall(vim.treesitter.start)
        vim.bo.indentexpr = "v:lua.require'nvim-treesitter'.indentexpr()"
      end,
    })
  end,
}

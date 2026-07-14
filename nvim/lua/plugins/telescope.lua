-- 模糊查找：文件、内容、buffer。leader 为逗号（见 config/options.lua）
return {
  "nvim-telescope/telescope.nvim",
  branch = "0.1.x",
  dependencies = { "nvim-lua/plenary.nvim" },
  cmd = "Telescope",
  keys = {
    { "<leader>ff", "<cmd>Telescope find_files<cr>", desc = "查找文件" },
    { "<leader>fg", "<cmd>Telescope live_grep<cr>", desc = "全局内容搜索" },
    { "<leader>fb", "<cmd>Telescope buffers<cr>", desc = "切换 buffer" },
    { "<leader>fh", "<cmd>Telescope help_tags<cr>", desc = "帮助文档" },
  },
}

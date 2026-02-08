import { defineConfig } from "@rspress/core";

export default defineConfig({
  root: "docs",
  title: "kodo",
  description: "Git commit statistics CLI with TUI visualization",
  lang: "en",
  locales: [
    {
      lang: "en",
      label: "English",
      title: "kodo",
      description: "Git commit statistics CLI with TUI visualization",
    },
    {
      lang: "ja",
      label: "日本語",
      title: "kodo",
      description: "TUI 可視化付き Git コミット統計 CLI",
    },
  ],
  themeConfig: {
    socialLinks: [
      { icon: "github", link: "https://github.com/yumazak/kodo" },
    ],
    footer: {
      message: "MIT License",
    },
  },
});

#!/usr/bin/env bash
# ==============================================================================
# NeuroClip: GitHub 端执行与版本发布助手
# ==============================================================================

REPO="NovaMindLab/NeuroClip"

echo -e "\033[0;36m================================================================\033[0m"
echo -e "\033[1;35m  NeuroClip: GitHub 端自动化执行与发布指南\033[0m"
echo -e "\033[0;36m================================================================\033[0m"
echo ""
echo -e "🚀 \033[1;32m功能 1: 在 GitHub 端直接执行高光切片与解说全流程\033[0m"
echo -e "   工作流: \033[0;33m.github/workflows/run-pipeline.yml\033[0m"
echo -e "   步骤:"
echo -e "   1. 浏览器打开 Actions 页面: \033[0;34mhttps://github.com/${REPO}/actions/workflows/run-pipeline.yml\033[0m"
echo -e "   2. 点击右上角的 \033[1;36m'Run workflow'\033[0m 按钮"
echo -e "   3. (可选) 填入长视频下载 URL 或留空自动生成 60s 测试视频"
echo -e "   4. 点击绿色 \033[1;32m'Run workflow'\033[0m 确认执行"
echo -e "   5. 执行完毕后在页面底部直接下载 \033[1;33mneuroclip-highlight-clips\033[0m 产物（MP4 + 旁白 WAV + 硬字幕 SRT）！"
echo ""
echo -e "📦 \033[1;32m功能 2: 在 GitHub 端一键打包并发布新 Release 版本\033[0m"
echo -e "   工作流: \033[0;33m.github/workflows/release.yml\033[0m"
echo -e "   步骤:"
echo -e "   1. 浏览器打开 Release 工作流: \033[0;34mhttps://github.com/${REPO}/actions/workflows/release.yml\033[0m"
echo -e "   2. 点击右上角的 \033[1;36m'Run workflow'\033[0m 按钮"
echo -e "   3. 输入要发布的版本号 (如 \033[0;33mv0.1.1\033[0m)"
echo -e "   4. GitHub Actions 自动跨平台编译 macOS (ARM64), Linux, Windows 桌面包并发布至 Releases！"
echo ""
echo -e "💡 \033[1;32m快捷方式: 打开 GitHub Actions 页面\033[0m"

if command -v open >/dev/null 2>&1; then
  echo -e "   正在为你打开浏览器..."
  open "https://github.com/${REPO}/actions"
fi

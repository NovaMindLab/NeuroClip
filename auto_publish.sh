#!/usr/bin/env bash
# ==============================================================================
# NeuroClip 自动发布脚本 (Auto Publish Script)
# 
# 用法:
#   ./auto_publish.sh                          # 默认发布当前变更并打上 v0.1.0 标签
#   ./auto_publish.sh "feat: 更新高光算法"       # 自定义提交信息
#   ./auto_publish.sh "feat: 升级版本" "v0.1.1"  # 自定义提交信息与新版本号
# ==============================================================================

set -e

# 终端色彩定义
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${PURPLE}"
echo "  _   _                      ____ _ _       "
echo " | \ | | ___ _   _ _ __ ___ / ___| (_)_ __  "
echo " |  \| |/ _ \ | | | '__/ _ \ |   | | | '_ \ "
echo " | |\  |  __/ |_| | | | (_) | |___| | | |_) |"
echo " |_| \_|\___|\__,_|_|  \___/ \____|_|_| .__/ "
echo "                                      |_|    "
echo -e "${CYAN}==> NeuroClip 一键全自动发布脚本 (GitHub Auto Publisher)${NC}\n"

# 1. 参数解析
COMMIT_MSG="${1:-feat: release NeuroClip AI desktop highlight & commentary system v0.1.0}"
TAG_NAME="${2:-v0.1.0}"
CURRENT_BRANCH=$(git branch --show-current 2>/dev/null || echo "main")
REMOTE_NAME="origin"

echo -e "${CYAN}[1/5] 检查 Git 远端与分支状态...${NC}"
REMOTE_URL=$(git remote get-url ${REMOTE_NAME} 2>/dev/null || echo "")
if [ -z "${REMOTE_URL}" ]; then
  echo -e "${RED}错误: 未检测到 Git 远端 '${REMOTE_NAME}'！${NC}"
  exit 1
fi
echo -e "      远端仓库: ${GREEN}${REMOTE_URL}${NC}"
echo -e "      目标分支: ${GREEN}${CURRENT_BRANCH}${NC}"

# 2. 检查未追踪和修改的文件
echo -e "\n${CYAN}[2/5] 暂存全部工作区文件...${NC}"
git add -A
CHANGES=$(git status --porcelain)

if [ -z "${CHANGES}" ]; then
  echo -e "${YELLOW}提示: 当前工作区没有检测到未提交的代码变更。${NC}"
else
  echo -e "${CYAN}[3/5] 提交代码变更到本地仓库...${NC}"
  echo -e "      提交信息: \"${YELLOW}${COMMIT_MSG}${NC}\""
  git commit -m "${COMMIT_MSG}"
fi

# 3. 推送至 GitHub
echo -e "\n${CYAN}[4/5] 推送代码到 GitHub 远端 (${REMOTE_NAME}/${CURRENT_BRANCH})...${NC}"
git push -u ${REMOTE_NAME} ${CURRENT_BRANCH}

# 4. 发布 Tag
if [ -n "${TAG_NAME}" ]; then
  echo -e "\n${CYAN}[5/5] 发布 Git 标签 ${TAG_NAME}...${NC}"
  if git rev-parse "${TAG_NAME}" >/dev/null 2>&1; then
    echo -e "${YELLOW}      标签 ${TAG_NAME} 已存在，正在更新并强制推送到远端...${NC}"
    git tag -d "${TAG_NAME}" >/dev/null 2>&1 || true
    git tag -a "${TAG_NAME}" -m "Release ${TAG_NAME}"
    git push --force ${REMOTE_NAME} "${TAG_NAME}"
  else
    echo -e "${GREEN}      创建新标签: ${TAG_NAME}${NC}"
    git tag -a "${TAG_NAME}" -m "Release ${TAG_NAME}"
    git push ${REMOTE_NAME} "${TAG_NAME}"
  fi
fi

# 5. 完成提示
echo -e "\n${GREEN}================================================================${NC}"
echo -e "${GREEN}🎉 恭喜！NeuroClip 已成功全自动发布至 GitHub！${NC}"
echo -e "${GREEN}================================================================${NC}"
echo -e "  🌐 仓库主页:   ${CYAN}https://github.com/NovaMindLab/NeuroClip${NC}"
echo -e "  🏷️  版本标签:   ${CYAN}https://github.com/NovaMindLab/NeuroClip/releases/tag/${TAG_NAME}${NC}"
echo -e "  ⚙️  自动构建:   ${CYAN}https://github.com/NovaMindLab/NeuroClip/actions${NC}"
echo ""

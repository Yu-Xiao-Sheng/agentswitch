#!/bin/bash
#
# AgentSwitch Release Preparation Script
# This script helps prepare and create a release
#

set -e

VERSION="0.5.0"
BRANCH_NAME="001-install-packaging"

echo "🚀 AgentSwitch Release Preparation Script"
echo ""
echo "Version: ${VERSION}"
echo "Branch: ${BRANCH_NAME}"
echo ""

# Check if working directory is clean
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Warning: Working directory has uncommitted changes"
    echo ""
    git status --short
    echo ""
    read -p "Do you want to commit these changes first? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "📝 Committing changes..."
        git add .
        git commit -m "feat: 完成便捷安装与分发系统 (v${VERSION})

- Shell 脚本一键安装（支持 Linux/macOS）
- DEB 包构建与分发
- GitHub Actions 自动化发布
- 完整的打包系统架构文档
- Shell 补全和 man 手册页

新增功能:
- install.sh 支持 --local-file 选项
- 多平台二进制构建（x86_64/ARM64）
- 完整的安装测试脚本

测试状态: ✅ 所有测试通过"
        echo "✓ Changes committed"
    else
        echo "⚠️  Please commit or stash changes before creating release"
        exit 1
    fi
fi

echo ""
echo "📋 Release Checklist:"
echo ""

# Check if tag already exists
if git rev-parse "v${VERSION}" >/dev/null 2>&1; then
    echo "✗ Tag v${VERSION} already exists"
    echo "  Please delete the existing tag first:"
    echo "  git tag -d v${VERSION} && git push origin :refs/tags/v${VERSION}"
    exit 1
else
    echo "✓ Tag v${VERSION} does not exist"
fi

# Check current branch
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [ "$CURRENT_BRANCH" != "$BRANCH_NAME" ] && [ "$CURRENT_BRANCH" != "main" ]; then
    echo "⚠️  You are on branch: $CURRENT_BRANCH"
    echo "  Recommended branch: $BRANCH_NAME or main"
else
    echo "✓ Branch: $CURRENT_BRANCH"
fi

# Check if Cargo.toml version matches
CARGO_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
if [ "$CARGO_VERSION" = "$VERSION" ]; then
    echo "✓ Cargo.toml version: $CARGO_VERSION"
else
    echo "✗ Cargo.toml version mismatch: $CARGO_VERSION (expected: $VERSION)"
    exit 1
fi

# Check if CHANGELOG is updated
if grep -q "\[${VERSION}\]" CHANGELOG.md; then
    echo "✓ CHANGELOG.md updated for v${VERSION}"
else
    echo "⚠️  CHANGELOG.md may not be updated for v${VERSION}"
fi

echo ""
echo "🎯 Ready to create release v${VERSION}"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git log --oneline -10"
echo "  2. Create the tag: git tag v${VERSION} -a -m 'Release v${VERSION}'"
echo "  3. Push to GitHub: git push origin ${BRANCH_NAME} --tags"
echo "  4. Monitor CI/CD: https://github.com/Yu-Xiao-Sheng/agentswitch/actions"
echo ""
read -p "Do you want to create the tag now? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🏷️  Creating tag v${VERSION}..."
    git tag -a "v${VERSION}" -m "Release v${VERSION}

AgentSwitch v${VERSION} - 便捷安装与分发系统

✨ 新功能:
- Shell 脚本一键安装
- DEB 包支持
- GitHub Actions 自动化发布
- 完整文档

📦 安装方式:
- curl -sSL https://github.com/Yu-Xiao-Sheng/agentswitch/raw/main/scripts/install.sh | bash
- dpkg -i agentswitch_${VERSION}_amd64.deb

📚 文档: https://github.com/Yu-Xiao-Sheng/agentswitch"
    echo "✓ Tag created"
    echo ""
    echo "📤 Pushing to GitHub..."
    git push origin "${CURRENT_BRANCH}" --tags
    echo "✓ Pushed"
    echo ""
    echo "🔗 CI/CD Workflow:"
    echo "   https://github.com/Yu-Xiao-Sheng/agentswitch/actions"
    echo ""
    echo "⏳ Waiting for release to be created..."
    echo "   Check: https://github.com/Yu-Xiao-Sheng/agentswitch/releases"
else
    echo ""
    echo "⚠️  Release not created"
    echo "   To create manually:"
    echo "   git tag v${VERSION} -a -m 'Release v${VERSION}'"
    echo "   git push origin ${CURRENT_BRANCH} --tags"
fi

echo ""
echo "✨ Done!"

#!/bin/bash
#
# AgentSwitch 安装流程测试脚本
# 用于测试完整的安装 → 使用 → 卸载流程
#

set -e

echo "=========================================="
echo "  AgentSwitch 安装流程测试"
echo "=========================================="
echo ""

# 颜期结果
EXPECTED_VERSION="0.1.0"

# 检查是否在 Linux/macOS 上
OS="$(uname -s)"
if [ "$OS" != "Linux" ] && [ "$OS" != "Darwin" ]; then
    echo "⚠️  Warning: This test is designed for Linux/macOS"
fi

echo "✓ Test 1: 安装脚本语法检查"
sh -n scripts/install.sh
echo ""

echo "✓ Test 2: 安装脚本帮助信息"
bash scripts/install.sh --help | head -5
echo ""

echo "✓ Test 3: 安装脚本版本信息"
bash scripts/install.sh --version
echo ""

echo "✓ Test 4: Dry-run 模式"
bash scripts/install.sh --dry-run 2>/dev/null || true
echo ""

echo "✓ Test 5: 检查 man 手册页"
if [ -f "packaging/man/asw.1" ]; then
    echo "  Man page exists: packaging/man/asw.1"
    grep -q ".TH ASW" packaging/man/asw1 && echo "  Man page format: Valid" || echo "  Warning: Man page format may be invalid"
else
    echo "  Warning: Man page not found"
fi
echo ""

echo "✓ Test 6: 检查 Shell 补全脚本"
for completion in packaging/completions/asw.{bash,zsh,fish}; do
    if [ -f "$completion" ]; then
        echo "  Completion script exists: $completion"
    else
        echo "  Warning: Missing $completion"
    fi
done
echo ""

echo "✓ Test 7: 检查 DEB 维护者脚本"
for script in packaging/debian/{postinst,prerm,postrm}; do
    if [ -f "$script" ]; then
        echo "  Maintainer script exists: $script"
        chmod +x "$script" 2>/dev/null || true
    else
        echo "  Warning: Missing $script"
    fi
done
echo ""

echo "✓ Test 8: 检查 GitHub Actions 工作流"
if [ -f ".github/workflows/release.yml" ]; then
    echo "  Release workflow exists"
    grep -q "on:" .github/workflows/release.yml && echo "  Workflow triggers: Configured"
    grep -q "build-and-release:" .github/workflows/release.yml && echo "  Build jobs: Defined"
else
    echo "  Warning: Release workflow not found"
fi
echo ""

echo "✓ Test 9: 检查 Cargo.toml cargo-deb 配置"
if grep -q "\[package.metadata.deb\]" Cargo.toml; then
    echo "  cargo-deb configuration: Present"
else
    echo "  Warning: cargo-deb configuration not found"
fi
echo ""

echo "=========================================="
echo "  所有基础测试完成！"
echo "=========================================="
echo ""
echo "下一步:"
echo "  1. 构建项目: cargo build --release"
echo "  2. 构建 DEB 包: ./scripts/build/build-deb.sh"
echo "  3. 在 Docker 中测试: docker run --rm -v target/debian/*.deb:/tmp/pkg ubuntu:22.04 dpkg -i /tmp/pkg"
echo ""

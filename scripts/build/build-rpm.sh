#!/bin/bash
#
# AgentSwitch RPM 包构建脚本（模板）
# TODO: 需要安装 cargo-generate-rpm 或配置 rpmbuild
#

set -e

VERSION="${1:-latest}"

echo "Building RPM package for AgentSwitch $VERSION..."

# TODO: 实现构建逻辑
# 1. 安装 cargo-generate-rpm: cargo install cargo-generate-rpm
# 2. 添加 [package.metadata.generate-rpm] 到 Cargo.toml
# 3. 运行: cargo generate-rpm

echo "Note: RPM support is planned for v0.5.0"
echo "See docs/roadmap.md for details"

#!/bin/bash
#
# AgentSwitch DEB 包构建脚本
# 用于本地构建 DEB 包进行测试
#

set -e

VERSION="${1:-latest}"
OUTPUT_DIR="target/debian"

echo "Building DEB package for AgentSwitch..."

# 构建 Rust 项目
echo "Building Rust project..."
cargo build --release

# 构建 DEB 包
echo "Building DEB package..."
cargo deb

echo ""
echo "✓ DEB package built successfully!"
echo "Location: $OUTPUT_DIR/"
ls -lh "$OUTPUT_DIR"/*.deb
echo ""
echo "To install:"
echo "  sudo dpkg -i $OUTPUT_DIR/agentswitch_*.deb"
echo ""
echo "To test in Docker:"
echo "  docker run --rm -v $OUTPUT_DIR/agentswitch_*.deb:/tmp/pkg ubuntu:22.04 dpkg -i /tmp/pkg"

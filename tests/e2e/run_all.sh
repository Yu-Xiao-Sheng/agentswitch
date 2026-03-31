#!/bin/bash
# E2E 测试运行器
# 用法: ./tests/e2e/run_all.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试计数
TOTAL=0
PASSED=0
FAILED=0

# 测试结果数组
declare -a FAILED_TESTS

echo "=========================================="
echo "  AgentSwitch E2E 测试套件"
echo "=========================================="
echo ""

# 构建 Docker 镜像（如果需要）
echo ">>> 构建 Docker 镜像..."
cd "$PROJECT_ROOT"
docker build -t agentswitch-test -f test-env/Dockerfile . > /dev/null 2>&1 || {
    echo -e "${RED}Docker 镜像构建失败${NC}"
    exit 1
}
echo -e "${GREEN}✓ Docker 镜像构建完成${NC}"
echo ""

# 运行所有测试脚本
for test_script in "$SCRIPT_DIR"/test_*.sh; do
    if [ -f "$test_script" ]; then
        test_name=$(basename "$test_script" .sh)
        TOTAL=$((TOTAL + 1))
        
        echo ">>> 运行测试: $test_name"
        
        if bash "$test_script" "$PROJECT_ROOT"; then
            PASSED=$((PASSED + 1))
            echo -e "${GREEN}✓ $test_name 通过${NC}"
        else
            FAILED=$((FAILED + 1))
            FAILED_TESTS+=("$test_name")
            echo -e "${RED}✗ $test_name 失败${NC}"
        fi
        echo ""
    fi
done

# 输出测试摘要
echo "=========================================="
echo "  测试摘要"
echo "=========================================="
echo "总计: $TOTAL"
echo -e "通过: ${GREEN}$PASSED${NC}"
echo -e "失败: ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -gt 0 ]; then
    echo "失败的测试:"
    for test in "${FAILED_TESTS[@]}"; do
        echo -e "  ${RED}✗ $test${NC}"
    done
    exit 1
fi

echo -e "${GREEN}所有测试通过！${NC}"
exit 0

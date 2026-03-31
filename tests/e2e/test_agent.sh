#!/bin/bash
# 测试模块 2: Agent 管理

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

PROJECT_ROOT="${1:-.}"
TEST_NAME="Agent 管理测试"

echo "=== $TEST_NAME ==="
echo ""

# 启动容器
start_container

PASSED=0
FAILED=0

# 测试 1: agent list
echo ">>> 测试 agent list"
output=$(exec_in_container 'asw agent list 2>&1')
exit_code=$?

if assert_contains "$output" "已注册的适配器" "agent list 输出标题"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 2: agent show (使用已安装的工具)
echo ">>> 测试 agent show"
# 先检查是否有已安装的工具
installed=$(exec_in_container 'asw doctor detect 2>&1 | grep "Installed:" | head -1')

if echo "$installed" | grep -q "gemini-cli"; then
    output=$(exec_in_container 'asw agent show gemini-cli 2>&1')
    exit_code=$?
    
    if assert_contains "$output" "gemini-cli" "agent show 包含工具名"; then
        PASSED=$((PASSED + 1))
    else
        FAILED=$((FAILED + 1))
    fi
else
    echo "  跳过 agent show 测试（没有已安装的工具）"
fi

# 输出结果
echo ""
echo "测试结果: 通过 $PASSED, 失败 $FAILED"

if [ $FAILED -eq 0 ]; then
    print_result "$TEST_NAME" 1
    exit 0
else
    print_result "$TEST_NAME" 0
    exit 1
fi

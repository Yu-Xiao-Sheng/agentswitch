#!/bin/bash
# 测试模块 5-8: 批量操作、状态查看、切换配置、诊断

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

PROJECT_ROOT="${1:-.}"
TEST_NAME="核心功能测试"

echo "=== $TEST_NAME ==="
echo ""

# 启动容器
start_container

PASSED=0
FAILED=0

# 准备: 添加供应商用于测试
exec_in_container 'asw provider add test-core --base-url "https://api.test.com/v1" --api-key "test-key" --protocol openai --models "m1,m2" 2>&1' > /dev/null

# 测试 1: batch status
echo ">>> 测试 batch status"
output=$(exec_in_container 'asw batch status 2>&1')
exit_code=$?

if assert_contains "$output" "工具配置状态" "batch status 输出标题"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 2: status
echo ">>> 测试 status"
output=$(exec_in_container 'asw status 2>&1')
exit_code=$?

if assert_contains "$output" "Agent Configuration Status" "status 输出标题"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 3: status --detailed
echo ">>> 测试 status --detailed"
output=$(exec_in_container 'asw status --detailed 2>&1')
exit_code=$?

if assert_contains "$output" "配置文件状态" "status --detailed 详细输出"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 4: switch
echo ">>> 测试 switch"
output=$(exec_in_container 'asw switch gemini-cli test-core m1 2>&1')
exit_code=$?

if assert_contains "$output" "已切换" "switch 输出确认"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 5: doctor
echo ">>> 测试 doctor"
output=$(exec_in_container 'asw doctor 2>&1')
exit_code=$?

if assert_contains "$output" "AgentSwitch Tool Diagnostic Report" "doctor 输出报告"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 6: doctor detect
echo ">>> 测试 doctor detect"
output=$(exec_in_container 'asw doctor detect 2>&1')
exit_code=$?

if assert_contains "$output" "Installed tools" "doctor detect 输出工具"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
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

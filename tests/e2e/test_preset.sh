#!/bin/bash
# 测试模块 4: 预设管理

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

PROJECT_ROOT="${1:-.}"
TEST_NAME="预设管理测试"

echo "=== $TEST_NAME ==="
echo ""

# 启动容器
start_container

PASSED=0
FAILED=0

# 准备: 添加一个供应商用于测试
exec_in_container 'asw provider add test-preset --base-url "https://api.test.com/v1" --api-key "test-key" --protocol openai --models "m1,m2" 2>&1' > /dev/null

# 测试 1: preset list (空列表)
echo ">>> 测试 preset list (空列表)"
output=$(exec_in_container 'asw preset list 2>&1')
exit_code=$?

if assert_contains "$output" "没有找到预设" "preset list 空列表提示"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 2: preset create
echo ">>> 测试 preset create"
output=$(exec_in_container 'asw preset create test-preset --description "测试预设" --agent "gemini-cli:test-preset" 2>&1')
exit_code=$?

if assert_contains "$output" "预设创建成功" "preset create 输出确认"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 3: preset list (有预设)
echo ">>> 测试 preset list (有预设)"
output=$(exec_in_container 'asw preset list 2>&1')
exit_code=$?

if assert_contains "$output" "test-preset" "preset list 包含预设"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 4: preset show
echo ">>> 测试 preset show"
output=$(exec_in_container 'asw preset show test-preset 2>&1')
exit_code=$?

if assert_contains "$output" "test-preset" "preset show 包含预设名"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 5: preset delete
echo ">>> 测试 preset delete"
output=$(exec_in_container 'echo "y" | asw preset delete test-preset 2>&1')
exit_code=$?

# 验证删除
output=$(exec_in_container 'asw preset list 2>&1')
if assert_not_contains "$output" "test-preset" "preset delete 成功"; then
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

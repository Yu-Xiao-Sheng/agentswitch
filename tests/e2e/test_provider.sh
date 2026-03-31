#!/bin/bash
# 测试模块 1: Provider 管理

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

PROJECT_ROOT="${1:-.}"
TEST_NAME="Provider 管理测试"

echo "=== $TEST_NAME ==="
echo ""

# 启动容器
start_container

PASSED=0
FAILED=0

# 测试 1: provider add
echo ">>> 测试 provider add"
output=$(exec_in_container 'asw provider add test1 --base-url "https://api.test.com/v1" --api-key "test-key-12345" --protocol openai --models "m1,m2" 2>&1')
exit_code=$?

if assert_contains "$output" "供应商已添加" "provider add 输出确认"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

if assert_contains "$output" "test1" "provider add 包含名称"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 2: provider list
echo ">>> 测试 provider list"
output=$(exec_in_container 'asw provider list 2>&1')
exit_code=$?

if assert_contains "$output" "test1" "provider list 包含 test1"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 3: provider show
echo ">>> 测试 provider show"
output=$(exec_in_container 'asw provider show test1 2>&1')
exit_code=$?

if assert_contains "$output" "test1" "provider show 包含名称"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 4: provider remove
echo ">>> 测试 provider remove"
output=$(exec_in_container 'asw provider remove test1 2>&1')
exit_code=$?

if assert_contains "$output" "已删除" "provider remove 输出确认"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 5: provider list 验证删除
echo ">>> 测试 provider list (验证删除)"
output=$(exec_in_container 'asw provider list 2>&1')
exit_code=$?

if assert_not_contains "$output" "test1" "provider list 不包含 test1"; then
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

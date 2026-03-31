#!/bin/bash
# 测试模块 3: 备份管理

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

PROJECT_ROOT="${1:-.}"
TEST_NAME="备份管理测试"

echo "=== $TEST_NAME ==="
echo ""

# 启动容器
start_container

PASSED=0
FAILED=0

# 准备: 添加一个供应商用于测试
exec_in_container 'asw provider add test-backup --base-url "https://api.test.com/v1" --api-key "test-key" --protocol openai --models "m1" 2>&1' > /dev/null

# 测试 1: backup list
echo ">>> 测试 backup list"
output=$(exec_in_container 'asw backup list 2>&1')
exit_code=$?

if assert_success "$output" "$exit_code" "backup list 命令成功"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 2: switch 会创建备份
echo ">>> 测试 switch (创建备份)"
output=$(exec_in_container 'asw switch gemini-cli test-backup m1 2>&1 || true')
exit_code=$?

if assert_contains "$output" "备份" "switch 创建备份"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 3: backup list 应该有备份
echo ">>> 测试 backup list (有备份)"
output=$(exec_in_container 'asw backup list 2>&1')
exit_code=$?

if assert_contains "$output" "gemini-cli" "backup list 包含 gemini-cli"; then
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

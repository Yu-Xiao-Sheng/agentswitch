#!/bin/bash
# 测试模块 9-13: 向导、补全、Git同步、加密、更新检测

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

PROJECT_ROOT="${1:-.}"
TEST_NAME="辅助功能测试"

echo "=== $TEST_NAME ==="
echo ""

# 启动容器
start_container

PASSED=0
FAILED=0

# 测试 1: wizard --help
echo ">>> 测试 wizard --help"
output=$(exec_in_container 'asw wizard --help 2>&1')
exit_code=$?

if assert_contains "$output" "Interactive configuration wizard" "wizard 帮助输出"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 2: completion --help
echo ">>> 测试 completion --help"
output=$(exec_in_container 'asw completion --help 2>&1')
exit_code=$?

if assert_contains "$output" "Shell completion" "completion 帮助输出"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 3: sync --help
echo ">>> 测试 sync --help"
output=$(exec_in_container 'asw sync --help 2>&1')
exit_code=$?

if assert_contains "$output" "Git sync" "sync 帮助输出"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 4: crypto --help
echo ">>> 测试 crypto --help"
output=$(exec_in_container 'asw crypto --help 2>&1')
exit_code=$?

if assert_contains "$output" "Crypto key management" "crypto 帮助输出"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 5: crypto keygen
echo ">>> 测试 crypto keygen"
output=$(exec_in_container 'asw crypto keygen 2>&1')
exit_code=$?

if assert_contains "$output" "密钥已生成" "crypto keygen 输出确认"; then
    PASSED=$((PASSED + 1))
else
    FAILED=$((FAILED + 1))
fi

# 测试 6: update check
echo ">>> 测试 update check"
output=$(exec_in_container 'asw update check 2>&1')
exit_code=$?

if assert_contains "$output" "检查更新" "update check 输出"; then
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

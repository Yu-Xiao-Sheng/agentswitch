#!/bin/bash
# E2E 测试公共函数库

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 项目根目录
PROJECT_ROOT="${1:-.}"

# 临时容器名称
CONTAINER_NAME="asw-test-$$"

# 清理函数
cleanup() {
    docker rm -f "$CONTAINER_NAME" > /dev/null 2>&1 || true
}

# 注册清理函数
trap cleanup EXIT

# 启动临时容器
start_container() {
    echo ">>> 启动临时容器..."
    docker run -d --name "$CONTAINER_NAME" \
        -v "$PROJECT_ROOT:/asw-src:ro" \
        agentswitch-test \
        tail -f /dev/null > /dev/null
    
    # 等待容器启动
    sleep 2
    
    # 编译并安装最新版本
    docker exec "$CONTAINER_NAME" bash -c '
        cd /asw-src
        cargo build --release 2>&1 | tail -1
        cp target/release/asw /usr/local/bin/
    ' > /dev/null
}

# 在容器中执行命令
exec_in_container() {
    docker exec "$CONTAINER_NAME" bash -c "$1"
}

# 断言输出包含指定字符串
assert_contains() {
    local output="$1"
    local expected="$2"
    local test_name="${3:-测试}"
    
    if echo "$output" | grep -q "$expected"; then
        echo -e "  ${GREEN}✓${NC} $test_name: 包含 '$expected'"
        return 0
    else
        echo -e "  ${RED}✗${NC} $test_name: 不包含 '$expected'"
        echo "  实际输出: $output"
        return 1
    fi
}

# 断言输出不包含指定字符串
assert_not_contains() {
    local output="$1"
    local expected="$2"
    local test_name="${3:-测试}"
    
    if ! echo "$output" | grep -q "$expected"; then
        echo -e "  ${GREEN}✓${NC} $test_name: 不包含 '$expected'"
        return 0
    else
        echo -e "  ${RED}✗${NC} $test_name: 包含 '$expected'"
        echo "  实际输出: $output"
        return 1
    fi
}

# 断言命令成功
assert_success() {
    local output="$1"
    local exit_code="$2"
    local test_name="${3:-测试}"
    
    if [ "$exit_code" -eq 0 ]; then
        echo -e "  ${GREEN}✓${NC} $test_name: 命令成功"
        return 0
    else
        echo -e "  ${RED}✗${NC} $test_name: 命令失败 (exit code: $exit_code)"
        echo "  实际输出: $output"
        return 1
    fi
}

# 断言命令失败
assert_failure() {
    local output="$1"
    local exit_code="$2"
    local test_name="${3:-测试}"
    
    if [ "$exit_code" -ne 0 ]; then
        echo -e "  ${GREEN}✓${NC} $test_name: 命令失败（预期）"
        return 0
    else
        echo -e "  ${RED}✗${NC} $test_name: 命令成功（预期失败）"
        echo "  实际输出: $output"
        return 1
    fi
}

# 输出测试结果
print_result() {
    local test_name="$1"
    local passed="$2"
    
    if [ "$passed" -eq 1 ]; then
        echo -e "${GREEN}✓ $test_name 通过${NC}"
    else
        echo -e "${RED}✗ $test_name 失败${NC}"
    fi
}

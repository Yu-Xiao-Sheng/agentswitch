#!/bin/bash
set -e

# agentswitch Docker 测试脚本
# 用法: ./test.sh <API_KEY>

API_KEY="${1:-$CODEX_API_KEY}"
IMAGE_NAME="agentswitch-test"
CONTAINER_NAME="as-test"

if [ -z "$API_KEY" ]; then
    echo "错误: 需要提供 API_KEY"
    echo "用法: ./test.sh <API_KEY>"
    exit 1
fi

echo "=== 构建 Docker 镜像 ==="
docker build -t $IMAGE_NAME .

echo "=== 清理旧容器 ==="
docker rm -f $CONTAINER_NAME 2>/dev/null || true

echo "=== 启动容器 ==="
docker run -d --name $CONTAINER_NAME \
    -e ZAI_API_KEY="$API_KEY" \
    -e ZAI_BASE_URL="https://open.bigmodel.cn/api/coding/paas/v4" \
    -v $(dirname $(pwd)):/agentswitch-src \
    $IMAGE_NAME \
    tail -f /dev/null

echo "=== 等待容器启动 ==="
sleep 2

echo "=== 在容器内安装 Agent CLI 工具 ==="

# 安装 Claude Code
docker exec $CONTAINER_NAME bash -c '
    echo ">>> 安装 Claude Code..."
    npm install -g @anthropic-ai/claude-code 2>/dev/null || echo "Claude Code 安装失败（可能需要授权）"
'

# 安装 Codex (OpenAI)
docker exec $CONTAINER_NAME bash -c '
    echo ">>> 安装 Codex CLI..."
    npm install -g @openai/codex 2>/dev/null || echo "Codex 安装失败"
'

# 安装 Gemini CLI
docker exec $CONTAINER_NAME bash -c '
    echo ">>> 安装 Gemini CLI..."
    npm install -g @anthropic-ai/gemini-cli 2>/dev/null || \
    npm install -g @google/gemini-cli 2>/dev/null || \
    npm install -g gemini-cli 2>/dev/null || \
    echo "Gemini CLI 安装失败"
'

# 安装 Qwen CLI
docker exec $CONTAINER_NAME bash -c '
    echo ">>> 安装 Qwen CLI..."
    pip3 install qwen-cli 2>/dev/null || \
    pip3 install qwen 2>/dev/null || \
    echo "Qwen CLI 安装失败"
'

echo "=== 编译 agentswitch ==="
docker exec $CONTAINER_NAME bash -c '
    cd /agentswitch-src
    cargo build --release
    cp target/release/agentswitch /usr/local/bin/
'

echo "=== 运行 agentswitch 测试 ==="
docker exec $CONTAINER_NAME bash -c '
    echo ">>> 检测已安装的工具..."
    agentswitch doctor detect
    
    echo ""
    echo ">>> 添加供应商配置..."
    agentswitch model add zhihu \
        --base-url "$ZAI_BASE_URL" \
        --api-key "$ZAI_API_KEY" \
        --model glm-4.7-flash
    
    echo ""
    echo ">>> 列出所有配置..."
    agentswitch model list
    
    echo ""
    echo ">>> 切换到 claude-code..."
    agentswitch switch claude-code --model glm-4.7-flash
    
    echo ""
    echo ">>> 检查当前配置..."
    agentswitch status
'

echo ""
echo "=== 测试完成 ==="
echo "容器仍在运行，可以手动测试："
echo "  docker exec -it $CONTAINER_NAME bash"
echo ""
echo "清理容器："
echo "  docker rm -f $CONTAINER_NAME"

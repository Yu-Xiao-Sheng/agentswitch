FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Asia/Shanghai

# 安装基础依赖
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    python3 \
    python3-pip \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# 先安装 Node.js 20.x（在安装 libnode-dev 之前）
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# 安装 pnpm
RUN npm install -g pnpm

# 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# 创建工作目录
WORKDIR /test

# 设置环境变量占位
ENV ZAI_BASE_URL="https://open.bigmodel.cn/api/coding/paas/v4"
ENV ZAI_API_KEY=""

CMD ["/bin/bash"]

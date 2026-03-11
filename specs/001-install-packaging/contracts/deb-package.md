# DEB 包接口契约

**功能**: Spec 001 - 便捷安装与分发系统
**契约类型**: DEB 包规范
**版本**: 1.0
**日期**: 2026-03-11

## 概述

本契约定义 AgentSwitch DEB 包的结构、元数据、文件清单和安装行为规范。

## 包元数据

### 基本信息

| 字段 | 值 | 说明 |
|------|-----|------|
| **Package Name** | `agentswitch` | 包名（小写，无特殊字符） |
| **Version** | `0.4.0-*` | 版本号（遵循语义化版本） |
| **Architecture** | `amd64` / `arm64` | 架构 |
| **Maintainer** | `Yu-Xiao-Sheng <your-email@example.com>` | 维护者 |
| **Section** | `utils` | 分类 |
| **Priority** | `optional` | 优先级 |
| **Description** | `A universal code agent tool configuration switcher` | 简短描述 |
| **Homepage** | `https://github.com/Yu-Xiao-Sheng/agentswitch` | 项目主页 |
| **License** | `MIT` | 许可证 |

### 完整描述示例

```
AgentSwitch is a universal configuration switcher for code agent tools.
It allows you to easily switch between different AI model providers
across tools like Claude Code, Codex, Gemini CLI, and more.

Features:
  - Unified model configuration management
  - Tool-specific adapter system
  - One-click configuration switching
  - Automatic backup and restore
  - Configuration presets and batch operations
  - Interactive wizard for first-time setup
```

### 依赖关系

| 依赖类型 | 值 | 说明 |
|----------|-----|------|
| **Depends** | `$auto`, `libc6` | 自动检测依赖 |
| **Recommends** | `bash-completion` | 推荐（可选） |
| **Suggests** | - | 无建议 |

### 系统要求

| 要求 | 值 |
|------|-----|
| **最低版本** | Ubuntu 20.04 LTS (Focal Fossa) / Debian 11 (Bullseye) |
| **最低 glibc** | 2.27 |
| **最低内核** | Linux 5.4 (Ubuntu 20.04) |

## 文件清单

### 二进制文件

| 源路径 | 目标路径 | 权限 | 描述 |
|--------|----------|------|------|
| `target/release/asw` | `/usr/bin/asw` | `0755` | 主程序二进制 |

### 文档文件

| 源路径 | 目标路径 | 权限 | 描述 |
|--------|----------|------|------|
| `packaging/man/asw.1` | `/usr/share/man/man1/asw.1` | `0644` | man 手册页 |
| `README.md` | `/usr/share/doc/agentswitch/README.md` | `0644` | README |
| `LICENSE` | `/usr/share/doc/agentswitch/LICENSE` | `0644` | 许可证 |
| `CHANGELOG.md` | `/usr/share/doc/agentswitch/changelog` | `0644` | 变更日志 (debian格式) |

### Shell 补全

| 源路径 | 目标路径 | 权限 | 描述 |
|--------|----------|------|------|
| `packaging/completions/asw.bash` | `/usr/share/bash-completion/completions/asw` | `0644` | Bash 补全 |
| `packaging/completions/asw.zsh` | `/usr/share/zsh/vendor-completions/_asw` | `0644` | Zsh 补全 |
| `packaging/completions/asw.fish` | `/usr/share/fish/vendor_completions.d/asw.fish` | `0644` | Fish 补全 |

### 配置文件

| 路径 | 描述 |
|------|------|
| `/var/lib/agentswitch/` | 状态目录（可选） |
| `~/.agentswitch/` | 用户配置目录（运行时创建） |
| `~/.config/agentswitch/` | XDG 配置目录（未来） |

**注意**: 配置文件 NOT 由 DEB 包安装，在首次运行时创建。

## Cargo.toml 配置

### cargo-deb 配置

```toml
[package.metadata.deb]
maintainer = "Yu-Xiao-Sheng <your-email@example.com>"
copyright = "2026, Yu-Xiao-Sheng <your-email@example.com>"
license-file = ["LICENSE", "0"]
depends = "$auto"
section = "utility"
priority = "optional"
extended-description = """\
AgentSwitch is a universal configuration switcher for code agent tools.
It allows you to easily switch between different AI model providers
across tools like Claude Code, Codex, Gemini CLI, and more.
"""
changelog-release = "release"

# 资源文件映射
[[package.metadata.deb.assets]]
source = "target/release/asw"
target = "usr/bin/asw"
mode = "755"

[[package.metadata.deb.assets]]
source = "packaging/man/asw.1"
target = "usr/share/man/man1/asw.1"
mode = "644"

[[package.metadata.deb.assets]]
source = "packaging/completions/asw.bash"
target = "usr/share/bash-completion/completions/asw"
mode = "644"

[[package.metadata.deb.assets]]
source = "README.md"
target = "usr/share/doc/agentswitch/README.md"
mode = "644"

[[package.metadata.deb.assets]]
source = "LICENSE"
target = "usr/share/doc/agentswitch/LICENSE"
mode = "644"

# systemd 服务（未来可选）
# [[package.metadata.deb.systemd-units]]
# unit = "packaging/agentswitch.service"
# enable = false
# start = false

# 维护者脚本
[package.metadata.deb.postinst]
script = "packaging/debian/postinst"

[package.metadata.deb.prerm]
script = "packaging/debian/prerm"

[package.metadata.deb.postrm]
script = "packaging/debian/postrm"
```

## 维护者脚本

### postinst (安装后)

```bash
#!/bin/bash
set -e

case "$1" in
    configure)
        # 重建 man 手册数据库
        if command -v mandb >/dev/null 2>&1; then
            mandb -q -p /usr/share/man
        fi

        # 显示欢迎消息
        echo ""
        echo "✓ AgentSwitch has been installed successfully!"
        echo ""
        echo "Quick start:"
        echo "  asw wizard init       # Start the initialization wizard"
        echo "  asw --help            # Show all commands"
        echo ""
        echo "Documentation:"
        echo "  man asw               # View manual page"
        echo "  https://github.com/Yu-Xiao-Sheng/agentswitch"
        echo ""
        ;;
esac

exit 0
```

### prerm (卸载前)

```bash
#!/bin/bash
set -e

case "$1" in
    remove|upgrade|deconfigure)
        # 停止任何运行中的服务（未来）
        # systemctl stop agentswitch || true
        ;;
esac

exit 0
```

### postrm (卸载后)

```bash
#!/bin/bash
set -e

case "$1" in
    purge)
        # 删除配置目录（如果用户确认）
        echo "Note: User configuration directory (~/.agentswitch/) was NOT removed."
        echo "To remove it manually:"
        echo "  rm -rf ~/.agentswitch/"
        ;;

    remove|upgrade|failed-upgrade|abort-install|abort-upgrade|disappear)
        # 不做任何操作
        ;;

esac

exit 0
```

## 构建流程

### 开发者手动构建

```bash
# 安装 cargo-deb
cargo install cargo-deb

# 构建 DEB 包
cargo deb

# 生成的包位置
# target/debian/agentswitch_0.4.0_amd64.deb
```

### CI/CD 自动构建

```yaml
# .github/workflows/release.yml
- name: Build DEB package
  run: |
    cargo install cargo-deb
    cargo deb --no-build --strip --separate-debug-symbols

- name: Upload DEB to Release
  uses: softprops/action-gh-release@v2
  with:
    files: target/debian/agentswitch_*_amd64.deb
```

## 验证要求

### 包完整性检查

```bash
# 检查包信息
dpkg -I agentswitch_0.4.0_amd64.deb

# 检查包内容（不安装）
dpkg -c agentswitch_0.4.0_amd64.deb

# 检查包依赖
dpkg --depends agentswitch_0.4.0_amd64.deb

# 检查包文件校验和
md5sum agentswitch_0.4.0_amd64.deb
sha256sum agentswitch_0.4.0_amd64.deb
```

### 安装测试

```bash
# 安装包
sudo dpkg -i agentswitch_0.4.0_amd64.deb

# 验证安装
which asw
asw --version
man asw

# 检查文件
dpkg -L agentswitch

# 测试补全
# 打开新 shell，输入 asw [Tab]
```

### 卸载测试

```bash
# 卸载（保留配置）
sudo dpkg -r agentswitch
ls ~/.agentswitch/  # 应该还存在

# 完全卸载（删除配置）
sudo dpkg -P agentswitch
```

## 版本控制

### 版本号格式

DEB 包版本号遵循 Debian 政策：
```
<upstream-version>-<debian-revision>
```

**示例**:
- `0.4.0-1` - upstream 版本 0.4.0，debian 修订版本 1
- `0.4.0-2` - 同一 upstream 版本，debian 修订版本 2（修复打包问题）

### 版本策略

- **upstream-version**: 与 Cargo.toml 中的 version 保持一致
- **debian-revision**: 从 1 开始递增
- 打包修复（不修改代码）时，仅增加 debian-revision
- 代码发布时，两者都增加

## 仓库配置（可选）

### APT 仓库结构

**未来**: 可以搭建自己的 APT 仓库：

```text
repo/
├── dists/
│   ├── stable/
│   │   ├── main/
│   │   │   ├── binary-amd64/
│   │   │   │   └── Packages
│   │   │   └── Release
│   └── unstable/
└── pool/
    └── main/
        └── a/
            └── agentswitch/
                ├── agentswitch_0.4.0_amd64.deb
                └── agentswitch_0.3.0_amd64.deb
```

### 用户添加仓库

```bash
# 添加仓库
echo "deb [signed-by=/usr/share/keyrings/agentswitch-archive-keyring.gpg] https://repo.example.com stable main" | sudo tee /etc/apt/sources.list.d/agentswitch.list

# 添加密钥
curl -fsSL https://repo.example.com/keyring.gpg | sudo gpg --dearmor -o /usr/share/keyrings/agentswitch-archive-keyring.gpg

# 安装
sudo apt update
sudo apt install agentswitch
```

## 测试矩阵

### 必须测试的发行版

| 发行版 | 版本 | 架构 | 优先级 |
|--------|------|------|--------|
| Ubuntu | 20.04 LTS | amd64 | P0 |
| Ubuntu | 22.04 LTS | amd64 | P0 |
| Ubuntu | 24.04 LTS | amd64 | P0 |
| Ubuntu | 24.04 LTS | arm64 | P1 |
| Debian | 11 (Bullseye) | amd64 | P1 |
| Debian | 12 (Bookworm) | amd64 | P1 |

### 测试场景

1. **全新安装**
2. **升级安装** (从旧版本升级)
3. **降级安装** (测试警告)
4. **卸载** (保留配置)
5. **完全卸载** (删除配置)
6. **依赖解析** (故意制造依赖冲突)
7. **文件覆盖** (修改后升级)

## 故障排除

### 常见问题

**问题 1**: 依赖关系无法满足
```
dpkg: dependency problems prevent configuration of agentswitch
```
**解决**:
```bash
sudo apt-get install -f
```

**问题 2**: 文件冲突
```
trying to overwrite '/usr/bin/asw', which is also in package other-tool
```
**解决**: 使用 `dpkg --force-overwrite` (不推荐)

**问题 3**: 架构不匹配
```
package architecture (amd64) does not match system (arm64)
```
**解决**: 下载正确架构的包

**问题 4**: 版本过旧
```
agentswitch: Depends: libc6 (>= 2.29) but 2.27 is installed
```
**解决**: 升级系统或使用静态链接

## 参考资料

- [Debian Policy Manual](https://www.debian.org/doc/debian-policy/)
- [cargo-deb Documentation](https://github.com/kornelski/cargo-deb)
- [Ubuntu Packaging Guide](https://packaging.ubuntu.com/html/)
- [Debian Developer's Reference](https://www.debian.org/doc/manuals/debmake-doc/)

#compdef asw
# Zsh completion script for AgentSwitch (asw)

_asw() {
    local -a commands subcommands

    commands=(
        'model:管理模型配置'
        'switch:切换工具到指定模型'
        'status:显示当前配置状态'
        'backup:管理配置备份'
        'preset:管理配置预设'
        'batch:批量操作多个工具'
        'wizard:启动初始化向导'
        'doctor:运行工具诊断'
        'completion:生成或安装Shell补全'
        'sync:Git配置同步'
        'help:显示帮助信息'
    )

    if (( CURRENT == 2 )); then
        _describe 'command' commands
    else
        case ${words[2]} in
            model)
                subcommands=('add:添加模型配置' 'list:列出所有模型' 'remove:删除模型配置' 'edit:编辑模型配置')
                if (( CURRENT == 3 )); then
                    _describe 'subcommand' subcommands
                fi
                ;;
            switch)
                if (( CURRENT == 3 )); then
                    _describe 'agent' '(claude-code"Claude Code" codex"Codex" gemini-cli"Gemini CLI")'
                elif (( CURRENT == 4 )); then
                    _describe 'model' '(glm-4"GLM-4" minimax"MiniMax")'
                fi
                ;;
            completion)
                subcommands=('generate:生成补全脚本' 'install:安装补全脚本')
                if (( CURRENT == 3 )); then
                    _describe 'subcommand' subcommands
                elif (( CURRENT == 4 )); then
                    _describe 'shell' '(bash"Bash" zsh"Zsh" fish"Fish")'
                fi
                ;;
        esac
    fi
}

_asw "$@"

# Bash completion script for AgentSwitch (asw)

_asw_completion() {
    local cur prev words cword
    _init_completion || return

    # 主命令补全
    if [[ ${cword} -eq 1 ]]; then
        if [[ ${cur} == * ]]; then
            COMPREPLY=($(compgen -W "model switch status backup preset batch wizard doctor completion sync help" -- "${cur}"))
        fi
        return 0
    fi

    # 子命令补全
    case ${words[1]} in
        model)
            _asw_model
            ;;
        switch)
            _asw_switch
            ;;
        completion)
            _asw_completion_subcommand
            ;;
    esac
}

_asw_model() {
    if [[ ${cword} -eq 2 ]]; then
        COMPREPLY=($(compgen -W "add list remove edit" -- "${cur}"))
    fi
}

_asw_switch() {
    if [[ ${cword} -eq 2 ]]; then
        COMPREPLY=($(compgen -W "claude-code codex gemini-cli qwen-cli grok-cli" -- "${cur}"))
    elif [[ ${cword} -eq 3 ]]; then
        COMPREPLY=($(compgen -W "glm-4 minimax" -- "${cur}"))
    fi
}

_asw_completion_subcommand() {
    if [[ ${cword} -eq 2 ]]; then
        COMPREPLY=($(compgen -W "generate install" -- "${cur}"))
    elif [[ ${cword} -eq 3 ]]; then
        COMPREPLY=($(compgen -W "bash zsh fish" -- "${cur}"))
    fi
}

complete -F _asw_completion asw

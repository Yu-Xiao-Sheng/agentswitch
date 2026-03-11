# Fish completion script for AgentSwitch (asw)

complete -c asw -f

complete -c asw -n '__fish_use_subcommand' -a model -d '管理模型配置'
complete -c asw -n '__fish_use_subcommand' -a switch -d '切换工具到指定模型'
complete -c asw -n '__fish_use_subcommand' -a status -d '显示当前配置状态'
complete -c asw -n '__fish_use_subcommand' -a backup -d '管理配置备份'
complete -c asw -n '__fish_use_subcommand' -a preset -d '管理配置预设'
complete -c asw -n '__fish_use_subcommand' -a batch -d '批量操作多个工具'
complete -c asw -n '__fish_use_subcommand' -a wizard -d '启动初始化向导'
complete -c asw -n '__fish_use_subcommand' -a doctor -d '运行工具诊断'
complete -c asw -n '__fish_use_subcommand' -a completion -d '生成或安装Shell补全'
complete -c asw -n '__fish_use_subcommand' -a sync -d 'Git配置同步'
complete -c asw -n '__fish_use_subcommand' -a help -d '显示帮助信息'

# model subcommands
complete -c asw -n '__fish_seen_subcommand_from model' -a add -d '添加模型配置'
complete -c asw -n '__fish_seen_subcommand_from model' -a list -d '列出所有模型'
complete -c asw -n '__fish_seen_subcommand_from model' -a remove -d '删除模型配置'
complete -c asw -n '__fish_seen_subcommand_from model' -a edit -d '编辑模型配置'

# completion subcommands
complete -c asw -n '__fish_seen_subcommand_from completion' -a generate -d '生成补全脚本'
complete -c asw -n '__fish_seen_subcommand_from completion' -a install -d '安装补全脚本'
complete -c asw -n '__fish_seen_subcommand_from completion' -n '__fish_seen_subcommand_from generate' -a bash -d 'Bash'
complete -c asw -n '__fish_seen_subcommand_from completion' -n '__fish_seen_subcommand_from generate' -a zsh -d 'Zsh'
complete -c asw -n '__fish_seen_subcommand_from completion' -n '__fish_seen_subcommand_from generate' -a fish -d 'Fish'

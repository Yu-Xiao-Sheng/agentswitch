use crate::crypto::master_key_exists;
use crate::output::{print_info, print_success, print_warning};
use colored::Colorize;
use std::path::Path;

/// 同步配置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncConfig {
    /// 远程仓库 URL
    pub remote_url: Option<String>,

    /// 远仓库名称
    pub remote_name: String,

    /// 分支名称
    pub branch: String,
}

fn config_dir() -> std::path::PathBuf {
    dirs::home_dir().unwrap().join(".agentswitch")
}

/// 检查仓库是否已初始化
fn repo_is_initialized() -> bool {
    let config_dir = config_dir();
    config_dir.join(".git").exists()
}

/// 打开仓库
fn open_repo() -> anyhow::Result<git2::Repository> {
    let config_dir = config_dir();
    git2::Repository::open(&config_dir).map_err(|e| {
        anyhow::anyhow!(
            "无法打开 Git 仓库: {}。请先运行 'asw sync init'",
            e.message()
        )
    })
}

/// 获取默认签名（用户名/邮箱），优先从 Git 配置读取
fn get_signature(repo: &git2::Repository) -> anyhow::Result<git2::Signature<'static>> {
    // 尝试从仓库配置读取
    if let Ok(config) = repo.config() {
        if let Ok(name) = config.get_string("user.name") {
            if let Ok(email) = config.get_string("user.email") {
                return git2::Signature::new(&name, &email, &git2::Time::new(0, 0))
                    .map_err(|e| anyhow::anyhow!("创建签名失败: {}", e.message()));
            }
        }
    }

    // 尝试从全局配置读取
    if let Ok(config) = git2::Config::open_default() {
        if let Ok(name) = config.get_string("user.name") {
            if let Ok(email) = config.get_string("user.email") {
                return git2::Signature::new(&name, &email, &git2::Time::new(0, 0))
                    .map_err(|e| anyhow::anyhow!("创建签名失败: {}", e.message()));
            }
        }
    }

    // 使用默认值
    git2::Signature::new("AgentSwitch", "agentswitch@local", &git2::Time::new(0, 0))
        .map_err(|e| anyhow::anyhow!("创建签名失败: {}", e.message()))
}

/// 检查工作区是否有未提交的更改
fn has_uncommitted_changes(repo: &git2::Repository) -> anyhow::Result<bool> {
    let mut status_opts = git2::StatusOptions::new();
    status_opts.include_untracked(true);
    status_opts.recurse_untracked_dirs(true);

    let statuses = repo.statuses(Some(&mut status_opts))?;
    Ok(statuses.iter().any(|s| {
        let st = s.status();
        // 排除 .gitignore 文件自身被忽略的情况
        st.intersects(
            git2::Status::INDEX_NEW
                | git2::Status::INDEX_MODIFIED
                | git2::Status::INDEX_DELETED
                | git2::Status::INDEX_RENAMED
                | git2::Status::WT_MODIFIED
                | git2::Status::WT_DELETED
                | git2::Status::WT_RENAMED
                | git2::Status::WT_NEW,
        )
    }))
}

/// 获取当前分支名称
fn current_branch_name(repo: &git2::Repository) -> Option<String> {
    repo.head()
        .ok()
        .and_then(|head| head.shorthand().map(|s| s.to_string()))
}

/// 获取远程仓库 URL
fn get_remote_url(repo: &git2::Repository, remote_name: &str) -> Option<String> {
    repo.find_remote(remote_name)
        .ok()
        .and_then(|r| r.url().map(|u| u.to_string()))
}

/// 获取远程列表
fn list_remotes(repo: &git2::Repository) -> Vec<String> {
    repo.remotes()
        .map(|names| {
            names
                .iter()
                .filter_map(|n| n.map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

/// 初始化 Git 同步
pub fn run_sync_init() -> anyhow::Result<()> {
    let config_dir = config_dir();

    println!("{}", "初始化 Git 同步".green().bold());
    println!("{}", "=".repeat(40).green());
    println!();

    // 1. 检查仓库是否已初始化
    if repo_is_initialized() {
        println!("{} Git 仓库已存在", "✓".green());
        print_info("如需重新初始化，请先删除 ~/.agentswitch/.git 目录");
        return Ok(());
    }

    // 2. 初始化 Git 仓库
    let repo = git2::Repository::init(&config_dir)?;
    println!("{} Git 仓库已初始化", "✓".green());

    // 3. 创建 .gitignore（如果不存在）
    let gitignore_path = config_dir.join(".gitignore");
    if !gitignore_path.exists() {
        let gitignore_content = "# AgentSwitch Git Sync\n*.key\nwizard_state.toml\n";
        std::fs::write(&gitignore_path, gitignore_content)?;
        println!("{} 已创建 .gitignore", "✓".green());
    } else {
        println!("{} .gitignore 已存在", "✓".green());
    }

    // 4. 检查密钥状态
    match master_key_exists() {
        Ok(true) => {
            println!("{} 加密密钥已配置", "✓".green());
        }
        Ok(false) => {
            print_warning("加密密钥未配置，建议运行 'asw crypto keygen' 生成密钥");
        }
        Err(e) => {
            print_warning(&format!("无法检查密钥状态: {}", e));
        }
    }

    // 5. 创建初始提交
    let signature = get_signature(&repo)?;
    let mut index = repo.index()?;

    // 添加 .gitignore 到索引
    index.add_path(Path::new(".gitignore"))?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit: AgentSwitch sync initialization",
        &tree,
        &[],
    )?;

    println!("{} 初始提交已创建", "✓".green());

    println!();
    print_success("Git 同步初始化完成");
    println!();
    print_info("下一步:");
    println!("  1. 添加远程仓库: asw sync remote add <url>");
    println!("  2. 推送配置: asw sync push");

    Ok(())
}

/// 推送配置到远程
pub fn run_sync_push() -> anyhow::Result<()> {
    println!("{}", "推送配置到远程".green().bold());
    println!("{}", "=".repeat(40).green());
    println!();

    // 1. 检查仓库是否已初始化
    let repo = open_repo()?;

    // 2. 检查是否有远程仓库
    let remotes = list_remotes(&repo);
    if remotes.is_empty() {
        anyhow::bail!("未配置远程仓库。请先运行 'asw sync remote add <url>'");
    }

    // 3. 确定远程名称
    let remote_name = if remotes.contains(&"origin".to_string()) {
        "origin"
    } else {
        &remotes[0]
    };
    let remote_url = get_remote_url(&repo, remote_name);
    println!(
        "远程仓库: {} ({})",
        remote_name,
        remote_url.as_deref().unwrap_or("未知")
    );

    // 4. 检查是否有未提交的更改
    if has_uncommitted_changes(&repo)? {
        println!("{} 检测到未提交的更改，正在提交...", "~".yellow());

        // 添加所有更改到暂存区
        let mut index = repo.index()?;
        index.add_all(["*"], git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let head = repo.head()?;
        let parent_commit = head.peel_to_commit()?;

        let signature = get_signature(&repo)?;
        let now = chrono::Local::now();
        let message = format!("Update configuration ({})", now.format("%Y-%m-%d %H:%M:%S"));

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &message,
            &tree,
            &[&parent_commit],
        )?;

        println!("{} 更改已提交: {}", "✓".green(), message);
    } else {
        println!("{} 没有未提交的更改", "✓".green());
    }

    // 5. 推送到远程
    let branch = current_branch_name(&repo).unwrap_or_else(|| "master".to_string());
    println!("正在推送到 {}/{}...", remote_name, branch);

    let mut remote = repo.find_remote(remote_name)?;
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);

    // 设置回调以处理认证等
    let mut push_options = git2::PushOptions::new();
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.push_update_reference(|refname, status| {
        if let Some(status) = status {
            return Err(git2::Error::from_str(&format!(
                "推送 {} 失败: {}",
                refname, status
            )));
        }
        Ok(())
    });
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        // 尝试使用默认凭据（SSH agent / 已配置的凭据）
        git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });
    push_options.remote_callbacks(callbacks);

    remote.push(&[&refspec], Some(&mut push_options))?;

    println!("{} 推送成功", "✓".green());
    print_success(&format!("配置已推送到 {}/{}", remote_name, branch));

    Ok(())
}

/// 从远程拉取配置
pub fn run_sync_pull() -> anyhow::Result<()> {
    println!("{}", "从远程拉取配置".green().bold());
    println!("{}", "=".repeat(40).green());
    println!();

    // 1. 检查仓库是否已初始化
    let mut repo = open_repo()?;

    // 2. 检查是否有远程仓库
    let remotes = list_remotes(&repo);
    if remotes.is_empty() {
        anyhow::bail!("未配置远程仓库。请先运行 'asw sync remote add <url>'");
    }

    // 3. 确定远程名称
    let remote_name = if remotes.contains(&"origin".to_string()) {
        "origin".to_string()
    } else {
        remotes[0].clone()
    };
    let remote_url = get_remote_url(&repo, &remote_name);
    println!(
        "远程仓库: {} ({})",
        remote_name,
        remote_url.as_deref().unwrap_or("未知")
    );

    // 4. 检查密钥状态
    match master_key_exists() {
        Ok(true) => println!("{} 加密密钥已配置", "✓".green()),
        Ok(false) => print_warning("加密密钥未配置，如需解密请先运行 'asw crypto keygen'"),
        Err(_) => {}
    }

    // 5. 获取当前分支
    let branch = current_branch_name(&repo).unwrap_or_else(|| "master".to_string());

    // 6. Fetch 远程更改
    println!("正在从 {} 拉取...", remote_name);

    {
        let refspec = format!(
            "refs/heads/{}:refs/remotes/{}/{}",
            branch, remote_name, branch
        );
        let mut fetch_options = git2::FetchOptions::new();
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });
        fetch_options.remote_callbacks(callbacks);

        let mut remote = repo.find_remote(&remote_name)?;
        remote.fetch(&[&refspec], Some(&mut fetch_options), None)?;
    }
    println!("{} 远程更改已获取", "✓".green());

    // 7. 获取 commit OIDs（不持有引用，避免借用冲突）
    let (fetch_commit_id, head_commit_id) = {
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_id = fetch_head.peel_to_commit()?.id();
        let head_id = repo.head()?.peel_to_commit()?.id();
        (fetch_id, head_id)
    };

    // 8. Stash 未提交的更改
    let stash_index = if has_uncommitted_changes(&repo)? {
        print_warning("工作区有未提交的更改，正在暂存...");
        let signature = get_signature(&repo)?;
        repo.stash_save2(
            &signature,
            Some("auto-stash before pull"),
            Some(git2::StashFlags::DEFAULT),
        )?;
        println!("{} 本地更改已暂存", "✓".green());
        Some(0usize)
    } else {
        None
    };

    // 9. 合并远程分支（所有 git2 对象在此作用域内使用后立即释放）
    let ancestor_id = repo.merge_base(head_commit_id, fetch_commit_id)?;

    if ancestor_id == head_commit_id {
        // Fast-forward: 本地落后于远程
        repo.reference("HEAD", fetch_commit_id, true, "Fast-forward pull")?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;
        println!("{} 快进合并完成 (fast-forward)", "✓".green());
    } else if ancestor_id == fetch_commit_id {
        // 本地已领先于远程
        println!("{} 本地已是最新的", "✓".green());
    } else {
        // 需要合并
        {
            let fetch_annotated = repo.find_annotated_commit(fetch_commit_id)?;
            let mut merge_opts = git2::MergeOptions::new();
            merge_opts.file_favor(git2::FileFavor::Normal);
            repo.merge(&[&fetch_annotated], Some(&mut merge_opts), None)?;
        }

        // 检查是否有冲突
        let mut index = repo.index()?;
        if index.has_conflicts() {
            repo.cleanup_state()?;
            anyhow::bail!("合并冲突！请手动解决冲突后运行 'asw sync push'");
        }

        // 自动提交合并结果
        {
            let head_commit = repo.find_commit(head_commit_id)?;
            let fetch_commit = repo.find_commit(fetch_commit_id)?;
            let signature = get_signature(&repo)?;
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;

            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &format!("Merge remote-tracking branch '{}/{}'", remote_name, branch),
                &tree,
                &[&head_commit, &fetch_commit],
            )?;
        }

        println!("{} 自动合并完成", "✓".green());
    }

    // 恢复 stash（如果有）
    if let Some(idx) = stash_index {
        let mut stash_apply_opts = git2::StashApplyOptions::new();
        repo.stash_apply(idx, Some(&mut stash_apply_opts))?;
        repo.stash_drop(idx)?;
        println!("{} 本地更改已恢复", "✓".green());
    }

    println!();
    print_success(&format!("配置已从 {}/{} 拉取完成", remote_name, branch));

    Ok(())
}

/// 显示同步状态
pub fn run_sync_status() -> anyhow::Result<()> {
    println!("{}", "同步状态".green().bold());
    println!("{}", "=".repeat(40).green());
    println!();

    // 1. 检查仓库状态
    if !repo_is_initialized() {
        println!("{:<20} {}", "Git 仓库:", "✗ 未初始化".red());
        println!();
        print_info("运行 'asw sync init' 初始化 Git 同步");
        return Ok(());
    }

    let repo = open_repo()?;

    println!("{:<20} {}", "Git 仓库:", "✓ 已初始化".green());

    // 2. 显示当前分支
    let branch = current_branch_name(&repo);
    match &branch {
        Some(b) => println!("{:<20} {}", "当前分支:", b.cyan()),
        None => println!("{:<20} {}", "当前分支:", "（空仓库）".yellow()),
    }

    // 3. 显示远程仓库
    let remotes = list_remotes(&repo);
    if remotes.is_empty() {
        println!("{:<20} {}", "远程仓库:", "✗ 未配置".red());
    } else {
        println!("{:<20}", "远程仓库:");
        for remote_name in &remotes {
            let url = get_remote_url(&repo, remote_name);
            println!(
                "  {} {}",
                format!("{}:", remote_name).cyan(),
                url.as_deref().unwrap_or("未知")
            );
        }
    }

    // 4. 显示工作区状态
    let has_changes = has_uncommitted_changes(&repo)?;
    if has_changes {
        println!("{:<20} {}", "工作区:", "⚠ 有未提交的更改".yellow());
    } else {
        println!("{:<20} {}", "工作区:", "✓ 干净".green());
    }

    // 5. 显示密钥状态
    match master_key_exists() {
        Ok(true) => println!("{:<20} {}", "加密密钥:", "✓ 已配置".green()),
        Ok(false) => println!("{:<20} {}", "加密密钥:", "✗ 未配置".yellow()),
        Err(e) => println!("{:<20} {}", "加密密钥:", format!("? 检查失败: {}", e).red()),
    }

    // 6. 显示最后提交信息
    if let Ok(head) = repo.head() {
        if let Ok(commit) = head.peel_to_commit() {
            let time = commit.time();
            let seconds = time.seconds();
            let offset = time.offset_minutes();
            let datetime = chrono::DateTime::from_timestamp(seconds, 0)
                .map(|dt: chrono::DateTime<chrono::Utc>| {
                    dt + chrono::Duration::minutes(offset as i64)
                })
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "未知".to_string());

            let message = commit.message().unwrap_or("(无消息)");
            let short_msg = if message.len() > 50 {
                format!("{}...", &message[..47])
            } else {
                message.to_string()
            };

            println!("{:<20} {}", "最后提交:", short_msg);
            println!("{:<20} {}", "提交时间:", datetime);
        }
    }

    println!();

    // 7. 显示提示信息
    if has_changes {
        print_info("运行 'asw sync push' 提交并推送更改");
    }

    Ok(())
}

/// 添加远程仓库
pub fn run_sync_remote_add(url: &str) -> anyhow::Result<()> {
    if !repo_is_initialized() {
        anyhow::bail!("Git 仓库未初始化。请先运行 'asw sync init'");
    }

    let repo = open_repo()?;

    // 检查 origin 是否已存在
    if repo.find_remote("origin").is_ok() {
        anyhow::bail!(
            "远程仓库 'origin' 已存在。使用 'asw sync remote set-url origin <new-url>' 修改"
        );
    }

    repo.remote("origin", url)?;
    print_success(&format!("远程仓库已添加: origin -> {}", url));
    Ok(())
}

/// 删除远程仓库
pub fn run_sync_remote_remove(name: &str) -> anyhow::Result<()> {
    if !repo_is_initialized() {
        anyhow::bail!("Git 仓库未初始化。请先运行 'asw sync init'");
    }

    let repo = open_repo()?;
    repo.remote_delete(name)?;
    print_success(&format!("远程仓库 '{}' 已删除", name));
    Ok(())
}

/// 列出远程仓库
pub fn run_sync_remote_list() -> anyhow::Result<()> {
    if !repo_is_initialized() {
        anyhow::bail!("Git 仓库未初始化。请先运行 'asw sync init'");
    }

    let repo = open_repo()?;
    let remotes = list_remotes(&repo);

    if remotes.is_empty() {
        println!("没有配置远程仓库");
        print_info("使用 'asw sync remote add <url>' 添加远程仓库");
        return Ok(());
    }

    println!("{}", "远程仓库列表:".green().bold());
    println!("{}", "-".repeat(60));
    println!("{:<15} URL", "名称");
    println!("{}", "-".repeat(60));

    for name in &remotes {
        let url = get_remote_url(&repo, name);
        println!("{:<15} {}", name, url.as_deref().unwrap_or("未知"));
    }

    Ok(())
}

/// 修改远程仓库 URL
pub fn run_sync_remote_set_url(name: &str, url: &str) -> anyhow::Result<()> {
    if !repo_is_initialized() {
        anyhow::bail!("Git 仓库未初始化。请先运行 'asw sync init'");
    }

    let repo = open_repo()?;

    // 检查远程仓库是否存在
    let existing_url = get_remote_url(&repo, name);
    if existing_url.is_none() {
        anyhow::bail!("远程仓库 '{}' 不存在", name);
    }

    repo.remote_set_url(name, url)?;
    print_success(&format!("远程仓库 '{}' 的 URL 已更新: {}", name, url));
    Ok(())
}

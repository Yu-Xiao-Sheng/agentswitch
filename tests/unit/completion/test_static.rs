use agentswitch::completion::static_completion;

#[test]
fn test_generate_completion_bash() {
    let result = static_completion::generate_completion("bash", "asw");
    assert!(result.is_ok());
    let script = result.unwrap();
    assert!(script.contains("bash"));
}

#[test]
fn test_generate_completion_zsh() {
    let result = static_completion::generate_completion("zsh", "asw");
    assert!(result.is_ok());
    let script = result.unwrap();
    assert!(script.contains("zsh"));
}

#[test]
fn test_generate_completion_fish() {
    let result = static_completion::generate_completion("fish", "asw");
    assert!(result.is_ok());
    let script = result.unwrap();
    assert!(script.contains("fish"));
}

#[test]
fn test_generate_completion_unsupported() {
    let result = static_completion::generate_completion("unsupported", "asw");
    assert!(result.is_err());
}

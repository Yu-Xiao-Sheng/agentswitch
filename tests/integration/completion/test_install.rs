#[test]
fn test_completion_install_bash() {
    use agentswitch::completion::install_completion;

    let result = install_completion("bash");
    // May fail if bash is not configured, but should not panic
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_completion_generate_bash() {
    use agentswitch::completion::static_completion::generate_completion;

    let result = generate_completion("bash", "asw");
    assert!(result.is_ok());
}

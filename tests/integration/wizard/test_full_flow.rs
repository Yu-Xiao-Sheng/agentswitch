// Integration tests for the interactive wizard
// Note: These tests are marked as ignored by default since they require interactive terminal

#[test]
#[ignore = "requires interactive terminal"]
fn test_wizard_basic_flow() {
    // This would test the full wizard flow
    // Marked as ignored since it requires user interaction
}

#[test]
#[ignore = "requires interactive terminal"]
fn test_wizard_resume_flow() {
    // Test resuming a saved wizard session
}

#[test]
#[ignore = "requires interactive terminal"]
fn test_wizard_reset_flow() {
    // Test resetting wizard progress
}

#[test]
fn test_wizard_state_persistence() {
    use agentswitch::wizard::{WizardState, WizardType};
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let state_file = temp_dir.path().join("wizard_state.toml");

    let state = WizardState::new(WizardType::InitialSetup);
    state.save(&state_file).unwrap();

    assert!(state_file.exists());

    let loaded = WizardState::load(&state_file).unwrap();
    assert!(loaded.is_some());
    let loaded = loaded.unwrap();
    assert_eq!(loaded.current_step, state.current_step);
}

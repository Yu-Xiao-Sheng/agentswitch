use agentswitch::wizard::{WizardState, WizardType};

#[test]
fn test_wizard_state_creation() {
    let state = WizardState::new(WizardType::InitialSetup);
    assert_eq!(state.current_step, 0);
    assert!(state.completed_steps.is_empty());
    assert!(state.data.is_empty());
}

#[test]
fn test_wizard_state_add_data() {
    let mut state = WizardState::new(WizardType::InitialSetup);
    state.data.insert("model_name".to_string(), "test-model".to_string());
    assert_eq!(state.data.get("model_name"), Some(&"test-model".to_string()));
}

#[test]
fn test_wizard_state_step_progression() {
    let mut state = WizardState::new(WizardType::InitialSetup);
    state.current_step = 1;
    state.completed_steps.push(0);
    assert_eq!(state.current_step, 1);
    assert_eq!(state.completed_steps.len(), 1);
}

#[test]
fn test_wizard_state_expiration() {
    let state = WizardState::new(WizardType::InitialSetup);
    // Fresh state should not be expired
    assert!(!state.is_expired());
}

#[test]
fn test_wizard_state_serialization() {
    let state = WizardState::new(WizardType::InitialSetup);
    let serialized = toml::to_string(&state).unwrap();
    assert!(serialized.contains("current_step"));
}

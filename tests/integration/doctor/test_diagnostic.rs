#[test]
fn test_doctor_basic_diagnostic() {
    use agentswitch::doctor::run_doctor;

    let result = run_doctor(false, false, false);
    assert!(result.is_ok());
}

#[test]
fn test_doctor_detect() {
    use agentswitch::doctor::run_detect;

    let result = run_detect();
    assert!(result.is_ok());
}

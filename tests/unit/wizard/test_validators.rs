use agentswitch::utils::validation;

#[test]
fn test_validate_model_name_valid() {
    assert!(validation::validate_model_name("valid-model").is_ok());
    assert!(validation::validate_model_name("my-model-123").is_ok());
}

#[test]
fn test_validate_model_name_invalid() {
    assert!(validation::validate_model_name("").is_err());
    assert!(validation::validate_model_name("invalid model").is_err());
    assert!(validation::validate_model_name("invalid@model").is_err());
}

#[test]
fn test_validate_url_valid() {
    assert!(validation::validate_url("https://example.com").is_ok());
    assert!(validation::validate_url("http://localhost:8080").is_ok());
}

#[test]
fn test_validate_url_invalid() {
    assert!(validation::validate_url("").is_err());
    assert!(validation::validate_url("not-a-url").is_err());
    assert!(validation::validate_url("ftp://example.com").is_err());
}

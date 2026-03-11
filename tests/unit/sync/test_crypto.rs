use agentswitch::sync::crypto::EncryptedValue;
use serde_json;

#[test]
fn test_encrypted_value_serialization() {
    let value = EncryptedValue {
        method: "aes-gcm".to_string(),
        data: "encrypted-data-base64".to_string(),
        nonce: Some("nonce-base64".to_string()),
    };

    let serialized = serde_json::to_string(&value).unwrap();
    assert!(serialized.contains("aes-gcm"));
}

#[test]
fn test_encrypted_value_deserialization() {
    let json = r#"{"method":"aes-gcm","data":"encrypted-data-base64","nonce":"nonce-base64"}"#;
    let value: EncryptedValue = serde_json::from_str(json).unwrap();
    assert_eq!(value.method, "aes-gcm");
}

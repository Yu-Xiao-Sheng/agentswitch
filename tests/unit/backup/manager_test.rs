use agentswitch::backup::BackupManager;

#[test]
fn test_manager_creation() {
    let manager = BackupManager::new();
    assert_eq!(manager.count(), 0);
}

#[test]
fn test_manager_default() {
    let manager = BackupManager::default();
    assert!(manager.list_all_backups().is_ok());
}

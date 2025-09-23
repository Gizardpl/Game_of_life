/// Moduł zarządzania konfiguracją gry
/// 
/// Zapewnia thread-safe dostęp do globalnej konfiguracji gry,
/// umożliwiając jej modyfikację w czasie działania aplikacji.

use std::sync::{Arc, RwLock};
use std::sync::OnceLock;
use super::rules::GameConfig;

/// Globalna instancja konfiguracji z możliwością modyfikacji
static GLOBAL_CONFIG: OnceLock<Arc<RwLock<GameConfig>>> = OnceLock::new();

/// Inicjalizuje globalną konfigurację
pub fn init_config() {
    GLOBAL_CONFIG.get_or_init(|| Arc::new(RwLock::new(GameConfig::default())));
}

/// Zwraca referencję do globalnej konfiguracji (tylko do odczytu)
pub fn get_config() -> GameConfig {
    let config_lock = GLOBAL_CONFIG.get_or_init(|| Arc::new(RwLock::new(GameConfig::default())));
    config_lock.read().unwrap().clone()
}

/// Modyfikuje globalną konfigurację za pomocą closure
pub fn modify_config<F>(modifier: F) 
where 
    F: FnOnce(&mut GameConfig)
{
    let config_lock = GLOBAL_CONFIG.get_or_init(|| Arc::new(RwLock::new(GameConfig::default())));
    let mut config = config_lock.write().unwrap();
    modifier(&mut config);
}

/// Resetuje konfigurację do wartości domyślnych
pub fn reset_config() {
    let config_lock = GLOBAL_CONFIG.get_or_init(|| Arc::new(RwLock::new(GameConfig::default())));
    let mut config = config_lock.write().unwrap();
    *config = GameConfig::default();
}

/// Ustawia nową konfigurację
pub fn set_config(new_config: GameConfig) {
    let config_lock = GLOBAL_CONFIG.get_or_init(|| Arc::new(RwLock::new(GameConfig::default())));
    let mut config = config_lock.write().unwrap();
    *config = new_config;
}

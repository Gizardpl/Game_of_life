/// Moduł konfiguracji gry Conway's Game of Life
/// 
/// Zawiera wszystkie parametry konfiguracyjne gry, które mogą być
/// modyfikowane przez użytkownika.

pub mod rules;
pub mod initial_state;
pub mod manager;

// Re-eksportujemy główne typy i funkcje
pub use rules::{BoardSizeMode};
pub use initial_state::{get_default_initial_state};
pub use manager::{get_config, init_config, modify_config};

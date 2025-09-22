/// Moduł konfiguracji gry Conway's Game of Life
/// 
/// Zawiera wszystkie parametry konfiguracyjne gry, które mogą być
/// modyfikowane przez użytkownika.

pub mod rules;

// Re-eksportujemy główne typy i funkcje
pub use rules::{get_config, init_config};

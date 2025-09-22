/// Moduł logic - zawiera całą logikę gry w życie
/// 
/// Ten moduł organizuje wszystkie komponenty logiczne gry:
/// - board: zarządzanie planszą i stanem komórek
/// - life_cycle: implementacja reguł gry Conway'a

pub mod board;
pub mod life_cycle;

// Re-eksportujemy najważniejsze typy z modułu board (gdy będą potrzebne)
// pub use board::{Board, CellState};
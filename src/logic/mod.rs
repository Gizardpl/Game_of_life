/// Moduł logic - zawiera całą logikę gry w życie
/// 
/// Ten moduł organizuje wszystkie komponenty logiczne gry:
/// - board: zarządzanie planszą i stanem komórek
/// - life_cycle: implementacja reguł gry Conway'a
/// - change_state: zarządzanie zmianą stanu komórek (klikanie i przeciąganie)

pub mod board;
pub mod life_cycle;
pub mod change_state;
pub mod prediction;

// Re-eksportujemy najważniejsze typy z modułu board (gdy będą potrzebne)
// pub use board::{Board, CellState};
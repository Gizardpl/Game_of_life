/// Moduł board - centralny punkt zarządzania planszą gry w życie
/// 
/// Ten moduł zawiera wszystkie komponenty niezbędne do zarządzania planszą:
/// - Strukturę danych Board przechowującą stan komórek
/// - Logikę mapowania współrzędnych 2D na indeksy 1D
/// - Funkcje dynamicznego rozszerzania planszy
/// - Narzędzia do optymalizacji rozmiaru planszy

// Eksportujemy główne komponenty modułu
pub mod structure;
pub mod expansion;

// Re-eksportujemy najważniejsze typy dla łatwiejszego dostępu
pub use structure::{Board, CellState};

// Opcjonalnie można dodać aliasy dla często używanych typów
pub type Position = (usize, usize);
pub type CellIterator<'a> = std::iter::Map<
    std::iter::Enumerate<std::slice::Iter<'a, CellState>>,
    fn((usize, &'a CellState)) -> (usize, usize, CellState)
>;
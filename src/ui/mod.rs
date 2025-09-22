/// Moduł interfejsu użytkownika dla gry w życie
/// 
/// Zawiera wszystkie komponenty odpowiedzialne za wyświetlanie
/// i interakcję z użytkownikiem.

pub mod render;
pub mod side_panel;

// Re-eksportujemy główne typy
pub use render::GameRenderer;
pub use side_panel::SidePanel;
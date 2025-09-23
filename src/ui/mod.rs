/// Moduł interfejsu użytkownika dla gry w życie
/// 
/// Zawiera wszystkie komponenty odpowiedzialne za wyświetlanie
/// i interakcję z użytkownikiem.

pub mod render;
pub mod side_panel;
pub mod preview_render;
pub mod settings;
pub mod styles;

// Re-eksportujemy główne typy
pub use render::{GameRenderer, MouseInteraction};
pub use side_panel::SidePanel;
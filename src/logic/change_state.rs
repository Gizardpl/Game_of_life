/// Moduł zarządzania zmianą stanu komórek
/// 
/// Zawiera logikę obsługi klikania i przeciągania po komórkach planszy.
/// Implementuje zaawansowaną logikę przeciągania z zachowaniem pierwszej akcji.

use crate::logic::board::{Board, CellState};

/// Typ akcji wykonanej na pierwszej komórce podczas przeciągania
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DragAction {
    /// Zmiana z martwej na żywą (tworzenie komórek)
    CreateCell,
    /// Zmiana z żywej na martwą (usuwanie komórek)
    KillCell,
}

/// Stan przeciągania myszy
#[derive(Debug, Clone)]
pub struct DragState {
    /// Czy przeciąganie jest aktywne
    pub is_dragging: bool,
    /// Typ akcji wykonanej na pierwszej komórce
    pub drag_action: Option<DragAction>,
    /// Ostatnia komórka, nad którą znajdował się kursor
    pub last_cell: Option<(usize, usize)>,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            drag_action: None,
            last_cell: None,
        }
    }
}

impl DragState {
    /// Tworzy nowy stan przeciągania
    pub fn new() -> Self {
        Self::default()
    }

    /// Rozpoczyna przeciąganie z określoną akcją
    pub fn start_drag(&mut self, action: DragAction, cell: (usize, usize)) {
        self.is_dragging = true;
        self.drag_action = Some(action);
        self.last_cell = Some(cell);
    }

    /// Kończy przeciąganie
    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        self.drag_action = None;
        self.last_cell = None;
    }

    /// Sprawdza czy przeciąganie jest aktywne
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    /// Zwraca typ akcji przeciągania
    pub fn drag_action(&self) -> Option<DragAction> {
        self.drag_action
    }

    /// Aktualizuje ostatnią komórkę
    pub fn update_last_cell(&mut self, cell: (usize, usize)) {
        self.last_cell = Some(cell);
    }

    /// Sprawdza czy to nowa komórka (różna od ostatniej)
    pub fn is_new_cell(&self, cell: (usize, usize)) -> bool {
        match self.last_cell {
            Some(last) => last != cell,
            None => true,
        }
    }
}

/// Manager zarządzania zmianą stanu komórek
pub struct CellStateManager {
    /// Stan przeciągania
    drag_state: DragState,
}

impl Default for CellStateManager {
    fn default() -> Self {
        Self {
            drag_state: DragState::new(),
        }
    }
}

impl CellStateManager {
    /// Tworzy nowy manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Obsługuje kliknięcie na komórkę (bez przeciągania)
    /// Zwraca true jeśli stan komórki został zmieniony
    pub fn handle_cell_click(&mut self, board: &mut Board, x: usize, y: usize) -> bool {
        // Proste przełączenie stanu komórki
        board.toggle_cell(x, y)
    }

    /// Rozpoczyna przeciąganie na danej komórce
    /// Zwraca true jeśli stan komórki został zmieniony
    pub fn start_drag(&mut self, board: &mut Board, x: usize, y: usize) -> bool {
        // Sprawdzamy aktualny stan komórki
        if let Some(current_state) = board.get_cell(x, y) {
            // Określamy typ akcji na podstawie aktualnego stanu
            let drag_action = match current_state {
                CellState::Dead => DragAction::CreateCell,
                CellState::Alive => DragAction::KillCell,
            };

            // Rozpoczynamy przeciąganie
            self.drag_state.start_drag(drag_action, (x, y));

            // Wykonujemy pierwszą akcję (przełączenie stanu)
            board.toggle_cell(x, y)
        } else {
            false
        }
    }

    /// Kontynuuje przeciąganie na danej komórce
    /// Zwraca true jeśli stan komórki został zmieniony
    pub fn continue_drag(&mut self, board: &mut Board, x: usize, y: usize) -> bool {
        // Sprawdzamy czy przeciąganie jest aktywne
        if !self.drag_state.is_dragging() {
            return false;
        }

        // Sprawdzamy czy to nowa komórka
        if !self.drag_state.is_new_cell((x, y)) {
            return false;
        }

        // Aktualizujemy ostatnią komórkę
        self.drag_state.update_last_cell((x, y));

        // Pobieramy typ akcji przeciągania
        let drag_action = match self.drag_state.drag_action() {
            Some(action) => action,
            None => return false,
        };

        // Pobieramy aktualny stan komórki
        let current_state = match board.get_cell(x, y) {
            Some(state) => state,
            None => return false,
        };

        // Wykonujemy akcję zgodnie z logiką przeciągania
        match drag_action {
            DragAction::CreateCell => {
                // Jeśli pierwsza akcja to tworzenie komórki, to:
                // - na martwych komórkach tworzymy żywe komórki
                // - na żywych komórkach nic nie robimy
                if current_state == CellState::Dead {
                    board.set_cell(x, y, CellState::Alive)
                } else {
                    false
                }
            }
            DragAction::KillCell => {
                // Jeśli pierwsza akcja to usuwanie komórki, to:
                // - na żywych komórkach tworzymy martwe komórki
                // - na martwych komórkach nic nie robimy
                if current_state == CellState::Alive {
                    board.set_cell(x, y, CellState::Dead)
                } else {
                    false
                }
            }
        }
    }

    /// Kończy przeciąganie
    pub fn end_drag(&mut self) {
        self.drag_state.end_drag();
    }

    /// Sprawdza czy przeciąganie jest aktywne
    pub fn is_dragging(&self) -> bool {
        self.drag_state.is_dragging()
    }

    /// Zwraca referencję do stanu przeciągania (do debugowania)
    pub fn drag_state(&self) -> &DragState {
        &self.drag_state
    }

    /// Obsługuje ruch myszy nad komórką podczas przeciągania
    /// Zwraca true jeśli stan komórki został zmieniony
    pub fn handle_mouse_over(&mut self, board: &mut Board, x: usize, y: usize) -> bool {
        if self.is_dragging() {
            self.continue_drag(board, x, y)
        } else {
            false
        }
    }

    /// Resetuje stan managera (przerywa przeciąganie)
    pub fn reset(&mut self) {
        self.drag_state.end_drag();
    }
}
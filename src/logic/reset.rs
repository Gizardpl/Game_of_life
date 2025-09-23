/// Moduł odpowiedzialny za logikę resetowania planszy
/// 
/// Implementuje dwuetapowy system resetowania:
/// 1. Pierwszy reset - powrót do stanu przed uruchomieniem symulacji
/// 2. Drugi reset - całkowicie pusta plansza

use super::board::Board;
use crate::config::{get_config, BoardSizeMode};

/// Manager odpowiedzialny za logikę resetowania planszy
pub struct ResetManager {
    /// Stan planszy przed pierwszym uruchomieniem (do resetowania)
    pre_start_board: Option<Board>,
    /// Czy ostatni reset był do stanu przed uruchomieniem (true) czy do pustej planszy (false)
    last_reset_was_to_pre_start: bool,
    /// Czy aplikacja była kiedykolwiek uruchomiona (do śledzenia czy pokazywać wzór)
    was_ever_started: bool,
}

impl Default for ResetManager {
    fn default() -> Self {
        Self {
            pre_start_board: None,
            last_reset_was_to_pre_start: false,
            was_ever_started: false,
        }
    }
}

impl ResetManager {
    /// Tworzy nowy manager resetowania
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Zapisuje aktualny stan planszy jako stan przed uruchomieniem
    pub fn save_pre_start_state(&mut self, board: &Board) {
        self.pre_start_board = Some(board.clone());
        self.last_reset_was_to_pre_start = false;
        self.was_ever_started = true;
    }
    
    /// Resetuje planszę zgodnie z dwuetapowym systemem
    /// 
    /// Zwraca nową planszę oraz informację czy aplikacja powinna być oznaczona jako "nie uruchomiona"
    pub fn reset_board(&mut self, _current_board: &Board, ever_started: bool) -> (Board, bool) {
        // ZAWSZE pobieramy najświeższe ustawienia z konfiguracji
        // aby uwzględnić zmiany dokonane przez użytkownika w GUI
        let config = get_config();
        
        // Pobieramy docelowy rozmiar planszy z aktualnych ustawień Board Settings
        let target_size = match config.board_size_mode {
            BoardSizeMode::Dynamic => config.initial_board_size,
            BoardSizeMode::Static => config.static_board_size,
        };
        
        if !ever_started {
            // Aplikacja nie była jeszcze uruchomiona - tworzymy pustą planszę
            // o rozmiarze zgodnym z aktualnymi ustawieniami Board Settings
            let new_board = Board::new(target_size, target_size);
            self.last_reset_was_to_pre_start = false;
            (new_board, false) // Nie zmieniamy stanu ever_started
        } else {
            // Aplikacja była uruchomiona - implementujemy dwuetapowy reset
            if let Some(ref pre_start_board) = self.pre_start_board {
                if !self.last_reset_was_to_pre_start {
                    // Pierwszy reset - wracamy do stanu przed uruchomieniem
                    // ale z rozmiarem dostosowanym do AKTUALNYCH ustawień Board Settings
                    let resized_board = self.resize_board_to_target(pre_start_board, target_size);
                    self.last_reset_was_to_pre_start = true;
                    (resized_board, false) // Nie resetujemy ever_started
                } else {
                    // Drugi reset - czyścimy planszę całkowicie (PUSTA PLANSZA)
                    // o rozmiarze zgodnym z AKTUALNYMI ustawieniami Board Settings
                    let new_board = Board::new(target_size, target_size);
                    self.last_reset_was_to_pre_start = false;
                    
                    // Resetujemy stan managera
                    self.pre_start_board = None;
                    self.was_ever_started = false;
                    
                    (new_board, true) // Resetujemy ever_started
                }
            } else {
                // Fallback - jeśli nie ma zapisanego stanu, tworzymy pustą planszę
                // o rozmiarze zgodnym z AKTUALNYMI ustawieniami Board Settings
                let new_board = Board::new(target_size, target_size);
                self.last_reset_was_to_pre_start = false;
                self.pre_start_board = None;
                self.was_ever_started = false;
                
                (new_board, true) // Resetujemy ever_started
            }
        }
    }
    
    /// Sprawdza czy ma zapisany stan przed uruchomieniem
    pub fn has_pre_start_state(&self) -> bool {
        self.pre_start_board.is_some()
    }
    
    /// Czyści zapisany stan przed uruchomieniem
    pub fn clear_pre_start_state(&mut self) {
        self.pre_start_board = None;
        self.last_reset_was_to_pre_start = false;
        // Nie resetujemy was_ever_started - to pozostaje jako historia
    }
    
    /// Zwraca informację o tym, jaki będzie następny reset
    pub fn get_next_reset_description(&self, ever_started: bool) -> &'static str {
        if !ever_started {
            "Reset to empty board"
        } else if let Some(_) = self.pre_start_board {
            if !self.last_reset_was_to_pre_start {
                "Reset to pre-start state"
            } else {
                "Reset to empty board"
            }
        } else {
            "Reset to empty board"
        }
    }
    
    /// Sprawdza czy następny reset będzie do pustej planszy
    pub fn next_reset_is_empty(&self, ever_started: bool) -> bool {
        if !ever_started {
            true // Aplikacja nie była uruchomiona - zawsze pusty
        } else if let Some(_) = self.pre_start_board {
            self.last_reset_was_to_pre_start // Jeśli ostatni był do pre-start, następny będzie pusty
        } else {
            true // Nie ma pre-start, więc będzie pusty
        }
    }
    
    /// Zmienia rozmiar planszy do docelowego rozmiaru, zachowując wzór
    /// 
    /// Funkcja przepisuje stan planszy do nowej planszy o docelowym rozmiarze.
    /// Jeśli nowy rozmiar jest większy, wzór jest wyśrodkowany.
    /// Jeśli nowy rozmiar jest mniejszy, wzór jest obcinany z zachowaniem środka.
    fn resize_board_to_target(&self, source_board: &Board, target_size: usize) -> Board {
        source_board.resize_to_square(target_size)
    }
}

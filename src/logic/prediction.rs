/// Moduł przewidywania następnego stanu gry
/// 
/// Zawiera funkcje do obliczania, które komórki będą żywe w następnej generacji
/// oraz identyfikuje komórki, które się narodzą (przejdą z martwych na żywe).

use super::board::{Board, CellState};
use crate::config::get_config;

/// Struktura przechowująca informacje o przewidywanym następnym stanie
#[derive(Debug, Clone)]
pub struct PredictionResult {
    /// Współrzędne komórek, które będą żywe w następnej generacji
    pub next_alive_cells: Vec<(usize, usize)>,
    /// Współrzędne komórek, które się narodzą (obecnie martwe, w następnej generacji żywe)
    pub birth_cells: Vec<(usize, usize)>,
    /// Współrzędne komórek, które umrą (obecnie żywe, w następnej generacji martwe)
    pub death_cells: Vec<(usize, usize)>,
}

impl PredictionResult {
    /// Tworzy nowy pusty wynik przewidywania
    pub fn new() -> Self {
        Self {
            next_alive_cells: Vec::new(),
            birth_cells: Vec::new(),
            death_cells: Vec::new(),
        }
    }
    
    /// Sprawdza czy komórka o podanych współrzędnych się narodzi
    pub fn will_be_born(&self, x: usize, y: usize) -> bool {
        self.birth_cells.contains(&(x, y))
    }
    
    /// Sprawdza czy komórka o podanych współrzędnych umrze
    pub fn will_die(&self, x: usize, y: usize) -> bool {
        self.death_cells.contains(&(x, y))
    }
    
    /// Sprawdza czy komórka o podanych współrzędnych będzie żywa w następnej generacji
    pub fn will_be_alive(&self, x: usize, y: usize) -> bool {
        self.next_alive_cells.contains(&(x, y))
    }
}

/// Przewiduje następny stan planszy i zwraca informacje o zmianach
pub fn predict_next_state(board: &Board) -> PredictionResult {
    let config = get_config();
    let mut result = PredictionResult::new();
    
    // Iterujemy przez wszystkie komórki planszy
    for y in 0..board.height() {
        for x in 0..board.width() {
            let current_state = board.get_cell(x, y).unwrap_or(CellState::Dead);
            let alive_neighbors = board.count_alive_neighbors(x, y);
            
            // Określamy nowy stan komórki na podstawie reguł
            let will_be_alive = match current_state {
                CellState::Alive => {
                    // Żywa komórka: sprawdzamy czy przeżyje
                    config.should_survive(alive_neighbors)
                },
                CellState::Dead => {
                    // Martwa komórka: sprawdzamy czy się narodzi
                    config.should_birth(alive_neighbors)
                }
            };
            
            // Zapisujemy wyniki
            if will_be_alive {
                result.next_alive_cells.push((x, y));
                
                // Jeśli komórka obecnie jest martwa, ale będzie żywa - to się narodzi
                if current_state == CellState::Dead {
                    result.birth_cells.push((x, y));
                }
            } else {
                // Jeśli komórka obecnie jest żywa, ale będzie martwa - to umrze
                if current_state == CellState::Alive {
                    result.death_cells.push((x, y));
                }
            }
        }
    }
    
    result
}

/// Przewiduje tylko komórki, które się narodzą w następnej generacji
/// (obecnie martwe, w następnej generacji żywe)
pub fn predict_birth_cells(board: &Board) -> Vec<(usize, usize)> {
    let prediction = predict_next_state(board);
    prediction.birth_cells
}

/// Przewiduje tylko komórki, które umrą w następnej generacji
/// (obecnie żywe, w następnej generacji martwe)
pub fn predict_death_cells(board: &Board) -> Vec<(usize, usize)> {
    let prediction = predict_next_state(board);
    prediction.death_cells
}

/// Sprawdza czy dana komórka się narodzi w następnej generacji
pub fn will_cell_be_born(board: &Board, x: usize, y: usize) -> bool {
    let current_state = board.get_cell(x, y).unwrap_or(CellState::Dead);
    
    // Komórka może się narodzić tylko jeśli obecnie jest martwa
    if current_state != CellState::Dead {
        return false;
    }
    
    let config = get_config();
    let alive_neighbors = board.count_alive_neighbors(x, y);
    
    config.should_birth(alive_neighbors)
}

/// Sprawdza czy dana komórka umrze w następnej generacji
pub fn will_cell_die(board: &Board, x: usize, y: usize) -> bool {
    let current_state = board.get_cell(x, y).unwrap_or(CellState::Dead);
    
    // Komórka może umrzeć tylko jeśli obecnie jest żywa
    if current_state != CellState::Alive {
        return false;
    }
    
    let config = get_config();
    let alive_neighbors = board.count_alive_neighbors(x, y);
    
    !config.should_survive(alive_neighbors)
}
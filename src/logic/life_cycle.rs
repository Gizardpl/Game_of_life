/// Implementacja reguł gry Conway's Game of Life
/// 
/// Ten moduł zawiera logikę określającą czy komórka przeżyje, umrze, czy się narodzi
/// w następnej generacji, bazując na konfiguracji zdefiniowanej w module config.

use super::board::{Board, CellState};
use crate::config::get_config;

impl Board {
    /// Oblicza następną generację planszy zgodnie z regułami gry
    /// 
    /// Dla każdej komórki sprawdza liczbę żywych sąsiadów i na podstawie
    /// konfiguracji określa jej stan w następnej generacji.
    pub fn next_generation(&self) -> Board {
        let config = get_config();
        let mut next_board = Board::new(self.width(), self.height());
        
        // Iterujemy przez wszystkie komórki planszy
        for y in 0..self.height() {
            for x in 0..self.width() {
                let current_state = self.get_cell(x, y).unwrap_or(CellState::Dead);
                let alive_neighbors = self.count_alive_neighbors(x, y);
                
                // Określamy nowy stan komórki na podstawie reguł
                let new_state = match current_state {
                    CellState::Alive => {
                        // Żywa komórka: sprawdzamy czy przeżyje
                        if config.should_survive(alive_neighbors) {
                            CellState::Alive
                        } else {
                            CellState::Dead
                        }
                    },
                    CellState::Dead => {
                        // Martwa komórka: sprawdzamy czy się narodzi
                        if config.should_birth(alive_neighbors) {
                            CellState::Alive
                        } else {
                            CellState::Dead
                        }
                    }
                };
                
                next_board.set_cell(x, y, new_state);
            }
        }
        
        next_board
    }
    
    /// Liczy liczbę żywych sąsiadów dla danej komórki
    /// 
    /// Sprawdza wszystkie 8 sąsiadujących komórek (w tym po przekątnej).
    /// Komórki poza granicami planszy są traktowane jako martwe.
    pub fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        
        // Sprawdzamy wszystkie 8 kierunków wokół komórki
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                // Pomijamy samą komórkę (środek)
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                // Obliczamy współrzędne sąsiada
                let neighbor_x = x as i32 + dx;
                let neighbor_y = y as i32 + dy;
                
                // Sprawdzamy czy sąsiad jest w granicach planszy
                if neighbor_x >= 0 && neighbor_y >= 0 {
                    let neighbor_x = neighbor_x as usize;
                    let neighbor_y = neighbor_y as usize;
                    
                    if neighbor_x < self.width() && neighbor_y < self.height() {
                        // Sąsiad jest w granicach planszy
                        if let Some(CellState::Alive) = self.get_cell(neighbor_x, neighbor_y) {
                            count += 1;
                        }
                    }
                    // Jeśli sąsiad jest poza granicami planszy, traktujemy go jako martwego
                    // (nie zwiększamy count)
                }
                // Jeśli współrzędne są ujemne, sąsiad jest poza planszą (martwy)
            }
        }
        
        count
    }
    
    /// Sprawdza czy plansza jest stabilna (nie zmieni się w następnej generacji)
    pub fn is_stable(&self) -> bool {
        let next = self.next_generation();
        
        // Porównujemy każdą komórkę
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get_cell(x, y) != next.get_cell(x, y) {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Sprawdza czy plansza jest pusta (wszystkie komórki martwe)
    pub fn is_empty(&self) -> bool {
        for (_, _, state) in self.iter_cells() {
            if state == CellState::Alive {
                return false;
            }
        }
        true
    }
}

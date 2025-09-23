/// Moduł randomizer - inteligentne generowanie losowej planszy
/// 
/// Zawiera funkcje do tworzenia losowej planszy z uwzględnieniem
/// prawdopodobieństwa bazowego i bonusów za sąsiadów.

use rand::Rng;
use super::board::{Board, CellState};
use crate::config::{get_config, RandomizerConfig};

/// Generuje losową planszę na podstawie aktualnego rozmiaru i konfiguracji randomizera
/// 
/// Algorytm działa w następujący sposób:
/// 1. Tworzy pustą planszę o rozmiarze aktualnej planszy
/// 2. Dla każdej komórki oblicza prawdopodobieństwo życia:
///    - Bazowe prawdopodobieństwo z konfiguracji
///    - Plus bonus za każdego żywego sąsiada (już wygenerowanego)
/// 3. Losuje czy komórka będzie żywa na podstawie obliczonego prawdopodobieństwa
pub fn generate_random_board(current_board: &Board) -> Board {
    let config = get_config();
    let randomizer_config = &config.randomizer_config;
    
    let width = current_board.width();
    let height = current_board.height();
    let mut new_board = Board::new(width, height);
    let mut rng = rand::thread_rng();
    
    // Iterujemy przez każdą komórkę planszy
    for y in 0..height {
        for x in 0..width {
            let probability = calculate_cell_probability(
                &new_board, 
                x, 
                y, 
                randomizer_config
            );
            
            // Losujemy czy komórka będzie żywa
            let random_value: f32 = rng.r#gen();
            if random_value < probability {
                new_board.set_cell(x, y, CellState::Alive);
            }
        }
    }
    
    new_board
}

/// Oblicza prawdopodobieństwo że komórka będzie żywa
fn calculate_cell_probability(
    board: &Board, 
    x: usize, 
    y: usize, 
    config: &RandomizerConfig
) -> f32 {
    let base_probability = config.base_probability;
    let neighbor_bonus = config.neighbor_bonus;
    
    // Zliczamy żywych sąsiadów (tylko tych już wygenerowanych)
    let alive_neighbors = count_alive_neighbors(board, x, y);
    
    // Obliczamy końcowe prawdopodobieństwo
    let total_probability = base_probability + (alive_neighbors as f32 * neighbor_bonus);
    
    // Ograniczamy do przedziału 0.0 - 1.0
    total_probability.min(1.0).max(0.0)
}

/// Zlicza liczbę żywych sąsiadów dla danej komórki
/// 
/// Sprawdza wszystkie 8 sąsiadujących komórek (jeśli istnieją)
/// i zlicza ile z nich jest żywych.
fn count_alive_neighbors(board: &Board, x: usize, y: usize) -> usize {
    let mut count = 0;
    
    // Sprawdzamy wszystkie 8 kierunków wokół komórki
    for dy in -1i32..=1i32 {
        for dx in -1i32..=1i32 {
            // Pomijamy samą komórkę (środek)
            if dx == 0 && dy == 0 {
                continue;
            }
            
            // Obliczamy współrzędne sąsiada
            let neighbor_x = x as i32 + dx;
            let neighbor_y = y as i32 + dy;
            
            // Sprawdzamy czy sąsiad mieści się w granicach planszy
            if neighbor_x >= 0 && neighbor_y >= 0 {
                let neighbor_x = neighbor_x as usize;
                let neighbor_y = neighbor_y as usize;
                
                if let Some(CellState::Alive) = board.get_cell(neighbor_x, neighbor_y) {
                    count += 1;
                }
            }
        }
    }
    
    count
}

/// Generuje całkowicie losową planszę bez uwzględnienia sąsiadów
/// 
/// Każda komórka ma takie samo prawdopodobieństwo życia (bazowe prawdopodobieństwo).
pub fn generate_simple_random_board(current_board: &Board) -> Board {
    let config = get_config();
    let base_probability = config.randomizer_config.base_probability;
    
    let width = current_board.width();
    let height = current_board.height();
    let mut new_board = Board::new(width, height);
    let mut rng = rand::thread_rng();
    
    // Iterujemy przez każdą komórkę planszy
    for y in 0..height {
        for x in 0..width {
            let random_value: f32 = rng.r#gen();
            if random_value < base_probability {
                new_board.set_cell(x, y, CellState::Alive);
            }
        }
    }
    
    new_board
}
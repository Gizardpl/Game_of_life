/// Definicja początkowego stanu planszy dla gry w życie
/// 
/// Ten moduł zawiera predefiniowane wzory, które mogą być użyte
/// jako punkt startowy dla symulacji.

use crate::logic::board::{Board, CellState};

/// Reprezentuje pozycję komórki na planszy
pub type Position = (usize, usize);

/// Predefiniowane wzory dla gry w życie
#[derive(Debug, Clone)]
pub enum Pattern {
    /// Pusty wzór (wszystkie komórki martwe)
    Empty,
    /// Pojedynczy blok 2x2
    Block,
    /// Oscylator "Blinker" (3 komórki w linii)
    Blinker,
    /// Glider - poruszający się wzór
    Glider,
    /// Niestandardowy wzór zdefiniowany przez listę pozycji
    Custom(Vec<Position>),
}

impl Pattern {
    /// Zwraca listę pozycji żywych komórek dla danego wzoru
    pub fn get_positions(&self) -> Vec<Position> {
        match self {
            Pattern::Empty => vec![],
            Pattern::Block => vec![
                (1, 1), (1, 2),
                (2, 1), (2, 2),
            ],
            Pattern::Blinker => vec![
                (1, 2), (2, 2), (3, 2),
            ],
            Pattern::Glider => vec![
                (1, 0),
                (2, 1),
                (0, 2), (1, 2), (2, 2),
            ],
            Pattern::Custom(positions) => positions.clone(),
        }
    }
    
    /// Zwraca minimalny rozmiar planszy potrzebny dla wzoru
    pub fn min_board_size(&self) -> (usize, usize) {
        let positions = self.get_positions();
        if positions.is_empty() {
            return (5, 5); // Minimalna plansza dla pustego wzoru
        }
        
        let max_x = positions.iter().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = positions.iter().map(|(_, y)| *y).max().unwrap_or(0);
        
        // Dodajemy margines 2 z każdej strony
        (max_x + 5, max_y + 5)
    }
}

/// Konfiguracja początkowego stanu gry
#[derive(Debug, Clone)]
pub struct InitialState {
    /// Wzór do umieszczenia na planszy
    pub pattern: Pattern,
    /// Pozycja wzoru na planszy (lewy górny róg)
    pub offset: Position,
}

impl Default for InitialState {
    fn default() -> Self {
        Self {
            pattern: Pattern::Empty,
            offset: (2, 2),
        }
    }
}

impl InitialState {
    /// Tworzy nową konfigurację początkowego stanu
    pub fn new(pattern: Pattern, offset: Position) -> Self {
        Self { pattern, offset }
    }
    
    /// Tworzy planszę z początkowym stanem
    pub fn create_board(&self) -> Board {
        let config = crate::config::get_config();
        let (min_width, min_height) = self.pattern.min_board_size();
        
        // Używamy większego z: rozmiaru z konfiguracji lub minimalnego rozmiaru dla wzoru
        let width = config.initial_board_size.max(min_width);
        let height = config.initial_board_size.max(min_height);
        
        let mut board = Board::new(width, height);
        self.apply_to_board(&mut board);
        board
    }
    
    /// Tworzy planszę z określonym rozmiarem
    pub fn create_board_with_size(&self, size: usize) -> Board {
        let (min_width, min_height) = self.pattern.min_board_size();
        
        // Używamy większego z: podanego rozmiaru lub minimalnego rozmiaru dla wzoru
        let width = size.max(min_width);
        let height = size.max(min_height);
        
        let mut board = Board::new(width, height);
        self.apply_to_board(&mut board);
        board
    }
    
    /// Aplikuje wzór do istniejącej planszy
    pub fn apply_to_board(&self, board: &mut Board) {
        // Najpierw czyścimy planszę
        board.clear();
        
        // Następnie ustawiamy żywe komórki zgodnie ze wzorem
        let positions = self.pattern.get_positions();
        for (x, y) in positions {
            let final_x = x + self.offset.0;
            let final_y = y + self.offset.1;
            
            // Sprawdzamy czy pozycja mieści się na planszy
            if board.is_valid_coords(final_x, final_y) {
                board.set_cell(final_x, final_y, CellState::Alive);
            }
        }
    }
}

/// Zwraca domyślną konfigurację początkowego stanu
pub fn get_default_initial_state() -> InitialState {
    InitialState::default()
}

/// Zwraca listę dostępnych wzorów
pub fn get_available_patterns() -> Vec<(&'static str, Pattern)> {
    vec![
        ("Empty", Pattern::Empty),
        ("Block", Pattern::Block),
        ("Blinker", Pattern::Blinker),
        ("Glider", Pattern::Glider),
    ]
}

use super::structure::{Board, CellState};
use crate::config::get_config;

/// Moduł odpowiedzialny za dynamiczne rozszerzanie planszy
/// 
/// Implementuje logikę powiększania planszy poprzez dodanie jednej warstwy
/// pustych (martwych) komórek dookoła istniejącej struktury.

impl Board {
    pub fn expand(&self) -> Board {
        // Obliczamy nowe wymiary - dodajemy po jednej komórce z każdej strony
        let new_width = self.width() + 2;
        let new_height = self.height() + 2;
        
        // Tworzymy nową planszę wypełnioną martwymi komórkami
        let mut expanded_board = Board::new(new_width, new_height);
        
        // Przepisujemy wszystkie komórki ze starej planszy do nowej
        // z offsetem (1, 1) aby wyśrodkować wzór
        for y in 0..self.height() {
            for x in 0..self.width() {
                // Pobieramy stan komórki ze starej planszy
                if let Some(cell_state) = self.get_cell(x, y) {
                    // Przepisujemy komórkę do nowej pozycji z offsetem
                    let new_x = x + 1; // Offset o 1 w poziomie
                    let new_y = y + 1; // Offset o 1 w pionie
                    
                    // Ustawiamy komórkę w nowej planszy
                    expanded_board.set_cell(new_x, new_y, cell_state);
                }
            }
        }
        
        expanded_board
    }

    /// Rozszerza planszę o określoną liczbę warstw
    /// 
    /// Wykonuje rozszerzenie planszy o podaną liczbę warstw komórek
    /// dookoła istniejącej struktury. Każda warstwa dodaje 2 do szerokości
    /// i 2 do wysokości planszy.
    pub fn expand_by_layers(&self, layers: usize) -> Option<Board> {
        if layers == 0 {
            return None;
        }
        
        // Obliczamy nowe wymiary
        let new_width = self.width() + (2 * layers);
        let new_height = self.height() + (2 * layers);
        
        // Tworzymy nową planszę
        let mut expanded_board = Board::new(new_width, new_height);
        
        // Obliczamy offset do wyśrodkowania wzoru
        let offset_x = layers;
        let offset_y = layers;
        
        // Przepisujemy wszystkie komórki ze starej planszy
        for y in 0..self.height() {
            for x in 0..self.width() {
                if let Some(cell_state) = self.get_cell(x, y) {
                    let new_x = x + offset_x;
                    let new_y = y + offset_y;
                    expanded_board.set_cell(new_x, new_y, cell_state);
                }
            }
        }
        
        Some(expanded_board)
    }

    /// Automatycznie rozszerza planszę jeśli żywe komórki są zbyt blisko krawędzi
    /// 
    /// Sprawdza czy istnieją żywe komórki w określonej odległości od krawędzi planszy.
    /// Jeśli tak, automatycznie rozszerza planszę aby zapewnić odpowiedni margines.
    /// Respektuje maksymalny rozmiar planszy zdefiniowany w konfiguracji.
    pub fn auto_expand_if_needed(&self, margin: usize) -> Option<Board> {
        let config = get_config();
        let mut needs_expansion = false;
        
        // Sprawdzamy czy plansza może być rozszerzona
        if !config.can_expand(self.width(), self.height(), config.expansion_layers) {
            // Plansza osiągnęła maksymalny rozmiar - nie rozszerzamy
            return None;
        }
        
        // Sprawdzamy czy istnieją żywe komórki zbyt blisko krawędzi
        for (x, y, state) in self.iter_cells() {
            if state == CellState::Alive {
                // Sprawdzamy odległość od każdej krawędzi
                if x < margin ||                           // Lewa krawędź
                   x >= self.width().saturating_sub(margin) || // Prawa krawędź
                   y < margin ||                           // Górna krawędź
                   y >= self.height().saturating_sub(margin)   // Dolna krawędź
                {
                    needs_expansion = true;
                    break;
                }
            }
        }
        
        // Jeśli potrzebne jest rozszerzenie, wykonujemy je z ograniczeniami
        if needs_expansion {
            // Sprawdzamy ile warstw możemy faktycznie dodać
            let layers = config.expansion_layers;
            let max_width = config.get_max_dimension(self.width(), layers);
            let max_height = config.get_max_dimension(self.height(), layers);
            
            // Obliczamy rzeczywiste wymiary po rozszerzeniu
            let target_width = (self.width() + 2 * layers).min(max_width);
            let target_height = (self.height() + 2 * layers).min(max_height);
            
            // Jeśli wymiary się nie zmieniły, nie ma sensu rozszerzać
            if target_width == self.width() && target_height == self.height() {
                return None;
            }
            
            // Tworzymy rozszerzoną planszę z ograniczeniami
            self.expand_with_limits(target_width, target_height)
        } else {
            None
        }
    }
    
    /// Rozszerza planszę do określonych wymiarów (z ograniczeniami)
    /// 
    /// Pomocnicza funkcja dla auto_expand_if_needed, która tworzy planszę
    /// o dokładnie określonych wymiarach, nie większych niż maksymalne.
    fn expand_with_limits(&self, target_width: usize, target_height: usize) -> Option<Board> {
        if target_width <= self.width() && target_height <= self.height() {
            return None;
        }
        
        // Tworzymy nową planszę o docelowych wymiarach
        let mut expanded_board = Board::new(target_width, target_height);
        
        // Obliczamy offset do wyśrodkowania wzoru
        let offset_x = (target_width.saturating_sub(self.width())) / 2;
        let offset_y = (target_height.saturating_sub(self.height())) / 2;
        
        // Przepisujemy wszystkie komórki ze starej planszy
        for y in 0..self.height() {
            for x in 0..self.width() {
                if let Some(cell_state) = self.get_cell(x, y) {
                    let new_x = x + offset_x;
                    let new_y = y + offset_y;
                    
                    // Sprawdzamy czy nowa pozycja mieści się w docelowej planszy
                    if new_x < target_width && new_y < target_height {
                        expanded_board.set_cell(new_x, new_y, cell_state);
                    }
                }
            }
        }
        
        Some(expanded_board)
    }

    /// Optymalizuje rozmiar planszy poprzez usunięcie pustych krawędzi
    /// 
    /// Analizuje planszę i znajduje najmniejszy prostokąt zawierający
    /// wszystkie żywe komórki. Tworzy nową planszę o optymalnych wymiarach.
    pub fn optimize_size(&self, margin: usize) -> Option<Board> {
        // Znajdujemy granice obszaru z żywymi komórkami
        let alive_cells: Vec<(usize, usize)> = self.iter_alive_cells().collect();
        
        if alive_cells.is_empty() {
            // Jeśli nie ma żywych komórek, zwracamy małą planszę
            return Some(Board::new(2 * margin + 1, 2 * margin + 1));
        }
        
        // Znajdujemy skrajne współrzędne
        let min_x = alive_cells.iter().map(|(x, _)| *x).min().unwrap();
        let max_x = alive_cells.iter().map(|(x, _)| *x).max().unwrap();
        let min_y = alive_cells.iter().map(|(_, y)| *y).min().unwrap();
        let max_y = alive_cells.iter().map(|(_, y)| *y).max().unwrap();
        
        // Obliczamy nowe wymiary z marginesem
        let start_x = min_x.saturating_sub(margin);
        let start_y = min_y.saturating_sub(margin);
        let end_x = (max_x + margin + 1).min(self.width());
        let end_y = (max_y + margin + 1).min(self.height());
        
        let new_width = end_x - start_x;
        let new_height = end_y - start_y;
        
        // Tworzymy nową, zoptymalizowaną planszę
        let mut optimized_board = Board::new(new_width, new_height);
        
        // Przepisujemy fragment starej planszy
        for y in start_y..end_y {
            for x in start_x..end_x {
                if let Some(cell_state) = self.get_cell(x, y) {
                    let new_x = x - start_x;
                    let new_y = y - start_y;
                    optimized_board.set_cell(new_x, new_y, cell_state);
                }
            }
        }
        
        Some(optimized_board)
    }
}
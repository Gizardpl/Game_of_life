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
    /// UWAGA: Funkcja działa tylko w trybie Dynamic - w trybie Static zawsze zwraca None.
    pub fn auto_expand_if_needed(&self, margin: usize) -> Option<Board> {
        let config = get_config();
        
        // W trybie Static NIGDY nie rozszerzamy planszy
        if !config.can_expand_in_current_mode() {
            return None;
        }
        
        let mut needs_expansion = false;
        
        // Sprawdzamy czy plansza może być rozszerzona (nie osiągnęła maksymalnego rozmiaru)
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

    /// Optymalizuje rozmiar planszy poprzez iteracyjne usuwanie pustych pierścieni krawędzi
    /// 
    /// Algorytm działa następująco:
    /// 1. Sprawdza czy można usunąć cały zewnętrzny pierścień (zachowując margines od żywych komórek)
    /// 2. Jeśli tak, usuwa jeden kompletny pierścień i sprawdza ponownie
    /// 3. Powtarza proces aż nie można już więcej usunąć
    /// 4. Zachowuje dokładnie `margin` pustych komórek od najbliższych żywych komórek
    /// 5. ZAWSZE zwraca kwadratową planszę
    pub fn optimize_size(&self, margin: usize) -> Option<Board> {
        // Plansza musi być kwadratem - bierzemy mniejszy wymiar jako bazę
        let current_size = self.width().min(self.height());
        
        if current_size <= 2 * margin + 1 {
            // Plansza jest już minimalna - nie można jej zmniejszyć
            return None;
        }
        
        // Rozpoczynamy z kwadratową wersją aktualnej planszy
        let mut current_board = self.resize_to_square(current_size);
        let mut was_optimized = false;
        
        loop {
            // Sprawdzamy czy można usunąć cały zewnętrzny pierścień
            if current_board.can_remove_outer_ring(margin) {
                // Usuwamy jeden kompletny pierścień z wszystkich stron
                current_board = current_board.remove_outer_ring();
                was_optimized = true;
                
                // Sprawdzamy czy plansza nie stała się zbyt mała
                if current_board.width() <= 2 * margin + 1 {
                    break;
                }
            } else {
                // Nie można usunąć więcej pierścieni
                break;
            }
        }
        
        // Zwracamy zoptymalizowaną planszę tylko jeśli rzeczywiście ją zmniejszyliśmy
        if was_optimized {
            Some(current_board)
        } else {
            None
        }
    }
    
    /// Sprawdza czy można usunąć cały zewnętrzny pierścień zachowując margines
    /// 
    /// Zewnętrzny pierścień to wszystkie komórki na krawędzi planszy:
    /// - Cały pierwszy wiersz (y = 0)
    /// - Cały ostatni wiersz (y = height - 1) 
    /// - Cała pierwsza kolumna (x = 0, bez narożników już policzone w wierszach)
    /// - Cała ostatnia kolumna (x = width - 1, bez narożników już policzone w wierszach)
    fn can_remove_outer_ring(&self, margin: usize) -> bool {
        let size = self.width(); // Plansza jest kwadratem
        
        if size <= 2 * margin + 1 {
            return false;
        }
        
        // Sprawdzamy czy w zewnętrznym pierścieniu i następnych `margin` warstwach są żywe komórki
        // Jeśli znajdziemy żywą komórkę w obszarze który zostałby usunięty lub zbyt blisko krawędzi, 
        // nie możemy usunąć pierścienia
        
        for layer in 0..=margin {
            if layer >= size / 2 {
                break; // Nie ma więcej warstw do sprawdzenia
            }
            
            // Sprawdzamy warstwę `layer` od krawędzi
            // Górny i dolny wiersz warstwy
            for x in layer..(size - layer) {
                if let Some(CellState::Alive) = self.get_cell(x, layer) {
                    return false;
                }
                if let Some(CellState::Alive) = self.get_cell(x, size - 1 - layer) {
                    return false;
                }
            }
            
            // Lewa i prawa kolumna warstwy (bez narożników już sprawdzonych)
            for y in (layer + 1)..(size - 1 - layer) {
                if let Some(CellState::Alive) = self.get_cell(layer, y) {
                    return false;
                }
                if let Some(CellState::Alive) = self.get_cell(size - 1 - layer, y) {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Usuwa cały zewnętrzny pierścień z planszy
    /// 
    /// Tworzy nową planszę o rozmiarze (size - 2) x (size - 2) i kopiuje
    /// wszystkie komórki z wewnętrznego obszaru, pomijając zewnętrzny pierścień.
    fn remove_outer_ring(&self) -> Board {
        let old_size = self.width(); // Plansza jest kwadratem
        
        if old_size <= 2 {
            // Nie można usunąć pierścienia z planszy 2x2 lub mniejszej
            return self.clone();
        }
        
        let new_size = old_size - 2;
        let mut new_board = Board::new(new_size, new_size);
        
        // Kopiujemy wewnętrzny obszar (pomijamy zewnętrzny pierścień)
        for y in 1..(old_size - 1) {
            for x in 1..(old_size - 1) {
                if let Some(cell_state) = self.get_cell(x, y) {
                    // Przesuwamy współrzędne o -1 w obu osiach
                    new_board.set_cell(x - 1, y - 1, cell_state);
                }
            }
        }
        
        new_board
    }

    /// Zmienia rozmiar planszy do określonych wymiarów
    /// 
    /// Jeśli nowy rozmiar jest większy, dodaje puste komórki dookoła.
    /// Jeśli nowy rozmiar jest mniejszy, obcina komórki z krawędzi.
    /// Komórki są wyśrodkowane w nowej planszy.
    pub fn resize_to(&self, new_width: usize, new_height: usize) -> Board {
        let mut new_board = Board::new(new_width, new_height);
        
        // Obliczamy offset do wyśrodkowania
        let offset_x = if new_width > self.width() {
            (new_width - self.width()) / 2
        } else {
            0
        };
        let offset_y = if new_height > self.height() {
            (new_height - self.height()) / 2
        } else {
            0
        };
        
        // Obliczamy zakres komórek do skopiowania
        let start_x = if new_width < self.width() {
            (self.width() - new_width) / 2
        } else {
            0
        };
        let start_y = if new_height < self.height() {
            (self.height() - new_height) / 2
        } else {
            0
        };
        
        let end_x = (start_x + new_width).min(self.width());
        let end_y = (start_y + new_height).min(self.height());
        
        // Kopiujemy komórki
        for y in start_y..end_y {
            for x in start_x..end_x {
                if let Some(cell_state) = self.get_cell(x, y) {
                    let new_x = (x - start_x) + offset_x;
                    let new_y = (y - start_y) + offset_y;
                    
                    if new_x < new_width && new_y < new_height {
                        new_board.set_cell(new_x, new_y, cell_state);
                    }
                }
            }
        }
        
        new_board
    }

    /// Zmienia rozmiar planszy do kwadratu o podanym rozmiarze
    pub fn resize_to_square(&self, size: usize) -> Board {
        self.resize_to(size, size)
    }
}
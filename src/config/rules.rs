/// Centralny plik konfiguracji dla gry Conway's Game of Life
/// 
/// Zawiera wszystkie parametry gry, które mogą być modyfikowane
/// przez użytkownika poprzez GUI.

use std::ops::RangeInclusive;
use std::sync::OnceLock;

/// Struktura zawierająca wszystkie parametry konfiguracyjne gry
#[derive(Debug, Clone)]
pub struct GameConfig {
    /// Przedział liczby sąsiadów potrzebnych do narodzin nowej komórki
    /// Domyślnie: 3 (standardowa reguła Conway'a)
    pub birth_neighbors: RangeInclusive<usize>,
    
    /// Przedział liczby sąsiadów potrzebnych do przeżycia komórki
    /// Domyślnie: 2-3 (standardowa reguła Conway'a)
    pub survival_neighbors: RangeInclusive<usize>,
    
    /// Maksymalny rozmiar planszy (szerokość i wysokość)
    /// Po osiągnięciu tego rozmiaru plansza nie będzie się dalej rozszerzać
    pub max_board_size: usize,
    
    /// Początkowy rozmiar planszy przy starcie gry
    pub initial_board_size: usize,
    
    /// Margines od krawędzi planszy, przy którym następuje automatyczne rozszerzenie
    /// (jeśli nie osiągnięto maksymalnego rozmiaru)
    pub expansion_margin: usize,
    
    /// Liczba warstw dodawanych podczas jednego rozszerzenia planszy
    pub expansion_layers: usize,
    
    /// Margines pozostawiany przy optymalizacji rozmiaru planszy
    pub optimization_margin: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            // Standardowe reguły Conway'a: B3/S23
            birth_neighbors: 3..=3,           // Narodziny przy dokładnie 3 sąsiadach
            survival_neighbors: 2..=3,        // Przeżycie przy 2 lub 3 sąsiadach
            
            // Ograniczenia rozmiaru planszy
            max_board_size: 100,              // Maksymalny rozmiar 100x100
            initial_board_size: 10,          // Początkowy rozmiar planszy
            
            // Parametry rozszerzania
            expansion_margin: 2,              // Rozszerzaj gdy żywe komórki są 2 pola od krawędzi
            expansion_layers: 1,              // Dodawaj 1 warstwę na raz
            optimization_margin: 3,           // Pozostaw 3 pola marginesu przy optymalizacji
        }
    }
}

impl GameConfig {
    /// Tworzy nową konfigurację z domyślnymi wartościami
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Sprawdza czy dana liczba sąsiadów pozwala na narodziny komórki
    pub fn should_birth(&self, neighbors: usize) -> bool {
        self.birth_neighbors.contains(&neighbors)
    }
    
    /// Sprawdza czy dana liczba sąsiadów pozwala na przeżycie komórki
    pub fn should_survive(&self, neighbors: usize) -> bool {
        self.survival_neighbors.contains(&neighbors)
    }
    
    /// Sprawdza czy plansza może być rozszerzona (nie przekroczy maksymalnego rozmiaru)
    pub fn can_expand(&self, current_width: usize, current_height: usize, layers: usize) -> bool {
        let new_width = current_width + (2 * layers);
        let new_height = current_height + (2 * layers);
        
        new_width <= self.max_board_size && new_height <= self.max_board_size
    }
    
    /// Zwraca maksymalny dozwolony rozmiar dla danego wymiaru
    pub fn get_max_dimension(&self, current_size: usize, layers: usize) -> usize {
        let proposed_size = current_size + (2 * layers);
        proposed_size.min(self.max_board_size)
    }
}

/// Globalna instancja konfiguracji
/// W przyszłości może być zastąpiona przez system ładowania z pliku lub GUI
static GLOBAL_CONFIG: OnceLock<GameConfig> = OnceLock::new();

/// Inicjalizuje globalną konfigurację
pub fn init_config() {
    GLOBAL_CONFIG.get_or_init(|| GameConfig::default());
}

/// Zwraca referencję do globalnej konfiguracji
pub fn get_config() -> &'static GameConfig {
    GLOBAL_CONFIG.get_or_init(|| GameConfig::default())
}

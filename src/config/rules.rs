/// Centralny plik konfiguracji dla gry Conway's Game of Life
/// 
/// Zawiera wszystkie parametry gry, które mogą być modyfikowane
/// przez użytkownika poprzez GUI.

use std::ops::RangeInclusive;

/// Tryb zarządzania rozmiarem planszy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardSizeMode {
    /// Dynamiczny rozmiar - plansza rozszerza się automatycznie
    Dynamic,
    /// Statyczny rozmiar - plansza ma stały rozmiar
    Static,
}

impl Default for BoardSizeMode {
    fn default() -> Self {
        BoardSizeMode::Dynamic
    }
}

/// Struktura zawierająca wszystkie parametry konfiguracyjne gry
#[derive(Debug, Clone)]
pub struct GameConfig {
    /// Przedział liczby sąsiadów potrzebnych do narodzin nowej komórki
    /// Domyślnie: 3 (standardowa reguła Conway'a)
    pub birth_neighbors: RangeInclusive<usize>,
    
    /// Przedział liczby sąsiadów potrzebnych do przeżycia komórki
    /// Domyślnie: 2-3 (standardowa reguła Conway'a)
    pub survival_neighbors: RangeInclusive<usize>,
    
    /// Tryb zarządzania rozmiarem planszy
    pub board_size_mode: BoardSizeMode,
    
    /// Maksymalny rozmiar planszy (szerokość i wysokość) - używany w trybie Dynamic
    /// Po osiągnięciu tego rozmiaru plansza nie będzie się dalej rozszerzać
    pub max_board_size: usize,
    
    /// Początkowy rozmiar planszy przy starcie gry - używany w trybie Dynamic
    pub initial_board_size: usize,
    
    /// Stały rozmiar planszy - używany w trybie Static
    pub static_board_size: usize,
    
    /// Margines od krawędzi planszy, przy którym następuje automatyczne rozszerzenie
    /// (jeśli nie osiągnięto maksymalnego rozmiaru)
    pub expansion_margin: usize,
    
    /// Liczba warstw dodawanych podczas jednego rozszerzenia planszy
    pub expansion_layers: usize,
    
    /// Margines pozostawiany przy optymalizacji rozmiaru planszy
    pub optimization_margin: usize,
    
    /// Parametry interfejsu użytkownika
    pub ui_config: UIConfig,
    
    /// Konfiguracja randomizera planszy
    pub randomizer_config: RandomizerConfig,
}

/// Konfiguracja randomizera planszy
#[derive(Debug, Clone)]
pub struct RandomizerConfig {
    /// Bazowe prawdopodobieństwo że komórka będzie żywa (0.0 - 1.0)
    pub base_probability: f32,
    
    /// Bonus prawdopodobieństwa za każdego żywego sąsiada (0.0 - 1.0)
    pub neighbor_bonus: f32,
}

impl Default for RandomizerConfig {
    fn default() -> Self {
        Self {
            base_probability: 0.20,    // 20% bazowe prawdopodobieństwo
            neighbor_bonus: 0.10,      // +10% za każdego sąsiada
        }
    }
}

/// Konfiguracja parametrów interfejsu użytkownika
#[derive(Debug, Clone)]
pub struct UIConfig {
    /// Domyślna prędkość symulacji (generacje na sekundę)
    pub default_simulation_speed: f32,
    
    /// Minimalna prędkość symulacji
    pub min_simulation_speed: f32,
    
    /// Maksymalna prędkość symulacji
    pub max_simulation_speed: f32,
    
    /// Krok zmiany prędkości na suwaку
    pub simulation_speed_step: f32,
    
    /// Domyślny rozmiar przycisków (szerokość, wysokość)
    pub default_button_size: (f32, f32),
    
    /// Rozmiary okna aplikacji
    pub window_config: WindowConfig,
}

/// Konfiguracja okna aplikacji
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Domyślny rozmiar okna (szerokość, wysokość)
    pub default_size: (f32, f32),
    
    /// Minimalny rozmiar okna (szerokość, wysokość)
    pub min_size: (f32, f32),
    
    /// Tytuł okna
    pub title: String,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            default_simulation_speed: 2.0,
            min_simulation_speed: 0.5,
            max_simulation_speed: 30.0,
            simulation_speed_step: 0.5,
            default_button_size: (100.0, 30.0),
            window_config: WindowConfig::default(),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            default_size: (1200.0, 800.0),
            min_size: (800.0, 600.0),
            title: "Conway's Game of Life".to_string(),
        }
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            // Standardowe reguły Conway'a: B3/S23
            birth_neighbors: 3..=3,           // Narodziny przy dokładnie 3 sąsiadach
            survival_neighbors: 2..=3,        // Przeżycie przy 2 lub 3 sąsiadach
            
            // Tryb zarządzania planszą
            board_size_mode: BoardSizeMode::Dynamic,
            
            // Ograniczenia rozmiaru planszy (tryb Dynamic)
            max_board_size: 101,              // Maksymalny rozmiar 101x101
            initial_board_size: 9,            // Początkowy rozmiar planszy
            
            // Stały rozmiar planszy (tryb Static)
            static_board_size: 21,            // Domyślny stały rozmiar 21x21
            
            // Parametry rozszerzania
            expansion_margin: 2,              // Rozszerzaj gdy żywe komórki są 2 pola od krawędzi
            expansion_layers: 1,              // Dodawaj 1 warstwę na raz
            optimization_margin: 3,           // Pozostaw 3 pola marginesu przy optymalizacji
            
            // Konfiguracja interfejsu użytkownika
            ui_config: UIConfig::default(),
            
            // Konfiguracja randomizera
            randomizer_config: RandomizerConfig::default(),
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
    
    /// Zwraca aktualny rozmiar planszy w zależności od trybu
    pub fn get_current_board_size(&self) -> usize {
        match self.board_size_mode {
            BoardSizeMode::Dynamic => self.initial_board_size,
            BoardSizeMode::Static => self.static_board_size,
        }
    }
    
    /// Sprawdza czy można rozszerzać planszę w aktualnym trybie
    pub fn can_expand_in_current_mode(&self) -> bool {
        self.board_size_mode == BoardSizeMode::Dynamic
    }
    
    /// Ustawia nowy przedział dla narodzin komórek
    pub fn set_birth_neighbors(&mut self, min: usize, max: usize) {
        self.birth_neighbors = min..=max;
    }
    
    /// Ustawia nowy przedział dla przeżycia komórek
    pub fn set_survival_neighbors(&mut self, min: usize, max: usize) {
        self.survival_neighbors = min..=max;
    }
    
    /// Ustawia tryb zarządzania planszą
    pub fn set_board_size_mode(&mut self, mode: BoardSizeMode) {
        self.board_size_mode = mode;
    }
    
    /// Ustawia maksymalny rozmiar planszy (tryb Dynamic)
    pub fn set_max_board_size(&mut self, size: usize) {
        self.max_board_size = size.max(3).min(201); // Ograniczenie 3-201
    }
    
    /// Ustawia początkowy rozmiar planszy (tryb Dynamic)
    pub fn set_initial_board_size(&mut self, size: usize) {
        self.initial_board_size = size.max(3).min(201); // Ograniczenie 3-201
    }
    
    /// Ustawia stały rozmiar planszy (tryb Static)
    pub fn set_static_board_size(&mut self, size: usize) {
        self.static_board_size = size.max(3).min(201); // Ograniczenie 3-201
    }
    
    /// Ustawia bazowe prawdopodobieństwo randomizera
    pub fn set_randomizer_base_probability(&mut self, probability: f32) {
        self.randomizer_config.base_probability = probability.max(0.0).min(1.0);
    }
    
    /// Ustawia bonus prawdopodobieństwa za sąsiada
    pub fn set_randomizer_neighbor_bonus(&mut self, bonus: f32) {
        self.randomizer_config.neighbor_bonus = bonus.max(0.0).min(1.0);
    }
}

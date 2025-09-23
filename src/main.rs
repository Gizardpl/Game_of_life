mod config;
mod logic;
mod ui;

use config::{init_config, get_default_initial_state};
use logic::board::{Board};
use logic::change_state::CellStateManager;
use logic::prediction::{predict_next_state, PredictionResult};
use logic::reset::ResetManager;
use logic::randomizer;
use ui::{GameRenderer, SidePanel, MouseInteraction};
use ui::side_panel::{SimulationState, UserAction};

use eframe::egui;
use std::time::{Duration, Instant};

/// Główna aplikacja gry w życie
struct GameOfLifeApp {
    /// Aktualna plansza gry
    board: Board,
    /// Początkowy stan planszy (do resetowania)
    initial_board: Board,
    /// Renderer planszy
    renderer: GameRenderer,
    /// Panel boczny z kontrolkami
    side_panel: SidePanel,
    /// Manager zarządzania zmianą stanu komórek
    cell_state_manager: CellStateManager,
    /// Czas ostatniej aktualizacji
    last_update: Instant,
    /// Przewidywanie następnego stanu (cache)
    current_prediction: Option<PredictionResult>,
    /// Czy aplikacja była kiedykolwiek uruchomiona
    ever_started: bool,
    /// Manager odpowiedzialny za logikę resetowania
    reset_manager: ResetManager,
}

impl Default for GameOfLifeApp {
    fn default() -> Self {
        // Inicjalizujemy konfigurację
        init_config();
        
        // Tworzymy początkowy stan planszy
        let initial_state = get_default_initial_state();
        let initial_board = initial_state.create_board();
        let board = initial_board.clone();
        
        let mut side_panel = SidePanel::new();
        side_panel.set_alive_cells_count(board.count_alive_cells());
        
        Self {
            board,
            initial_board,
            renderer: GameRenderer::new(),
            side_panel,
            cell_state_manager: CellStateManager::new(),
            last_update: Instant::now(),
            current_prediction: None,
            ever_started: false,
            reset_manager: ResetManager::new(),
        }
    }
}

impl eframe::App for GameOfLifeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Sprawdzamy czy należy wykonać następny krok symulacji
        if self.side_panel.simulation_state() == SimulationState::Running {
            let elapsed = self.last_update.elapsed();
            let target_duration = Duration::from_secs_f32(self.side_panel.time_between_generations());
            
            if elapsed >= target_duration {
                self.next_generation();
                self.last_update = Instant::now();
            }
            
            // Żądamy ponownego renderowania dla płynnej animacji
            ctx.request_repaint();
        }
        
        // Główny layout aplikacji
        egui::CentralPanel::default().show(ctx, |ui| {
            // Pobieramy dostępny obszar
            let available_rect = ui.available_rect_before_wrap();
            
            // Obliczamy rozmiar panelu bocznego (szerokość ekranu - wysokość ekranu)
            let board_size = available_rect.height(); // Plansza jest kwadratem o boku równym wysokości
            let side_panel_width = available_rect.width() - board_size;
            
            ui.horizontal(|ui| {
                // Panel boczny po lewej stronie
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(side_panel_width, available_rect.height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        let action = self.side_panel.render(ui);
                        self.handle_user_action(action);
                    }
                );
                
                // Obszar renderowania planszy po prawej stronie
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(board_size, available_rect.height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        let board_rect = ui.available_rect_before_wrap();
                        
                        // Aktualizujemy przewidywanie jeśli potrzeba
                        self.update_prediction_if_needed();
                        
                        // Renderujemy planszę z podglądem
                        let mouse_interaction = self.renderer.render_board_with_preview(
                            ui, 
                            &self.board, 
                            board_rect,
                            self.current_prediction.as_ref(),
                            self.side_panel.show_next_state_preview(),
                            self.side_panel.show_previous_state_preview()
                        );
                        
                        // Obsługujemy interakcje myszy tylko gdy symulacja zatrzymana
                        if self.side_panel.simulation_state() == SimulationState::Stopped {
                            self.handle_mouse_interaction(mouse_interaction);
                        }
                    }
                );
            });
        });
    }
}

impl GameOfLifeApp {
    /// Obsługuje akcje użytkownika z panelu bocznego
    fn handle_user_action(&mut self, action: UserAction) {
        match action {
            UserAction::Start => {
                // Jeśli to pierwsze uruchomienie, zapisujemy aktualny stan planszy
                if !self.ever_started {
                    self.reset_manager.save_pre_start_state(&self.board);
                }
                
                self.side_panel.set_simulation_state(SimulationState::Running);
                self.last_update = Instant::now();
                self.ever_started = true;
            }
            UserAction::Stop => {
                self.side_panel.set_simulation_state(SimulationState::Stopped);
            }
            UserAction::Reset => {
                self.reset_to_initial_state();
            }
            UserAction::Step => {
                if self.side_panel.simulation_state() == SimulationState::Stopped {
                    self.next_generation();
                }
            }
            UserAction::EditCell(x, y) => {
                // Edycja komórki jest dozwolona tylko gdy symulacja jest zatrzymana
                if self.side_panel.simulation_state() == SimulationState::Stopped {
                    if self.cell_state_manager.handle_cell_click(&mut self.board, x, y) {
                        // Aktualizujemy liczbę żywych komórek po zmianie
                        self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
                        // Invalidujemy cache przewidywania po zmianie
                        self.current_prediction = None;
                    }
                }
            }
            UserAction::RulesChanged => {
                // Zasady gry zostały zmienione - invalidujemy cache przewidywania
                self.current_prediction = None;
            }
            UserAction::BoardSettingsChanged => {
                // Ustawienia planszy zostały zmienione - invalidujemy cache przewidywania
                // Nie zmieniamy rozmiaru planszy automatycznie - to powinno się dziać tylko
                // przez explicit BoardSizeChanged lub Reset
                self.current_prediction = None;
            }
            UserAction::BoardSizeChanged(new_size) => {
                // Zmieniono rozmiar planszy - musimy zmienić rozmiar aktualnej planszy
                self.resize_board_to(new_size);
            }
            UserAction::RandomFill => {
                // Generuj losową planszę - tylko gdy symulacja jest zatrzymana
                if self.side_panel.simulation_state() == SimulationState::Stopped {
                    self.generate_random_board();
                }
            }
            UserAction::None => {
                // Brak akcji
            }
        }
    }
    
    /// Obsługuje interakcje myszy z planszą
    fn handle_mouse_interaction(&mut self, interaction: MouseInteraction) {
        let mut board_changed = false;
        
        // Obsługa kliknięcia (bez przeciągania)
        if let Some((x, y)) = interaction.clicked_cell {
            if !self.cell_state_manager.is_dragging() {
                board_changed = self.cell_state_manager.handle_cell_click(&mut self.board, x, y);
            }
        }
        
        // Obsługa rozpoczęcia przeciągania
        if interaction.mouse_pressed {
            if let Some((x, y)) = interaction.hovered_cell {
                board_changed = self.cell_state_manager.start_drag(&mut self.board, x, y);
            }
        }
        
        // Obsługa kontynuacji przeciągania
        if interaction.is_mouse_down && self.cell_state_manager.is_dragging() {
            if let Some((x, y)) = interaction.hovered_cell {
                if self.cell_state_manager.handle_mouse_over(&mut self.board, x, y) {
                    board_changed = true;
                }
            }
        }
        
        // Obsługa zakończenia przeciągania
        if interaction.mouse_released {
            self.cell_state_manager.end_drag();
        }
        
        // Aktualizujemy liczbę żywych komórek jeśli plansza się zmieniła
        if board_changed {
            self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
            // Invalidujemy cache przewidywania po zmianie planszy
            self.current_prediction = None;
        }
    }
    
    /// Wykonuje następną generację gry
    fn next_generation(&mut self) {
        self.board = self.board.next_generation();
        self.side_panel.increment_generation();
        self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
        
        // Sprawdzamy czy plansza potrzebuje rozszerzenia
        let config = config::get_config();
        if let Some(expanded_board) = self.board.auto_expand_if_needed(config.expansion_margin) {
            self.board = expanded_board;
        }
        
        // Invalidujemy cache przewidywania po zmianie stanu
        self.current_prediction = None;
    }
    
    /// Resetuje planszę do stanu początkowego
    fn reset_to_initial_state(&mut self) {
        // Zatrzymujemy symulację
        self.side_panel.set_simulation_state(SimulationState::Stopped);
        self.side_panel.reset_generation_count();
        self.cell_state_manager.reset();
        
        // Używamy ResetManager do obsługi logiki resetowania
        let (new_board, should_reset_ever_started) = self.reset_manager.reset_board(&self.board, self.ever_started);
        
        // Aktualizujemy planszę
        self.board = new_board;
        
        // Resetujemy flagę ever_started jeśli to konieczne
        if should_reset_ever_started {
            self.ever_started = false;
        }
        
        // Aktualizujemy planszę początkową
        self.initial_board = self.board.clone();
        
        // Aktualizujemy statystyki
        self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
        
        // Synchronizujemy ustawienia w GUI z konfiguracją po resecie
        self.side_panel.sync_settings_with_config();
        
        // Invalidujemy cache przewidywania po resecie
        self.current_prediction = None;
    }
    
    /// Aktualizuje przewidywanie następnego stanu jeśli jest potrzebne
    fn update_prediction_if_needed(&mut self) {
        // Obliczamy przewidywanie tylko jeśli:
        // 1. Symulacja jest zatrzymana (aby nie obciążać podczas działania)
        // 2. Użytkownik włączył podgląd
        // 3. Nie mamy jeszcze cache'owanego przewidywania
        if self.side_panel.simulation_state() == SimulationState::Stopped 
            && (self.side_panel.show_next_state_preview() || self.side_panel.show_previous_state_preview())
            && self.current_prediction.is_none() {
            self.current_prediction = Some(predict_next_state(&self.board));
        }
        
        // Jeśli użytkownik wyłączył podgląd, możemy wyczyścić cache
        if !self.side_panel.show_next_state_preview() && !self.side_panel.show_previous_state_preview() {
            self.current_prediction = None;
        }
    }
    
    /// Zmienia rozmiar planszy do podanego rozmiaru
    fn resize_board_to(&mut self, new_size: usize) {
        // Zatrzymujemy symulację podczas zmiany rozmiaru
        self.side_panel.set_simulation_state(SimulationState::Stopped);
        
        // Pobieramy aktualne ustawienia z konfiguracji
        let config = config::get_config();
        
        // Zmieniamy rozmiar tylko jeśli aplikacja nie była jeszcze uruchomiona
        // lub jeśli użytkownik świadomie zmienia rozmiar w trybie Static
        if !self.ever_started {
            // Aplikacja nie była uruchomiona - możemy bezpiecznie zmienić rozmiar
            self.board = self.board.resize_to_square(new_size);
            self.initial_board = self.board.clone();
            
            // Aktualizujemy liczbę żywych komórek
            self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
        } else {
            // Aplikacja była uruchomiona - w obu trybach pozwalamy na zmianę rozmiaru
            // ale w trybie Dynamic nie zmieniamy aktualnej planszy, tylko zapisujemy nowy rozmiar
            // który zostanie użyty przy następnym resecie
            if config.board_size_mode == config::BoardSizeMode::Static {
                // W trybie Static zmieniamy rozmiar natychmiast
                self.board = self.board.resize_to_square(new_size);
                
                // Aktualizujemy też zapisany stan przed uruchomieniem jeśli istnieje
                if self.reset_manager.has_pre_start_state() {
                    // Tworzymy tymczasową planszę do aktualizacji stanu przed uruchomieniem
                    // To jest trochę skomplikowane, ale potrzebne aby zachować enkapsulację
                    let (temp_board, _) = self.reset_manager.reset_board(&self.board, true);
                    let resized_temp = temp_board.resize_to_square(new_size);
                    self.reset_manager.clear_pre_start_state();
                    self.reset_manager.save_pre_start_state(&resized_temp);
                }
                
                // Aktualizujemy liczbę żywych komórek
                self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
            }
            // W trybie Dynamic nie zmieniamy aktualnej planszy, ale nowy rozmiar
            // jest już zapisany w konfiguracji i zostanie użyty przy resecie
        }
        
        // Invalidujemy cache przewidywania
        self.current_prediction = None;
    }
    
    /// Generuje losową planszę używając inteligentnego algorytmu randomizera
    fn generate_random_board(&mut self) {
        // Generujemy nową losową planszę na podstawie aktualnego rozmiaru
        let new_board = randomizer::generate_random_board(&self.board);
        
        // Zastępujemy aktualną planszę nową losową planszą
        self.board = new_board;
        
        // Aktualizujemy liczbę żywych komórek w panelu bocznym
        self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
        
        // Invalidujemy cache przewidywania
        self.current_prediction = None;
        
        // Resetujemy licznik generacji, ponieważ to nowy początkowy stan
        self.side_panel.reset_generation_count();
        
        // Zapisujemy nowy stan jako stan początkowy do resetowania
        // (jeśli gra była już kiedyś uruchomiona)
        if self.ever_started {
            self.reset_manager.clear_pre_start_state();
            self.reset_manager.save_pre_start_state(&self.board);
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    // Inicjalizujemy konfigurację
    init_config();
    let config = config::get_config();
    
    // Konfiguracja okna aplikacji z centralnych ustawień
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                config.ui_config.window_config.default_size.0,
                config.ui_config.window_config.default_size.1
            ])
            .with_min_inner_size([
                config.ui_config.window_config.min_size.0,
                config.ui_config.window_config.min_size.1
            ])
            .with_title(&config.ui_config.window_config.title),
        ..Default::default()
    };
    
    // Uruchomienie aplikacji
    eframe::run_native(
        "Conway's Game of Life",
        options,
        Box::new(|_cc| {
            Ok(Box::new(GameOfLifeApp::default()))
        }),
    )
}

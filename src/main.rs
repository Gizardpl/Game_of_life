mod config;
mod logic;
mod ui;

use config::{init_config, get_default_initial_state};
use logic::board::Board;
use ui::{GameRenderer, SidePanel};
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
    /// Czas ostatniej aktualizacji
    last_update: Instant,
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
            last_update: Instant::now(),
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
                        if let Some((x, y)) = self.renderer.render_board(ui, &self.board, board_rect) {
                            // Użytkownik kliknął na komórkę - obsługujemy edycję tylko gdy symulacja zatrzymana
                            if self.side_panel.simulation_state() == SimulationState::Stopped {
                                self.handle_user_action(UserAction::EditCell(x, y));
                            }
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
                self.side_panel.set_simulation_state(SimulationState::Running);
                self.last_update = Instant::now();
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
                    if self.board.toggle_cell(x, y) {
                        // Aktualizujemy liczbę żywych komórek po zmianie
                        self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
                    }
                }
            }
            UserAction::None => {
                // Brak akcji
            }
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
    }
    
    /// Resetuje planszę do stanu początkowego
    fn reset_to_initial_state(&mut self) {
        self.board = self.initial_board.clone();
        self.side_panel.set_simulation_state(SimulationState::Stopped);
        self.side_panel.reset_generation_count();
        self.side_panel.set_alive_cells_count(self.board.count_alive_cells());
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

/// Panel boczny z kontrolkami gry
/// 
/// Zawiera przyciski Start/Stop, Reset oraz inne opcje sterowania symulacją.

use egui::{Button, RichText, Color32};

/// Stan symulacji
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimulationState {
    /// Symulacja jest zatrzymana
    Stopped,
    /// Symulacja jest uruchomiona
    Running,
}

/// Akcje które może wykonać użytkownik
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserAction {
    /// Uruchom symulację
    Start,
    /// Zatrzymaj symulację
    Stop,
    /// Resetuj planszę do stanu początkowego
    Reset,
    /// Wykonaj jeden krok symulacji
    Step,
    /// Brak akcji
    None,
}

/// Panel boczny z kontrolkami
pub struct SidePanel {
    /// Aktualny stan symulacji
    simulation_state: SimulationState,
    /// Liczba wykonanych generacji
    generation_count: u64,
    /// Liczba żywych komórek
    alive_cells_count: usize,
    /// Prędkość symulacji (generacje na sekundę)
    simulation_speed: f32,
}

impl Default for SidePanel {
    fn default() -> Self {
        let config = crate::config::get_config();
        Self {
            simulation_state: SimulationState::Stopped,
            generation_count: 0,
            alive_cells_count: 0,
            simulation_speed: config.ui_config.default_simulation_speed,
        }
    }
}

impl SidePanel {
    /// Tworzy nowy panel boczny
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Ustawia stan symulacji
    pub fn set_simulation_state(&mut self, state: SimulationState) {
        self.simulation_state = state;
    }
    
    /// Zwraca aktualny stan symulacji
    pub fn simulation_state(&self) -> SimulationState {
        self.simulation_state
    }
    
    /// Ustawia liczbę generacji
    pub fn set_generation_count(&mut self, count: u64) {
        self.generation_count = count;
    }
    
    /// Zwiększa liczbę generacji o 1
    pub fn increment_generation(&mut self) {
        self.generation_count += 1;
    }
    
    /// Resetuje licznik generacji
    pub fn reset_generation_count(&mut self) {
        self.generation_count = 0;
    }
    
    /// Ustawia liczbę żywych komórek
    pub fn set_alive_cells_count(&mut self, count: usize) {
        self.alive_cells_count = count;
    }
    
    /// Ustawia prędkość symulacji
    pub fn set_simulation_speed(&mut self, speed: f32) {
        let config = crate::config::get_config();
        self.simulation_speed = speed
            .max(config.ui_config.min_simulation_speed)
            .min(config.ui_config.max_simulation_speed);
    }
    
    /// Zwraca prędkość symulacji
    pub fn simulation_speed(&self) -> f32 {
        self.simulation_speed
    }
    
    /// Zwraca czas między generacjami w sekundach
    pub fn time_between_generations(&self) -> f32 {
        1.0 / self.simulation_speed
    }
    
    /// Renderuje panel boczny i zwraca akcję użytkownika
    pub fn render(&mut self, ui: &mut egui::Ui) -> UserAction {
        let mut action = UserAction::None;
        let config = crate::config::get_config();
        
        ui.vertical(|ui| {
            // Tytuł
            ui.heading(RichText::new("Conway's Game of Life").strong());
            ui.separator();
            
            // Sekcja kontroli
            ui.group(|ui| {
                ui.label(RichText::new("Controls").strong());
                
                // Przycisk Start/Stop
                let (button_text, button_color) = match self.simulation_state {
                    SimulationState::Stopped => ("▶ Start", Color32::GREEN),
                    SimulationState::Running => ("⏸ Stop", Color32::RED),
                };
                
                let start_stop_button = Button::new(
                    RichText::new(button_text).color(button_color).strong()
                ).min_size(egui::Vec2::new(
                    config.ui_config.default_button_size.0,
                    config.ui_config.default_button_size.1
                ));
                
                if ui.add(start_stop_button).clicked() {
                    action = match self.simulation_state {
                        SimulationState::Stopped => UserAction::Start,
                        SimulationState::Running => UserAction::Stop,
                    };
                }
                
                // Przycisk Reset
                let reset_button = Button::new(
                    RichText::new("🔄 Reset").color(Color32::BLUE).strong()
                ).min_size(egui::Vec2::new(
                    config.ui_config.default_button_size.0,
                    config.ui_config.default_button_size.1
                ));
                
                if ui.add(reset_button).clicked() {
                    action = UserAction::Reset;
                }
                
                // Przycisk Step (tylko gdy symulacja zatrzymana)
                if self.simulation_state == SimulationState::Stopped {
                    let step_button = Button::new(
                        RichText::new("⏭ Step").color(Color32::GRAY).strong()
                    ).min_size(egui::Vec2::new(
                        config.ui_config.default_button_size.0,
                        config.ui_config.default_button_size.1
                    ));
                    
                    if ui.add(step_button).clicked() {
                        action = UserAction::Step;
                    }
                }
            });
            
            ui.separator();
            
            // Sekcja statystyk
            ui.group(|ui| {
                ui.label(RichText::new("Statistics").strong());
                
                ui.horizontal(|ui| {
                    ui.label("Generation:");
                    ui.label(RichText::new(format!("{}", self.generation_count)).monospace());
                });
                
                ui.horizontal(|ui| {
                    ui.label("Alive cells:");
                    ui.label(RichText::new(format!("{}", self.alive_cells_count)).monospace());
                });
                
                ui.horizontal(|ui| {
                    ui.label("Status:");
                    let (status_text, status_color) = match self.simulation_state {
                        SimulationState::Stopped => ("Stopped", Color32::RED),
                        SimulationState::Running => ("Running", Color32::GREEN),
                    };
                    ui.label(RichText::new(status_text).color(status_color).strong());
                });
            });
            
            ui.separator();
            
            // Sekcja ustawień
            ui.group(|ui| {
                ui.label(RichText::new("Settings").strong());
                
                ui.horizontal(|ui| {
                    ui.label("Speed:");
                    if ui.add(egui::Slider::new(
                        &mut self.simulation_speed, 
                        config.ui_config.min_simulation_speed..=config.ui_config.max_simulation_speed
                    ).step_by(config.ui_config.simulation_speed_step as f64)
                     .text("gen/s")).changed() {
                        // Prędkość została zmieniona
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Interval:");
                    ui.label(RichText::new(format!("{:.1}ms", 
                        self.time_between_generations() * 1000.0)).monospace());
                });
            });
            
            ui.separator();
            
            // Sekcja informacji
            ui.group(|ui| {
                ui.label(RichText::new("Instructions").strong());
                ui.label("• Click Start to begin simulation");
                ui.label("• Use Reset to restore initial state");
                ui.label("• Step executes one generation");
                ui.label("• Adjust speed with the slider");
            });
        });
        
        action
    }
}
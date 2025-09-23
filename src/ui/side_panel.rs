/// Panel boczny z kontrolkami gry
/// 
/// Zawiera przyciski Start/Stop, Reset oraz inne opcje sterowania symulacją.

use egui::RichText;
use super::settings::{SettingsPanel, SettingsAction};
use super::styles::{UIStyles, ButtonType, TextType, helpers};

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
    /// Edytuj komórkę na podanych współrzędnych (x, y)
    EditCell(usize, usize),
    /// Zmieniono zasady gry
    RulesChanged,
    /// Zmieniono ustawienia planszy
    BoardSettingsChanged,
    /// Zmieniono rozmiar planszy (nowy rozmiar)
    BoardSizeChanged(usize),
    /// Wygeneruj losową planszę
    RandomFill,
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
    /// Czy pokazywać podgląd zmian (zarówno narodziny jak i śmierci)
    show_preview: bool,
    /// Czy sekcja instrukcji jest rozwinięta
    instructions_expanded: bool,
    /// Panel ustawień gry
    settings_panel: SettingsPanel,
    /// Style UI
    styles: UIStyles,
}

impl Default for SidePanel {
    fn default() -> Self {
        let config = crate::config::get_config();
        Self {
            simulation_state: SimulationState::Stopped,
            generation_count: 0,
            alive_cells_count: 0,
            simulation_speed: config.ui_config.default_simulation_speed,
            show_preview: false,
            instructions_expanded: false,
            settings_panel: SettingsPanel::new(),
            styles: UIStyles::new(),
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
    
    /// Ustawia czy pokazywać podgląd zmian
    pub fn set_show_preview(&mut self, show: bool) {
        self.show_preview = show;
    }
    
    /// Zwraca czy pokazywać podgląd zmian
    pub fn show_preview(&self) -> bool {
        self.show_preview
    }
    
    /// Zwraca czy pokazywać podgląd następnego stanu (dla kompatybilności wstecznej)
    pub fn show_next_state_preview(&self) -> bool {
        self.show_preview
    }
    
    /// Zwraca czy pokazywać podgląd poprzedniego stanu (dla kompatybilności wstecznej)
    pub fn show_previous_state_preview(&self) -> bool {
        self.show_preview
    }
    
    /// Renderuje panel boczny i zwraca akcję użytkownika
    pub fn render(&mut self, ui: &mut egui::Ui) -> UserAction {
        let mut action = UserAction::None;
        let config = crate::config::get_config();
        
        // Dodajemy scroll area do całego panelu
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Tytuł aplikacji
                    ui.add_space(self.styles.dimensions.margin_medium);
                    ui.label(helpers::section_header("Conway's Game of Life", &self.styles));
                    ui.add_space(self.styles.separator_spacing());
                    
                    // Sekcja kontroli z prędkością
                    self.styles.group_style().show(ui, |ui| {
                        ui.label(helpers::section_header("Controls", &self.styles));
                        ui.add_space(self.styles.dimensions.margin_small);
                        
                        // Przyciski kontroli w jednym rzędzie
                        ui.horizontal(|ui| {
                            // Przycisk Start/Stop
                            let (button_text, button_color) = match self.simulation_state {
                                SimulationState::Stopped => ("▶ Start", self.styles.colors.button_start),
                                SimulationState::Running => ("⏸ Stop", self.styles.colors.button_stop),
                            };
                            
                            if ui.add(helpers::styled_button(button_text, button_color, &self.styles, ButtonType::Medium)).clicked() {
                                action = match self.simulation_state {
                                    SimulationState::Stopped => UserAction::Start,
                                    SimulationState::Running => UserAction::Stop,
                                };
                            }
                            
                            // Przycisk Reset
                            if ui.add(helpers::styled_button("🔄 Reset", self.styles.colors.button_reset, &self.styles, ButtonType::Medium)).clicked() {
                                action = UserAction::Reset;
                            }
                            
                            // Przycisk Step (tylko gdy symulacja zatrzymana)
                            if self.simulation_state == SimulationState::Stopped {
                                if ui.add(helpers::styled_button("⏭ Step", self.styles.colors.button_step, &self.styles, ButtonType::Medium)).clicked() {
                                    action = UserAction::Step;
                                }
                            }
                        });
                        
                        ui.add_space(self.styles.dimensions.margin_medium);
                        
                        // Ustawienia prędkości w tej samej sekcji
                        ui.add_space(self.styles.dimensions.margin_medium);
                        
                        // Kontener dla kontroli prędkości z lepszym layoutem
                        ui.vertical(|ui| {
                            ui.label(helpers::subsection_header("Speed", &self.styles));
                            ui.add_space(self.styles.dimensions.margin_small);
                            
                            ui.horizontal(|ui| {
                                // Przycisk zmniejszenia prędkości
                                let can_decrease = self.simulation_speed > config.ui_config.min_simulation_speed;
                                if ui.add(helpers::arrow_button("◀", can_decrease, &self.styles)).clicked() && can_decrease {
                                    self.simulation_speed = (self.simulation_speed - config.ui_config.simulation_speed_step)
                                        .max(config.ui_config.min_simulation_speed);
                                }
                                
                                // Slider prędkości - wydłużony, zajmuje dostępną przestrzeń
                                ui.allocate_ui_with_layout(
                                    egui::Vec2::new(ui.available_width() - 80.0, self.styles.dimensions.slider_height),
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        if ui.add(helpers::wide_slider(
                                            &mut self.simulation_speed, 
                                            config.ui_config.min_simulation_speed..=config.ui_config.max_simulation_speed,
                                            "gen/s",
                                            &self.styles
                                        ).step_by(config.ui_config.simulation_speed_step as f64)).changed() {
                                            // Prędkość została zmieniona
                                        }
                                    }
                                );
                                
                                // Przycisk zwiększenia prędkości
                                let can_increase = self.simulation_speed < config.ui_config.max_simulation_speed;
                                if ui.add(helpers::arrow_button("▶", can_increase, &self.styles)).clicked() && can_increase {
                                    self.simulation_speed = (self.simulation_speed + config.ui_config.simulation_speed_step)
                                        .min(config.ui_config.max_simulation_speed);
                                }
                            });
                        });
                    });
                    
                    ui.add_space(self.styles.separator_spacing());
                    
                    // Sekcja statystyk z podglądem
                    self.styles.group_style().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Statystyki po lewej
                            ui.vertical(|ui| {
                                ui.label(helpers::section_header("Statistics", &self.styles));
                                ui.add_space(self.styles.dimensions.margin_small);
                                
                                ui.horizontal(|ui| {
                                    ui.label(helpers::label_text("Generation:", &self.styles));
                                    ui.label(helpers::value_text(&format!("{}", self.generation_count), &self.styles));
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label(helpers::label_text("Alive cells:", &self.styles));
                                    ui.label(helpers::value_text(&format!("{}", self.alive_cells_count), &self.styles));
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label(helpers::label_text("Status:", &self.styles));
                                    let (status_text, status_color) = match self.simulation_state {
                                        SimulationState::Stopped => ("Stopped", self.styles.colors.error),
                                        SimulationState::Running => ("Running", self.styles.colors.success),
                                    };
                                    ui.label(RichText::new(status_text)
                                        .font(self.styles.font_id(TextType::Medium))
                                        .color(status_color)
                                        .strong());
                                });
                            });
                            
                            ui.separator();
                            
                            // Preview Options po prawej - wyłączone gdy gra jest uruchomiona
                            ui.vertical(|ui| {
                                let is_running = self.simulation_state == SimulationState::Running;
                                let header_color = if is_running { self.styles.colors.text_disabled } else { self.styles.colors.text_primary };
                                
                                ui.label(RichText::new("Preview Options")
                                    .font(self.styles.font_id(TextType::Large))
                                    .color(header_color)
                                    .strong());
                                ui.add_space(self.styles.dimensions.margin_small);
                                
                                ui.add_enabled_ui(!is_running, |ui| {
                                    ui.horizontal(|ui| {
                                        helpers::styled_checkbox(ui, &mut self.show_preview, "Show changes", &self.styles);
                                        if ui.small_button("?").on_hover_text("Show cells that will be born (green) and die (red) in the next generation").clicked() {
                                            // Tooltip jest już wyświetlany przez on_hover_text
                                        }
                                    });
                                });
                                
                                // Pokazuj Birth/Deaths tylko gdy gra jest zatrzymana I show_preview jest zaznaczone
                                if self.show_preview && !is_running {
                                    ui.horizontal(|ui| {
                                        ui.colored_label(self.styles.colors.preview_birth, "● Births");
                                        ui.colored_label(self.styles.colors.preview_death, "● Deaths");
                                    });
                                }
                                
                                // Przycisk Random Fill - tylko gdy gra jest zatrzymana
                                ui.add_enabled_ui(!is_running, |ui| {
                                    ui.add_space(self.styles.dimensions.margin_small);
                                    if ui.add(helpers::styled_button("🎲 Random Fill", self.styles.colors.button_step, &self.styles, ButtonType::Medium)).clicked() {
                                        action = UserAction::RandomFill;
                                    }
                                });
                                // Gdy gra jest uruchomiona, nie pokazujemy wcale Birth/Deaths
                            });
                        });
                    });
                    
                    ui.add_space(self.styles.separator_spacing());
                    
                    // Sekcja ustawień gry ze stylizowanymi zagnieżdżeniami
                    let settings_action = self.render_styled_settings(ui);
                    match settings_action {
                        SettingsAction::RulesChanged => action = UserAction::RulesChanged,
                        SettingsAction::BoardSettingsChanged => action = UserAction::BoardSettingsChanged,
                        SettingsAction::BoardSizeChanged(size) => action = UserAction::BoardSizeChanged(size),
                        SettingsAction::ResetRules => action = UserAction::RulesChanged,
                        SettingsAction::ResetBoardSettings => action = UserAction::BoardSettingsChanged,
                        SettingsAction::RandomizerChanged => {}, // Randomizer nie wymaga akcji - tylko zmiana konfiguracji
                        SettingsAction::ResetRandomizer => {}, // Reset randomizera też nie wymaga akcji
                        SettingsAction::None => {}
                    }
                    
                    ui.add_space(self.styles.separator_spacing());
                    
                    // Sekcja informacji (zwijalna)
                    self.styles.group_style().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let instructions_text = if self.instructions_expanded {
                                "🔽 Instructions & Editing"
                            } else {
                                "▶ Instructions & Editing"
                            };
                            
                            if ui.add(helpers::styled_button(instructions_text, self.styles.colors.text_primary, &self.styles, ButtonType::Large)).clicked() {
                                self.instructions_expanded = !self.instructions_expanded;
                            }
                        });
                        
                        if self.instructions_expanded {
                            ui.add_space(self.styles.dimensions.margin_medium);
                            
                            ui.label(helpers::subsection_header("Controls:", &self.styles));
                            ui.label(helpers::label_text("• Click Start to begin simulation", &self.styles));
                            ui.label(helpers::label_text("• Use Reset to restore initial state", &self.styles));
                            ui.label(helpers::label_text("• Step executes one generation", &self.styles));
                            ui.label(helpers::label_text("• Adjust speed with the slider", &self.styles));
                            
                            ui.add_space(self.styles.dimensions.margin_small);
                            
                            ui.label(helpers::subsection_header("Editing:", &self.styles));
                            ui.label(helpers::label_text("• Click cells when stopped to edit", &self.styles));
                            ui.label(helpers::label_text("• Toggle cells between alive/dead", &self.styles));
                            ui.label(helpers::label_text("• Changes persist in next generations", &self.styles));
                        }
                    });
                });
            });
        
        action
    }
    
    /// Renderuje stylizowaną sekcję ustawień gry
    fn render_styled_settings(&mut self, ui: &mut egui::Ui) -> SettingsAction {
        // Delegujemy do settings_panel, ale z naszymi stylami
        self.settings_panel.render_with_styles(ui, &self.styles)
    }
    
    /// Synchronizuje ustawienia z konfiguracją
    pub fn sync_settings_with_config(&mut self) {
        self.settings_panel.sync_with_config();
    }
}
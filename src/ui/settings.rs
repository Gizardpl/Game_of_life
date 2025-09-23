/// Moduł ustawień gry w interfejsie użytkownika
/// 
/// Zawiera komponenty UI do edycji zasad gry i ustawień planszy.

use egui::{Slider, RichText, Color32};
use crate::config::{BoardSizeMode, modify_config, get_config};
use super::styles::{UIStyles, ButtonType, TextType, helpers};

/// Akcje związane z ustawieniami
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SettingsAction {
    /// Brak akcji
    None,
    /// Zmieniono zasady gry
    RulesChanged,
    /// Zmieniono ustawienia planszy
    BoardSettingsChanged,
    /// Zmieniono rozmiar planszy (nowy rozmiar)
    BoardSizeChanged(usize),
    /// Zresetuj zasady gry do wartości domyślnych
    ResetRules,
    /// Zresetuj ustawienia planszy do wartości domyślnych
    ResetBoardSettings,
}

/// Panel ustawień gry
pub struct SettingsPanel {
    /// Czy sekcja ustawień jest rozwinięta
    settings_expanded: bool,
    /// Czy sekcja zasad jest rozwinięta
    rules_expanded: bool,
    /// Czy sekcja ustawień planszy jest rozwinięta
    board_settings_expanded: bool,
    
    // Lokalne kopie wartości do edycji
    birth_min: usize,
    birth_max: usize,
    survival_min: usize,
    survival_max: usize,
    board_mode: BoardSizeMode,
    max_board_size: usize,
    initial_board_size: usize,
    static_board_size: usize,
}

impl Default for SettingsPanel {
    fn default() -> Self {
        let config = get_config();
        Self {
            settings_expanded: false,
            rules_expanded: false,
            board_settings_expanded: false,
            birth_min: *config.birth_neighbors.start(),
            birth_max: *config.birth_neighbors.end(),
            survival_min: *config.survival_neighbors.start(),
            survival_max: *config.survival_neighbors.end(),
            board_mode: config.board_size_mode,
            max_board_size: config.max_board_size,
            initial_board_size: config.initial_board_size,
            static_board_size: config.static_board_size,
        }
    }
}

impl SettingsPanel {
    /// Tworzy nowy panel ustawień
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Synchronizuje lokalne wartości z globalną konfiguracją
    pub fn sync_with_config(&mut self) {
        let config = get_config();
        self.birth_min = *config.birth_neighbors.start();
        self.birth_max = *config.birth_neighbors.end();
        self.survival_min = *config.survival_neighbors.start();
        self.survival_max = *config.survival_neighbors.end();
        self.board_mode = config.board_size_mode;
        self.max_board_size = config.max_board_size;
        self.initial_board_size = config.initial_board_size;
        self.static_board_size = config.static_board_size;
    }
    
    /// Renderuje panel ustawień
    pub fn render(&mut self, ui: &mut egui::Ui) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        // Główna sekcja ustawień (zwijalna)
        ui.group(|ui| {
            ui.horizontal(|ui| {
                let settings_text = if self.settings_expanded {
                    "▼ Game Settings"
                } else {
                    "▶ Game Settings"
                };
                
                if ui.button(RichText::new(settings_text).strong()).clicked() {
                    self.settings_expanded = !self.settings_expanded;
                }
            });
            
            if self.settings_expanded {
                ui.separator();
                
                // Sekcja zasad gry
                action = self.render_rules_section(ui).max(action);
                
                ui.separator();
                
                // Sekcja ustawień planszy
                action = self.render_board_settings_section(ui).max(action);
            }
        });
        
        action
    }
    
    /// Renderuje sekcję zasad gry
    fn render_rules_section(&mut self, ui: &mut egui::Ui) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        ui.horizontal(|ui| {
            let rules_text = if self.rules_expanded {
                "▼ Game Rules"
            } else {
                "▶ Game Rules"
            };
            
            if ui.button(RichText::new(rules_text).strong()).clicked() {
                self.rules_expanded = !self.rules_expanded;
            }
            
            // Przycisk resetowania zasad
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button(RichText::new("🗑 Restart Settings").color(Color32::RED)).clicked() {
                    action = SettingsAction::ResetRules;
                }
            });
        });
        
        if self.rules_expanded {
            ui.indent("rules", |ui| {
                // Birth Neighbors
                ui.label(RichText::new("Birth Neighbors:").strong());
                ui.horizontal(|ui| {
                    ui.label("Min:");
                    if ui.add(Slider::new(&mut self.birth_min, 0..=8)).changed() {
                        if self.birth_min > self.birth_max {
                            self.birth_max = self.birth_min;
                        }
                        action = SettingsAction::RulesChanged;
                    }
                    
                    ui.label("Max:");
                    if ui.add(Slider::new(&mut self.birth_max, 0..=8)).changed() {
                        if self.birth_max < self.birth_min {
                            self.birth_min = self.birth_max;
                        }
                        action = SettingsAction::RulesChanged;
                    }
                });
                
                // Wyświetl aktualny przedział
                let birth_range_text = if self.birth_min == self.birth_max {
                    format!("Birth at: {}", self.birth_min)
                } else {
                    format!("Birth range: {}-{}", self.birth_min, self.birth_max)
                };
                ui.label(RichText::new(birth_range_text).color(Color32::GRAY).small());
                
                ui.separator();
                
                // Survival Neighbors
                ui.label(RichText::new("Survival Neighbors:").strong());
                ui.horizontal(|ui| {
                    ui.label("Min:");
                    if ui.add(Slider::new(&mut self.survival_min, 0..=8)).changed() {
                        if self.survival_min > self.survival_max {
                            self.survival_max = self.survival_min;
                        }
                        action = SettingsAction::RulesChanged;
                    }
                    
                    ui.label("Max:");
                    if ui.add(Slider::new(&mut self.survival_max, 0..=8)).changed() {
                        if self.survival_max < self.survival_min {
                            self.survival_min = self.survival_max;
                        }
                        action = SettingsAction::RulesChanged;
                    }
                });
                
                // Wyświetl aktualny przedział
                let survival_range_text = if self.survival_min == self.survival_max {
                    format!("Survive at: {}", self.survival_min)
                } else {
                    format!("Survival range: {}-{}", self.survival_min, self.survival_max)
                };
                ui.label(RichText::new(survival_range_text).color(Color32::GRAY).small());
                
                // Zastosuj zmiany
                if action == SettingsAction::RulesChanged {
                    modify_config(|config| {
                        config.set_birth_neighbors(self.birth_min, self.birth_max);
                        config.set_survival_neighbors(self.survival_min, self.survival_max);
                    });
                } else if action == SettingsAction::ResetRules {
                    // Resetuj do wartości domyślnych
                    let default_config = crate::config::rules::GameConfig::default();
                    self.birth_min = *default_config.birth_neighbors.start();
                    self.birth_max = *default_config.birth_neighbors.end();
                    self.survival_min = *default_config.survival_neighbors.start();
                    self.survival_max = *default_config.survival_neighbors.end();
                    
                    modify_config(|config| {
                        config.set_birth_neighbors(self.birth_min, self.birth_max);
                        config.set_survival_neighbors(self.survival_min, self.survival_max);
                    });
                    
                    action = SettingsAction::RulesChanged; // Informuj o zmianie
                }
            });
        }
        
        action
    }
    
    /// Renderuje sekcję ustawień planszy
    fn render_board_settings_section(&mut self, ui: &mut egui::Ui) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        ui.horizontal(|ui| {
            let board_text = if self.board_settings_expanded {
                "▼ Board Settings"
            } else {
                "▶ Board Settings"
            };
            
            if ui.button(RichText::new(board_text).strong()).clicked() {
                self.board_settings_expanded = !self.board_settings_expanded;
            }
            
            // Przycisk resetowania ustawień planszy
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.small_button(RichText::new("🗑 Restart Settings").color(Color32::RED)).clicked() {
                    action = SettingsAction::ResetBoardSettings;
                }
            });
        });
        
        if self.board_settings_expanded {
            ui.indent("board", |ui| {
                // Przełącznik trybu
                ui.label(RichText::new("Board Mode:").strong());
                ui.horizontal(|ui| {
                    if ui.radio_value(&mut self.board_mode, BoardSizeMode::Dynamic, "Dynamic").clicked() {
                        action = SettingsAction::BoardSettingsChanged;
                    }
                    if ui.radio_value(&mut self.board_mode, BoardSizeMode::Static, "Static").clicked() {
                        action = SettingsAction::BoardSettingsChanged;
                    }
                });
                
                ui.separator();
                
                // Ustawienia w zależności od trybu
                match self.board_mode {
                    BoardSizeMode::Dynamic => {
                        action = self.render_dynamic_settings(ui).max(action);
                    }
                    BoardSizeMode::Static => {
                        action = self.render_static_settings(ui).max(action);
                    }
                }
                
                // Zastosuj zmiany trybu
                if action == SettingsAction::BoardSettingsChanged {
                    modify_config(|config| {
                        config.set_board_size_mode(self.board_mode);
                        config.set_max_board_size(self.max_board_size);
                        config.set_initial_board_size(self.initial_board_size);
                        config.set_static_board_size(self.static_board_size);
                    });
                } else if action == SettingsAction::ResetBoardSettings {
                    // Resetuj do wartości domyślnych
                    let default_config = crate::config::rules::GameConfig::default();
                    self.board_mode = default_config.board_size_mode;
                    self.max_board_size = default_config.max_board_size;
                    self.initial_board_size = default_config.initial_board_size;
                    self.static_board_size = default_config.static_board_size;
                    
                    modify_config(|config| {
                        config.set_board_size_mode(self.board_mode);
                        config.set_max_board_size(self.max_board_size);
                        config.set_initial_board_size(self.initial_board_size);
                        config.set_static_board_size(self.static_board_size);
                    });
                    
                    action = SettingsAction::BoardSettingsChanged; // Informuj o zmianie
                }
            });
        }
        
        action
    }
    
    /// Renderuje ustawienia trybu dynamicznego
    fn render_dynamic_settings(&mut self, ui: &mut egui::Ui) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        ui.label(RichText::new("Dynamic Mode Settings:").color(Color32::BLUE));
        ui.label("Board expands automatically when cells reach edges");
        
        ui.horizontal(|ui| {
            ui.label("Initial size:");
            if ui.add(Slider::new(&mut self.initial_board_size, 3..=201)
                .step_by(2.0) // Tylko nieparzyste wartości
                .text("cells")).changed() {
                // Zapewnij nieparzystość
                if self.initial_board_size % 2 == 0 {
                    self.initial_board_size += 1;
                }
                // W trybie Dynamic, zmiana Initial Size powinna natychmiast zmienić rozmiar planszy
                // Zapisujemy zmianę do konfiguracji natychmiast
                modify_config(|config| {
                    config.set_initial_board_size(self.initial_board_size);
                });
                action = SettingsAction::BoardSizeChanged(self.initial_board_size);
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Max size:");
            if ui.add(Slider::new(&mut self.max_board_size, 3..=201)
                .step_by(2.0) // Tylko nieparzyste wartości
                .text("cells")).changed() {
                // Zapewnij nieparzystość
                if self.max_board_size % 2 == 0 {
                    self.max_board_size += 1;
                }
                // Upewnij się, że max >= initial
                let old_initial_size = self.initial_board_size;
                if self.max_board_size < self.initial_board_size {
                    self.initial_board_size = self.max_board_size;
                }
                
                // Wyślij akcję zmiany rozmiaru planszy tylko jeśli initial size rzeczywiście się zmienił
                // i tylko jeśli aplikacja nie była jeszcze uruchomiona (aby nie psuć aktualnej planszy)
                if old_initial_size != self.initial_board_size {
                    action = SettingsAction::BoardSizeChanged(self.initial_board_size);
                } else {
                    action = SettingsAction::BoardSettingsChanged;
                }
            }
        });
        
        action
    }
    
    /// Renderuje ustawienia trybu statycznego
    fn render_static_settings(&mut self, ui: &mut egui::Ui) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        ui.label(RichText::new("Static Mode Settings:").color(Color32::RED));
        ui.label("Board has fixed size - no automatic expansion");
        
        let old_size = self.static_board_size;
        
        ui.horizontal(|ui| {
            ui.label("Board size:");
            if ui.add(Slider::new(&mut self.static_board_size, 3..=201)
                .step_by(2.0) // Tylko nieparzyste wartości
                .text("cells")).changed() {
                // Zapewnij nieparzystość
                if self.static_board_size % 2 == 0 {
                    self.static_board_size += 1;
                }
                
                // Zapisujemy zmianę do konfiguracji natychmiast
                modify_config(|config| {
                    config.set_static_board_size(self.static_board_size);
                });
                
                action = SettingsAction::BoardSettingsChanged;
                
                // Jeśli rozmiar się zmienił, wyślij dodatkową akcję
                if old_size != self.static_board_size {
                    action = SettingsAction::BoardSizeChanged(self.static_board_size);
                }
            }
        });
        
        action
    }
    
    /// Renderuje panel ustawień z niestandardowymi stylami
    pub fn render_with_styles(&mut self, ui: &mut egui::Ui, styles: &UIStyles) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        // Główna sekcja ustawień (zwijalna) ze stylizowanym wyglądem
        styles.group_style().show(ui, |ui| {
            ui.horizontal(|ui| {
                let settings_text = if self.settings_expanded {
                    "🔽 Game Settings"
                } else {
                    "▶ Game Settings"
                };
                
                if ui.add(helpers::styled_button(settings_text, styles.colors.text_primary, styles, ButtonType::Large)).clicked() {
                    self.settings_expanded = !self.settings_expanded;
                }
            });
            
            if self.settings_expanded {
                ui.add_space(styles.dimensions.margin_medium);
                
                // Sekcja zasad gry
                action = self.render_rules_section_styled(ui, styles).max(action);
                
                ui.add_space(styles.separator_spacing());
                
                // Sekcja ustawień planszy
                action = self.render_board_settings_section_styled(ui, styles).max(action);
            }
        });
        
        action
    }
    
    /// Renderuje sekcję zasad gry ze stylami
    fn render_rules_section_styled(&mut self, ui: &mut egui::Ui, styles: &UIStyles) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        styles.nested_group_style().show(ui, |ui| {
            ui.horizontal(|ui| {
                let rules_text = if self.rules_expanded {
                    "🔽 Game Rules"
                } else {
                    "▶ Game Rules"
                };
                
                if ui.add(helpers::styled_button(rules_text, styles.colors.text_secondary, styles, ButtonType::Medium)).clicked() {
                    self.rules_expanded = !self.rules_expanded;
                }
                
                // Przycisk resetowania zasad
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(helpers::styled_button("🗑 Reset", styles.colors.error, styles, ButtonType::Small)).clicked() {
                        action = SettingsAction::ResetRules;
                    }
                });
            });
            
            if self.rules_expanded {
                ui.add_space(styles.dimensions.margin_medium);
                
                // Birth Neighbors
                ui.label(helpers::subsection_header("Birth Neighbors:", styles));
                ui.add_space(styles.dimensions.margin_small);
                
                ui.horizontal(|ui| {
                    ui.label(helpers::label_text("Min:", styles));
                    if ui.add(Slider::new(&mut self.birth_min, 0..=8)
                        .text("")
                        .min_decimals(0)
                        .max_decimals(0)).changed() {
                        if self.birth_min > self.birth_max {
                            self.birth_max = self.birth_min;
                        }
                        action = SettingsAction::RulesChanged;
                    }
                    
                    ui.label(helpers::label_text("Max:", styles));
                    if ui.add(Slider::new(&mut self.birth_max, 0..=8)
                        .text("")
                        .min_decimals(0)
                        .max_decimals(0)).changed() {
                        if self.birth_max < self.birth_min {
                            self.birth_min = self.birth_max;
                        }
                        action = SettingsAction::RulesChanged;
                    }
                });
                
                // Wyświetl aktualny przedział
                let birth_range_text = if self.birth_min == self.birth_max {
                    format!("Birth at: {}", self.birth_min)
                } else {
                    format!("Birth range: {}-{}", self.birth_min, self.birth_max)
                };
                ui.label(RichText::new(birth_range_text)
                    .font(styles.font_id(TextType::Small))
                    .color(styles.colors.text_muted));
                
                ui.add_space(styles.dimensions.margin_medium);
                
                // Survival Neighbors
                ui.label(helpers::subsection_header("Survival Neighbors:", styles));
                ui.add_space(styles.dimensions.margin_small);
                
                ui.horizontal(|ui| {
                    ui.label(helpers::label_text("Min:", styles));
                    if ui.add(Slider::new(&mut self.survival_min, 0..=8)
                        .text("")
                        .min_decimals(0)
                        .max_decimals(0)).changed() {
                        if self.survival_min > self.survival_max {
                            self.survival_max = self.survival_min;
                        }
                        action = SettingsAction::RulesChanged;
                    }
                    
                    ui.label(helpers::label_text("Max:", styles));
                    if ui.add(Slider::new(&mut self.survival_max, 0..=8)
                        .text("")
                        .min_decimals(0)
                        .max_decimals(0)).changed() {
                        if self.survival_max < self.survival_min {
                            self.survival_min = self.survival_max;
                        }
                        action = SettingsAction::RulesChanged;
                    }
                });
                
                // Wyświetl aktualny przedział
                let survival_range_text = if self.survival_min == self.survival_max {
                    format!("Survive at: {}", self.survival_min)
                } else {
                    format!("Survival range: {}-{}", self.survival_min, self.survival_max)
                };
                ui.label(RichText::new(survival_range_text)
                    .font(styles.font_id(TextType::Small))
                    .color(styles.colors.text_muted));
                
                // Zastosuj zmiany
                if action == SettingsAction::RulesChanged {
                    modify_config(|config| {
                        config.set_birth_neighbors(self.birth_min, self.birth_max);
                        config.set_survival_neighbors(self.survival_min, self.survival_max);
                    });
                } else if action == SettingsAction::ResetRules {
                    // Resetuj do wartości domyślnych
                    let default_config = crate::config::rules::GameConfig::default();
                    self.birth_min = *default_config.birth_neighbors.start();
                    self.birth_max = *default_config.birth_neighbors.end();
                    self.survival_min = *default_config.survival_neighbors.start();
                    self.survival_max = *default_config.survival_neighbors.end();
                    
                    modify_config(|config| {
                        config.set_birth_neighbors(self.birth_min, self.birth_max);
                        config.set_survival_neighbors(self.survival_min, self.survival_max);
                    });
                    
                    action = SettingsAction::RulesChanged; // Informuj o zmianie
                }
            }
        });
        
        action
    }
    
    /// Renderuje sekcję ustawień planszy ze stylami
    fn render_board_settings_section_styled(&mut self, ui: &mut egui::Ui, styles: &UIStyles) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        styles.nested_group_style().show(ui, |ui| {
            ui.horizontal(|ui| {
                let board_text = if self.board_settings_expanded {
                    "🔽 Board Settings"
                } else {
                    "▶ Board Settings"
                };
                
                if ui.add(helpers::styled_button(board_text, styles.colors.text_secondary, styles, ButtonType::Medium)).clicked() {
                    self.board_settings_expanded = !self.board_settings_expanded;
                }
                
                // Przycisk resetowania ustawień planszy
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(helpers::styled_button("🗑 Reset", styles.colors.error, styles, ButtonType::Small)).clicked() {
                        action = SettingsAction::ResetBoardSettings;
                    }
                });
            });
            
            if self.board_settings_expanded {
                ui.add_space(styles.dimensions.margin_medium);
                
                // Przełącznik trybu
                ui.label(helpers::subsection_header("Board Mode:", styles));
                ui.add_space(styles.dimensions.margin_small);
                
                ui.horizontal(|ui| {
                    if ui.radio_value(&mut self.board_mode, BoardSizeMode::Dynamic, "Dynamic").clicked() {
                        action = SettingsAction::BoardSettingsChanged;
                    }
                    if ui.radio_value(&mut self.board_mode, BoardSizeMode::Static, "Static").clicked() {
                        action = SettingsAction::BoardSettingsChanged;
                    }
                });
                
                ui.add_space(styles.dimensions.margin_medium);
                
                // Ustawienia w zależności od trybu
                match self.board_mode {
                    BoardSizeMode::Dynamic => {
                        action = self.render_dynamic_settings_styled(ui, styles).max(action);
                    }
                    BoardSizeMode::Static => {
                        action = self.render_static_settings_styled(ui, styles).max(action);
                    }
                }
                
                // Zastosuj zmiany trybu
                if action == SettingsAction::BoardSettingsChanged {
                    modify_config(|config| {
                        config.set_board_size_mode(self.board_mode);
                        config.set_max_board_size(self.max_board_size);
                        config.set_initial_board_size(self.initial_board_size);
                        config.set_static_board_size(self.static_board_size);
                    });
                } else if action == SettingsAction::ResetBoardSettings {
                    // Resetuj do wartości domyślnych
                    let default_config = crate::config::rules::GameConfig::default();
                    self.board_mode = default_config.board_size_mode;
                    self.max_board_size = default_config.max_board_size;
                    self.initial_board_size = default_config.initial_board_size;
                    self.static_board_size = default_config.static_board_size;
                    
                    modify_config(|config| {
                        config.set_board_size_mode(self.board_mode);
                        config.set_max_board_size(self.max_board_size);
                        config.set_initial_board_size(self.initial_board_size);
                        config.set_static_board_size(self.static_board_size);
                    });
                    
                    action = SettingsAction::BoardSettingsChanged; // Informuj o zmianie
                }
            }
        });
        
        action
    }
    
    /// Renderuje ustawienia trybu dynamicznego ze stylami
    fn render_dynamic_settings_styled(&mut self, ui: &mut egui::Ui, styles: &UIStyles) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        ui.label(RichText::new("Dynamic Mode Settings:")
            .font(styles.font_id(TextType::Medium))
            .color(styles.colors.info));
        ui.label(helpers::label_text("Board expands automatically when cells reach edges", styles));
        
        ui.add_space(styles.dimensions.margin_small);
        
        ui.horizontal(|ui| {
            ui.label(helpers::label_text("Initial size:", styles));
            if ui.add(Slider::new(&mut self.initial_board_size, 3..=201)
                .step_by(2.0) // Tylko nieparzyste wartości
                .text("cells")).changed() {
                // Zapewnij nieparzystość
                if self.initial_board_size % 2 == 0 {
                    self.initial_board_size += 1;
                }
                // W trybie Dynamic, zmiana Initial Size powinna natychmiast zmienić rozmiar planszy
                // Zapisujemy zmianę do konfiguracji natychmiast
                modify_config(|config| {
                    config.set_initial_board_size(self.initial_board_size);
                });
                action = SettingsAction::BoardSizeChanged(self.initial_board_size);
            }
        });
        
        ui.horizontal(|ui| {
            ui.label(helpers::label_text("Max size:", styles));
            if ui.add(Slider::new(&mut self.max_board_size, 3..=201)
                .step_by(2.0) // Tylko nieparzyste wartości
                .text("cells")).changed() {
                // Zapewnij nieparzystość
                if self.max_board_size % 2 == 0 {
                    self.max_board_size += 1;
                }
                // Upewnij się, że max >= initial
                let old_initial_size = self.initial_board_size;
                if self.max_board_size < self.initial_board_size {
                    self.initial_board_size = self.max_board_size;
                }
                
                // Wyślij akcję zmiany rozmiaru planszy tylko jeśli initial size rzeczywiście się zmienił
                // i tylko jeśli aplikacja nie była jeszcze uruchomiona (aby nie psuć aktualnej planszy)
                if old_initial_size != self.initial_board_size {
                    action = SettingsAction::BoardSizeChanged(self.initial_board_size);
                } else {
                    action = SettingsAction::BoardSettingsChanged;
                }
            }
        });
        
        action
    }
    
    /// Renderuje ustawienia trybu statycznego ze stylami
    fn render_static_settings_styled(&mut self, ui: &mut egui::Ui, styles: &UIStyles) -> SettingsAction {
        let mut action = SettingsAction::None;
        
        ui.label(RichText::new("Static Mode Settings:")
            .font(styles.font_id(TextType::Medium))
            .color(styles.colors.error));
        ui.label(helpers::label_text("Board has fixed size - no automatic expansion", styles));
        
        ui.add_space(styles.dimensions.margin_small);
        
        let old_size = self.static_board_size;
        
        ui.horizontal(|ui| {
            ui.label(helpers::label_text("Board size:", styles));
            if ui.add(Slider::new(&mut self.static_board_size, 3..=201)
                .step_by(2.0) // Tylko nieparzyste wartości
                .text("cells")).changed() {
                // Zapewnij nieparzystość
                if self.static_board_size % 2 == 0 {
                    self.static_board_size += 1;
                }
                
                // Zapisujemy zmianę do konfiguracji natychmiast
                modify_config(|config| {
                    config.set_static_board_size(self.static_board_size);
                });
                
                action = SettingsAction::BoardSettingsChanged;
                
                // Jeśli rozmiar się zmienił, wyślij dodatkową akcję
                if old_size != self.static_board_size {
                    action = SettingsAction::BoardSizeChanged(self.static_board_size);
                }
            }
        });
        
        action
    }
}


use egui::{Image, Vec2, Rect, Color32, Stroke, Pos2};
use crate::assets::{PatternManager, Pattern};
use super::styles::{UIStyles, helpers};

/// Selektor wzorów do umieszczania na planszy
pub struct PatternSelector {
    pattern_manager: PatternManager,
    styles: UIStyles,
}

impl PatternSelector {
    pub fn new() -> Self {
        Self {
            pattern_manager: PatternManager::new(),
            styles: UIStyles::new(),
        }
    }
    
    /// Renderuje sekcję wyboru wzorów
    pub fn render(&mut self, ui: &mut egui::Ui, simulation_stopped: bool) -> Option<String> {
        let mut selected_pattern = None;
        
        ui.group(|ui| {
            ui.add_enabled_ui(simulation_stopped, |ui| {
            
            // Nagłówek sekcji
            ui.label(helpers::section_header("Predefined Patterns", &self.styles));
            ui.add_space(self.styles.dimensions.margin_small);
            
            if !simulation_stopped {
                ui.label(helpers::disabled_text("Stop simulation to use patterns", &self.styles));
                return;
            }
            
            // Siatka wzorów
            let patterns = self.pattern_manager.get_all_patterns();
            
            if patterns.is_empty() {
                ui.label(helpers::label_text("No patterns available", &self.styles));
                return;
            }
            
            // Renderujemy wzory w układzie adaptacyjnym
            let available_width = ui.available_width();
            let spacing = 10.0;
            let base_height = 80.0; // bazowa wysokość wzoru
            
            // Renderujemy każdy wzór osobno z odpowiednim rozmiarem
            for pattern in patterns {
                let pattern_width = if pattern.name == "Glider Gun" {
                    // Glider Gun ma podwójną szerokość
                    available_width - spacing
                } else {
                    // Pozostałe wzory mają pełną szerokość
                    available_width - spacing
                };
                
                let pattern_height = if pattern.name == "Glider Gun" {
                    // Glider Gun ma mniejszą wysokość (prostokątny)
                    base_height * 0.6
                } else {
                    // Pozostałe wzory mają standardową wysokość
                    base_height
                };
                
                if self.render_pattern_button(ui, pattern, pattern_width, pattern_height) {
                    selected_pattern = Some(pattern.name.clone());
                }
                ui.add_space(spacing);
            }
            });
        });
        
        selected_pattern
    }
    
    /// Renderuje przycisk dla pojedynczego wzoru
    fn render_pattern_button(&self, ui: &mut egui::Ui, pattern: &Pattern, width: f32, height: f32) -> bool {
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());
        
        // Tło przycisku
        let bg_color = if response.hovered() {
            self.styles.colors.hover_overlay
        } else {
            self.styles.colors.background_medium
        };
        
        ui.painter().rect_filled(rect, 4.0, bg_color);
        
        // Ramka
        let stroke_color = if response.hovered() {
            self.styles.colors.text_primary
        } else {
            self.styles.colors.border_subtle
        };
        
        let stroke = Stroke::new(1.0, stroke_color);
        ui.painter().rect_stroke(rect, 4.0, stroke, egui::StrokeKind::Inside);
        
        // Próbujemy załadować obrazek wzoru
        if let Some(image_path) = &pattern.image_path {
            // Sprawdzamy czy plik istnieje
            if std::path::Path::new(image_path).exists() {
                // Renderujemy obrazek - wykorzystujemy prawie całą dostępną przestrzeń
                let padding = 4.0;
                let image_rect = rect.shrink(padding);
                
                ui.scope_builder(egui::UiBuilder::new().max_rect(image_rect), |ui| {
                    ui.add(Image::from_uri(format!("file://{}", image_path))
                        .fit_to_exact_size(image_rect.size()));
                });
            } else {
                // Fallback - renderujemy wzór jako mini planszę
                self.render_pattern_preview(ui, pattern, rect);
            }
        } else {
            // Renderujemy wzór jako mini planszę
            self.render_pattern_preview(ui, pattern, rect);
        }
        
        // Nazwa wzoru pod przyciskiem
        let text_rect = Rect::from_min_size(
            Pos2::new(rect.min.x, rect.max.y + 2.0),
            Vec2::new(width, 20.0)
        );
        
        ui.scope_builder(egui::UiBuilder::new().max_rect(text_rect), |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(helpers::small_text(&pattern.name, &self.styles));
            });
        });
        
        response.clicked()
    }
    
    /// Renderuje podgląd wzoru jako mini planszę
    fn render_pattern_preview(&self, ui: &mut egui::Ui, pattern: &Pattern, rect: Rect) {
        let padding = 8.0;
        let preview_rect = rect.shrink(padding);
        
        // Obliczamy rozmiar komórki bazując na dostępnej przestrzeni i rozmiarze wzoru
        let cell_size_x = preview_rect.width() / pattern.size.0 as f32;
        let cell_size_y = preview_rect.height() / pattern.size.1 as f32;
        let cell_size = cell_size_x.min(cell_size_y).floor().max(1.0);
        
        // Centrujemy wzór w dostępnym obszarze
        let pattern_width = pattern.size.0 as f32 * cell_size;
        let pattern_height = pattern.size.1 as f32 * cell_size;
        
        let start_x = preview_rect.center().x - pattern_width / 2.0;
        let start_y = preview_rect.center().y - pattern_height / 2.0;
        
        // Renderujemy tło wzoru
        let pattern_rect = Rect::from_min_size(
            Pos2::new(start_x, start_y),
            Vec2::new(pattern_width, pattern_height)
        );
        
        ui.painter().rect_filled(pattern_rect, 0.0, Color32::from_gray(240));
        
        // Renderujemy żywe komórki
        for cell in &pattern.cells {
            let cell_rect = Rect::from_min_size(
                Pos2::new(
                    start_x + cell.x as f32 * cell_size,
                    start_y + cell.y as f32 * cell_size
                ),
                Vec2::splat(cell_size)
            );
            
            ui.painter().rect_filled(cell_rect, 0.0, Color32::BLACK);
        }
        
        // Siatka (opcjonalnie dla większych wzorów)
        if cell_size > 3.0 {
            for x in 0..=pattern.size.0 {
                let line_x = start_x + x as f32 * cell_size;
                ui.painter().line_segment(
                    [Pos2::new(line_x, start_y), Pos2::new(line_x, start_y + pattern_height)],
                    Stroke::new(0.5, Color32::from_gray(200))
                );
            }
            
            for y in 0..=pattern.size.1 {
                let line_y = start_y + y as f32 * cell_size;
                ui.painter().line_segment(
                    [Pos2::new(start_x, line_y), Pos2::new(start_x + pattern_width, line_y)],
                    Stroke::new(0.5, Color32::from_gray(200))
                );
            }
        }
    }
    
    /// Zwraca wzór o podanej nazwie
    pub fn get_pattern(&self, name: &str) -> Option<&Pattern> {
        self.pattern_manager.get_pattern(name)
    }
}

impl Default for PatternSelector {
    fn default() -> Self {
        Self::new()
    }
}

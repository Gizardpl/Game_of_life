/// Moduł renderowania podglądu następnego stanu gry
/// 
/// Zawiera funkcje do renderowania zielonych podświetleń komórek,
/// które będą żywe w następnej generacji.

use egui::{Color32, Pos2, Rect, Vec2};
use crate::logic::prediction::PredictionResult;

/// Renderer podglądu następnego stanu
pub struct PreviewRenderer {
    /// Kolor podświetlenia komórek, które się narodzą (delikatnie zielony, przezroczysty)
    birth_highlight_color: Color32,
    /// Kolor podświetlenia komórek, które umrą (delikatnie czerwony, przezroczysty)
    death_highlight_color: Color32,
}

impl Default for PreviewRenderer {
    fn default() -> Self {
        Self {
            // Delikatnie zielony, przezroczysty kolor dla komórek, które się narodzą
            birth_highlight_color: Color32::from_rgba_unmultiplied(0, 255, 0, 60),
            // Delikatnie czerwony, przezroczysty kolor dla komórek, które umrą
            death_highlight_color: Color32::from_rgba_unmultiplied(255, 0, 0, 40),
        }
    }
}

impl PreviewRenderer {
    /// Tworzy nowy renderer podglądu z domyślnymi ustawieniami
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Ustawia kolor podświetlenia komórek, które się narodzą
    pub fn set_birth_highlight_color(&mut self, color: Color32) {
        self.birth_highlight_color = color;
    }
    
    /// Ustawia kolor podświetlenia komórek, które umrą
    pub fn set_death_highlight_color(&mut self, color: Color32) {
        self.death_highlight_color = color;
    }
    
    /// Renderuje podświetlenia komórek na podstawie przewidywania
    pub fn render_preview_highlights(
        &self,
        ui: &mut egui::Ui,
        prediction: &PredictionResult,
        board_rect: Rect,
        cell_size: f32,
        show_births: bool,
        show_deaths: bool,
    ) {
        let painter = ui.painter();
        
        // Renderujemy podświetlenia komórek, które się narodzą
        if show_births {
            for &(x, y) in &prediction.birth_cells {
                let cell_rect = self.get_cell_rect(board_rect, x, y, cell_size);
                painter.rect_filled(cell_rect, 0.0, self.birth_highlight_color);
            }
        }
        
        // Renderujemy podświetlenia komórek, które umrą
        if show_deaths {
            for &(x, y) in &prediction.death_cells {
                let cell_rect = self.get_cell_rect(board_rect, x, y, cell_size);
                painter.rect_filled(cell_rect, 0.0, self.death_highlight_color);
            }
        }
    }
    
    /// Renderuje tylko podświetlenia komórek, które się narodzą
    pub fn render_birth_highlights(
        &self,
        ui: &mut egui::Ui,
        birth_cells: &[(usize, usize)],
        board_rect: Rect,
        cell_size: f32,
    ) {
        let painter = ui.painter();
        
        for &(x, y) in birth_cells {
            let cell_rect = self.get_cell_rect(board_rect, x, y, cell_size);
            painter.rect_filled(cell_rect, 0.0, self.birth_highlight_color);
        }
    }
    
    /// Renderuje tylko podświetlenia komórek, które umrą
    pub fn render_death_highlights(
        &self,
        ui: &mut egui::Ui,
        death_cells: &[(usize, usize)],
        board_rect: Rect,
        cell_size: f32,
    ) {
        let painter = ui.painter();
        
        for &(x, y) in death_cells {
            let cell_rect = self.get_cell_rect(board_rect, x, y, cell_size);
            painter.rect_filled(cell_rect, 0.0, self.death_highlight_color);
        }
    }
    
    /// Oblicza prostokąt dla pojedynczej komórki
    fn get_cell_rect(&self, board_rect: Rect, x: usize, y: usize, cell_size: f32) -> Rect {
        let cell_min = Pos2::new(
            board_rect.min.x + x as f32 * cell_size,
            board_rect.min.y + y as f32 * cell_size,
        );
        
        Rect::from_min_size(cell_min, Vec2::splat(cell_size))
    }
    
    /// Zwraca aktualny kolor podświetlenia komórek, które się narodzą
    pub fn birth_highlight_color(&self) -> Color32 {
        self.birth_highlight_color
    }
    
    /// Zwraca aktualny kolor podświetlenia komórek, które umrą
    pub fn death_highlight_color(&self) -> Color32 {
        self.death_highlight_color
    }
}

/// Pomocnicze funkcje do tworzenia kolorów podświetleń
pub mod colors {
    use egui::Color32;
    
    /// Tworzy delikatnie zielony, przezroczysty kolor dla podświetleń narodzin
    pub fn birth_highlight(alpha: u8) -> Color32 {
        Color32::from_rgba_unmultiplied(0, 255, 0, alpha)
    }
    
    /// Tworzy delikatnie czerwony, przezroczysty kolor dla podświetleń śmierci
    pub fn death_highlight(alpha: u8) -> Color32 {
        Color32::from_rgba_unmultiplied(255, 0, 0, alpha)
    }
    
    /// Tworzy delikatnie żółty, przezroczysty kolor dla innych podświetleń
    pub fn warning_highlight(alpha: u8) -> Color32 {
        Color32::from_rgba_unmultiplied(255, 255, 0, alpha)
    }
    
    /// Tworzy delikatnie niebieski, przezroczysty kolor dla informacji
    pub fn info_highlight(alpha: u8) -> Color32 {
        Color32::from_rgba_unmultiplied(0, 0, 255, alpha)
    }
}

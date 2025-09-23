/// Moduł renderowania planszy gry w życie
/// 
/// Odpowiada za wizualizację stanu gry w oknie aplikacji.
/// Plansza jest renderowana jako kwadrat wyrównany do prawej strony.

use egui::{Color32, Pos2, Rect, Stroke, Vec2};
use crate::logic::board::{Board, CellState};
use crate::logic::prediction::PredictionResult;
use crate::assets::Pattern;
use super::preview_render::PreviewRenderer;

/// Informacje o interakcji myszy z planszą
#[derive(Debug, Clone)]
pub struct MouseInteraction {
    /// Współrzędne komórki, na którą kliknięto (lewy przycisk myszy)
    pub clicked_cell: Option<(usize, usize)>,
    /// Współrzędne komórki, nad którą znajduje się kursor
    pub hovered_cell: Option<(usize, usize)>,
    /// Czy lewy przycisk myszy jest wciśnięty
    pub is_mouse_down: bool,
    /// Czy lewy przycisk myszy został właśnie wciśnięty
    pub mouse_pressed: bool,
    /// Czy lewy przycisk myszy został właśnie zwolniony
    pub mouse_released: bool,
}

/// Renderer planszy gry
pub struct GameRenderer {
    /// Rozmiar pojedynczej komórki w pikselach
    cell_size: f32,
    /// Kolor żywych komórek
    alive_color: Color32,
    /// Kolor martwych komórek
    dead_color: Color32,
    /// Kolor siatki
    grid_color: Color32,
    /// Grubość linii siatki
    grid_stroke: Stroke,
    /// Renderer podglądu następnego stanu
    preview_renderer: PreviewRenderer,
}

impl Default for GameRenderer {
    fn default() -> Self {
        Self {
            cell_size: 10.0,
            alive_color: Color32::BLACK,
            dead_color: Color32::WHITE,
            grid_color: Color32::GRAY,
            grid_stroke: Stroke::new(1.0, Color32::GRAY),
            preview_renderer: PreviewRenderer::new(),
        }
    }
}

impl GameRenderer {
    /// Tworzy nowy renderer z domyślnymi ustawieniami
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Ustawia rozmiar komórki
    pub fn set_cell_size(&mut self, size: f32) {
        self.cell_size = size.max(1.0);
    }
    
    /// Zwraca aktualny rozmiar komórki
    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }
    
    /// Oblicza rozmiar planszy w pikselach
    pub fn calculate_board_size(&self, board: &Board) -> Vec2 {
        Vec2::new(
            board.width() as f32 * self.cell_size,
            board.height() as f32 * self.cell_size,
        )
    }
    
    /// Oblicza optymalny rozmiar komórki dla danej wysokości okna
    pub fn calculate_optimal_cell_size(&self, board: &Board, available_height: f32) -> f32 {
        let board_height = board.height() as f32;
        if board_height > 0.0 {
            (available_height / board_height).max(1.0)
        } else {
            self.cell_size
        }
    }
    
    /// Renderuje planszę w podanym obszarze i zwraca informacje o interakcji myszy
    pub fn render_board(
        &mut self,
        ui: &mut egui::Ui,
        board: &Board,
        available_rect: Rect,
    ) -> MouseInteraction {
        self.render_board_with_preview(ui, board, available_rect, None, false, false)
    }
    
    /// Renderuje planszę z podglądem następnego stanu
    pub fn render_board_with_preview(
        &mut self,
        ui: &mut egui::Ui,
        board: &Board,
        available_rect: Rect,
        prediction: Option<&PredictionResult>,
        show_births: bool,
        show_deaths: bool,
    ) -> MouseInteraction {
        self.render_board_with_pattern_preview(
            ui, board, available_rect, prediction, show_births, show_deaths, None
        )
    }
    
    /// Renderuje planszę z podglądem wzoru do umieszczenia
    pub fn render_board_with_pattern_preview(
        &mut self,
        ui: &mut egui::Ui,
        board: &Board,
        available_rect: Rect,
        prediction: Option<&PredictionResult>,
        show_births: bool,
        show_deaths: bool,
        pattern_preview: Option<&Pattern>,
    ) -> MouseInteraction {
        // Obliczamy optymalny rozmiar komórki na podstawie wysokości
        let optimal_cell_size = self.calculate_optimal_cell_size(board, available_rect.height());
        self.set_cell_size(optimal_cell_size);
        
        // Obliczamy rozmiar planszy w pikselach
        let board_size = self.calculate_board_size(board);
        
        // Wyrównujemy planszę do prawej strony dostępnego obszaru
        let board_rect = Rect::from_min_size(
            Pos2::new(
                available_rect.max.x - board_size.x,
                available_rect.min.y,
            ),
            board_size,
        );
        
        // Sprawdzamy czy plansza mieści się w dostępnym obszarze
        let final_board_rect = if board_rect.min.x < available_rect.min.x {
            // Jeśli plansza nie mieści się, centrujemy ją
            Rect::from_center_size(available_rect.center(), board_size)
        } else {
            board_rect
        };
        
        // Sprawdzamy interakcje myszy PRZED renderowaniem, żeby móc użyć hover do podglądu wzoru
        let pointer_pos = ui.input(|i| i.pointer.interact_pos());
        let hovered_cell = if let Some(pos) = pointer_pos {
            self.screen_to_cell_coords(final_board_rect, pos)
        } else {
            None
        };
        
        // Renderujemy planszę
        self.render_board_in_rect(ui, board, final_board_rect);
        
        // Renderujemy podgląd wzoru jeśli jest wybrany i myszka jest nad planszą
        if let (Some(pattern), Some((hover_x, hover_y))) = (pattern_preview, hovered_cell) {
            self.render_pattern_hover_preview(ui, pattern, final_board_rect, hover_x, hover_y);
        }
        
        // Renderujemy podgląd następnego stanu jeśli jest dostępny
        if let Some(prediction) = prediction {
            self.preview_renderer.render_preview_highlights(
                ui, 
                prediction, 
                final_board_rect, 
                self.cell_size, 
                show_births, 
                show_deaths
            );
        }
        
        let clicked_cell = if ui.input(|i| i.pointer.any_click()) {
            hovered_cell
        } else {
            None
        };
        
        let is_mouse_down = ui.input(|i| i.pointer.primary_down());
        let mouse_pressed = ui.input(|i| i.pointer.primary_pressed());
        let mouse_released = ui.input(|i| i.pointer.primary_released());
        
        MouseInteraction {
            clicked_cell,
            hovered_cell,
            is_mouse_down,
            mouse_pressed,
            mouse_released,
        }
    }
    
    /// Renderuje podgląd wzoru pod kursorem myszy
    fn render_pattern_hover_preview(
        &self,
        ui: &mut egui::Ui,
        pattern: &Pattern,
        board_rect: Rect,
        hover_x: usize,
        hover_y: usize,
    ) {
        let painter = ui.painter();
        let center_pos = crate::assets::Position::new(hover_x as i32, hover_y as i32);
        
        // Podświetlamy centrum wzoru (żółty)
        let center_cell_rect = self.get_cell_rect(board_rect, hover_x, hover_y);
        painter.rect_filled(center_cell_rect, 0.0, Color32::YELLOW);
        
        // Renderujemy podgląd wzoru (półprzezroczyste komórki)
        let pattern_cells = pattern.get_cells_at_center(center_pos);
        for pos in pattern_cells {
            if pos.x >= 0 && pos.y >= 0 {
                let x = pos.x as usize;
                let y = pos.y as usize;
                
                let cell_rect = self.get_cell_rect(board_rect, x, y);
                // Sprawdzamy czy komórka jest w granicach planszy
                if board_rect.contains(cell_rect.center()) {
                    painter.rect_filled(cell_rect, 0.0, Color32::from_rgba_unmultiplied(0, 255, 0, 100));
                }
            }
        }
        
        // Renderujemy obszar, który zostanie wyczyszczony (półprzezroczyste czerwone)
        let clear_area = pattern.get_clear_area(center_pos);
        for pos in clear_area {
            if pos.x >= 0 && pos.y >= 0 {
                let x = pos.x as usize;
                let y = pos.y as usize;
                
                let cell_rect = self.get_cell_rect(board_rect, x, y);
                // Sprawdzamy czy komórka jest w granicach planszy
                if board_rect.contains(cell_rect.center()) {
                    let stroke = Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 0, 0, 150));
                    painter.rect_stroke(cell_rect, 0.0, stroke, egui::StrokeKind::Inside);
                }
            }
        }
    }
    
    /// Renderuje planszę w określonym prostokącie
    fn render_board_in_rect(
        &self,
        ui: &mut egui::Ui,
        board: &Board,
        rect: Rect,
    ) {
        let painter = ui.painter();
        
        // Renderujemy tło planszy
        painter.rect_filled(rect, 0.0, self.dead_color);
        
        // Renderujemy komórki
        for (x, y, state) in board.iter_cells() {
            let cell_rect = self.get_cell_rect(rect, x, y);
            
            match state {
                CellState::Alive => {
                    painter.rect_filled(cell_rect, 0.0, self.alive_color);
                }
                CellState::Dead => {
                    // Martwe komórki są już wyrenderowane jako tło
                }
            }
        }
        
        // Renderujemy siatkę
        self.render_grid(ui, board, rect);
    }
    
    /// Renderuje siatkę na planszy
    fn render_grid(&self, ui: &mut egui::Ui, board: &Board, rect: Rect) {
        let painter = ui.painter();
        
        // Linie pionowe
        for x in 0..=board.width() {
            let x_pos = rect.min.x + x as f32 * self.cell_size;
            painter.line_segment(
                [Pos2::new(x_pos, rect.min.y), Pos2::new(x_pos, rect.max.y)],
                self.grid_stroke,
            );
        }
        
        // Linie poziome
        for y in 0..=board.height() {
            let y_pos = rect.min.y + y as f32 * self.cell_size;
            painter.line_segment(
                [Pos2::new(rect.min.x, y_pos), Pos2::new(rect.max.x, y_pos)],
                self.grid_stroke,
            );
        }
    }
    
    /// Oblicza prostokąt dla pojedynczej komórki
    fn get_cell_rect(&self, board_rect: Rect, x: usize, y: usize) -> Rect {
        let cell_min = Pos2::new(
            board_rect.min.x + x as f32 * self.cell_size,
            board_rect.min.y + y as f32 * self.cell_size,
        );
        
        Rect::from_min_size(cell_min, Vec2::splat(self.cell_size))
    }
    
    /// Konwertuje pozycję myszy na współrzędne komórki
    pub fn screen_to_cell_coords(
        &self,
        board_rect: Rect,
        screen_pos: Pos2,
    ) -> Option<(usize, usize)> {
        if !board_rect.contains(screen_pos) {
            return None;
        }
        
        let relative_pos = screen_pos - board_rect.min;
        let x = (relative_pos.x / self.cell_size) as usize;
        let y = (relative_pos.y / self.cell_size) as usize;
        
        Some((x, y))
    }
}
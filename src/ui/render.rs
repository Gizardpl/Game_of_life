/// Moduł renderowania planszy gry w życie
/// 
/// Odpowiada za wizualizację stanu gry w oknie aplikacji.
/// Plansza jest renderowana jako kwadrat wyrównany do prawej strony.

use egui::{Color32, Pos2, Rect, Stroke, Vec2};
use crate::logic::board::{Board, CellState};

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
}

impl Default for GameRenderer {
    fn default() -> Self {
        Self {
            cell_size: 10.0,
            alive_color: Color32::BLACK,
            dead_color: Color32::WHITE,
            grid_color: Color32::GRAY,
            grid_stroke: Stroke::new(1.0, Color32::GRAY),
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
    
    /// Renderuje planszę w podanym obszarze
    pub fn render_board(
        &mut self,
        ui: &mut egui::Ui,
        board: &Board,
        available_rect: Rect,
    ) {
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
        
        // Renderujemy planszę
        self.render_board_in_rect(ui, board, final_board_rect);
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
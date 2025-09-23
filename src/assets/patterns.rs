use std::collections::HashMap;
use super::{carpet, pulsar, glider_gun};

/// Reprezentuje pozycję na planszy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Reprezentuje predefiniowaną strukturę/wzór
#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub size: (u32, u32), // (width, height)
    pub center_offset: (i32, i32), // offset od lewego górnego rogu do centrum
    pub cells: Vec<Position>, // pozycje żywych komórek względem lewego górnego rogu
    pub image_path: Option<String>, // ścieżka do obrazka
}

impl Pattern {
    pub fn new(
        name: String,
        description: String,
        size: (u32, u32),
        center_offset: (i32, i32),
        cells: Vec<Position>,
        image_path: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            size,
            center_offset,
            cells,
            image_path,
        }
    }

    /// Zwraca pozycje komórek względem podanego centrum
    pub fn get_cells_at_center(&self, center: Position) -> Vec<Position> {
        let offset_x = center.x - self.center_offset.0;
        let offset_y = center.y - self.center_offset.1;
        
        self.cells
            .iter()
            .map(|pos| Position::new(pos.x + offset_x, pos.y + offset_y))
            .collect()
    }

    /// Zwraca obszar, który zostanie wyczyszczony przed umieszczeniem wzoru
    pub fn get_clear_area(&self, center: Position) -> Vec<Position> {
        let offset_x = center.x - self.center_offset.0;
        let offset_y = center.y - self.center_offset.1;
        
        let mut area = Vec::new();
        for y in 0..self.size.1 as i32 {
            for x in 0..self.size.0 as i32 {
                area.push(Position::new(offset_x + x, offset_y + y));
            }
        }
        area
    }
}

/// Manager predefiniowanych wzorów
pub struct PatternManager {
    patterns: HashMap<String, Pattern>,
}

impl PatternManager {
    pub fn new() -> Self {
        let mut manager = Self {
            patterns: HashMap::new(),
        };
        manager.load_default_patterns();
        manager
    }

    /// Ładuje domyślne wzory
    fn load_default_patterns(&mut self) {
        // Dodaj Carpet
        let carpet = carpet::create_carpet();
        self.patterns.insert("Carpet".to_string(), carpet);
        
        // Dodaj Pulsar
        let pulsar = pulsar::create_pulsar();
        self.patterns.insert("Pulsar".to_string(), pulsar);
        
        // Dodaj Glider Gun
        let glider_gun = glider_gun::create_glider_gun();
        self.patterns.insert("Glider Gun".to_string(), glider_gun);
    }

    pub fn get_pattern(&self, name: &str) -> Option<&Pattern> {
        self.patterns.get(name)
    }

    pub fn get_all_patterns(&self) -> Vec<&Pattern> {
        self.patterns.values().collect()
    }

    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.insert(pattern.name.clone(), pattern);
    }
}

impl Default for PatternManager {
    fn default() -> Self {
        Self::new()
    }
}

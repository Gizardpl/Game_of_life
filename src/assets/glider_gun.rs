use super::patterns::{Pattern, Position};

/// Tworzy wzorzec Glider Gun - struktura produkująca glidery
pub fn create_glider_gun() -> Pattern {
    let glider_gun_cells = vec![
        // Lewy blok (2x2)
        Position::new(0, 4), Position::new(1, 4),
        Position::new(0, 5), Position::new(1, 5),
        
        // Lewy emiter
        Position::new(10, 4), Position::new(10, 5), Position::new(10, 6),
        Position::new(11, 3), Position::new(11, 7),
        Position::new(12, 2), Position::new(12, 8),
        Position::new(13, 2), Position::new(13, 8),
        Position::new(14, 5),
        Position::new(15, 3), Position::new(15, 7),
        Position::new(16, 4), Position::new(16, 5), Position::new(16, 6),
        Position::new(17, 5),
        
        // Środkowy blok
        Position::new(20, 2), Position::new(20, 3), Position::new(20, 4),
        Position::new(21, 2), Position::new(21, 3), Position::new(21, 4),
        Position::new(22, 1), Position::new(22, 5),
        Position::new(24, 0), Position::new(24, 1), Position::new(24, 5), Position::new(24, 6),
        
        // Prawy blok (2x2)
        Position::new(34, 2), Position::new(35, 2),
        Position::new(34, 3), Position::new(35, 3),
    ];

    Pattern::new(
        "Glider Gun".to_string(),
        "Gosper's Glider Gun - pierwsza odkryta struktura produkująca glidery w nieskończoność".to_string(),
        (36, 9), // rozmiar 36x9
        (18, 4), // centrum w środku
        glider_gun_cells,
        None, // brak obrazka na razie
    )
}

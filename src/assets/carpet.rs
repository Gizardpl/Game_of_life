use super::patterns::{Pattern, Position};

/// Tworzy wzorzec Carpet - symetryczna struktura przypominająca dywan
pub fn create_carpet() -> Pattern {
    let carpet_cells = vec![
        // Górne lewe ramię
        Position::new(1, 1),
        Position::new(2, 2),
        Position::new(3, 3),
        
        // Górne prawe ramię  
        Position::new(9, 1),
        Position::new(8, 2),
        Position::new(7, 3),
        
        // Centralny kwadrat 3x3 (pozycje 4,4 do 6,6)
        Position::new(4, 4), Position::new(5, 4), Position::new(6, 4),
        Position::new(4, 5), Position::new(5, 5), Position::new(6, 5),
        Position::new(4, 6), Position::new(5, 6), Position::new(6, 6),
        
        // Dolne lewe ramię
        Position::new(3, 7),
        Position::new(2, 8),
        Position::new(1, 9),
        
        // Dolne prawe ramię
        Position::new(7, 7),
        Position::new(8, 8),
        Position::new(9, 9),
    ];

    Pattern::new(
        "Carpet".to_string(),
        "Symetryczna struktura przypominająca dywan".to_string(),
        (11, 11), // rozmiar 11x11
        (5, 5),   // centrum w środku
        carpet_cells,
        Some("assets/carpet.png".to_string()),
    )
}

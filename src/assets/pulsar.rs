use super::patterns::{Pattern, Position};

/// Tworzy wzorzec Pulsar - oscylator o okresie 3
pub fn create_pulsar() -> Pattern {
    let pulsar_cells = vec![
        // Górna część - lewa strona
        Position::new(2, 0), Position::new(3, 0), Position::new(4, 0),
        Position::new(0, 2),
        Position::new(0, 3),
        Position::new(0, 4),
        Position::new(2, 5), Position::new(3, 5), Position::new(4, 5),
        
        // Górna część - prawa strona
        Position::new(8, 0), Position::new(9, 0), Position::new(10, 0),
        Position::new(12, 2),
        Position::new(12, 3),
        Position::new(12, 4),
        Position::new(8, 5), Position::new(9, 5), Position::new(10, 5),
        
        // Środkowe poziome paski - tylko brzegi, nie wypełnione kwadraty
        Position::new(5, 2), Position::new(7, 2),
        Position::new(5, 3), Position::new(7, 3),
        Position::new(5, 4), Position::new(7, 4),
        
        Position::new(5, 8), Position::new(7, 8),
        Position::new(5, 9), Position::new(7, 9),
        Position::new(5, 10), Position::new(7, 10),
        
        // Dolna część - lewa strona
        Position::new(2, 7), Position::new(3, 7), Position::new(4, 7),
        Position::new(0, 8),
        Position::new(0, 9),
        Position::new(0, 10),
        Position::new(2, 12), Position::new(3, 12), Position::new(4, 12),
        
        // Dolna część - prawa strona
        Position::new(8, 7), Position::new(9, 7), Position::new(10, 7),
        Position::new(12, 8),
        Position::new(12, 9),
        Position::new(12, 10),
        Position::new(8, 12), Position::new(9, 12), Position::new(10, 12),
    ];

    Pattern::new(
        "Pulsar".to_string(),
        "Oscylator o okresie 3 - jedna z najczęstszych struktur oscylujących".to_string(),
        (13, 13), // rozmiar 13x13
        (6, 6),   // centrum w środku
        pulsar_cells,
        None, // brak obrazka na razie
    )
}

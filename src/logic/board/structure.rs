/// Reprezentuje stan pojedynczej komórki w grze w życie
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    /// Komórka jest martwa (pusta)
    Dead,
    /// Komórka jest żywa (aktywna)
    Alive,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Dead
    }
}

/// Współrzędne 2D są mapowane na indeksy 1D za pomocą wzoru: indeks = y * szerokość + x
#[derive(Debug, Clone)]
pub struct Board {
    cells: Vec<CellState>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let total_cells = width * height;
        Self {
            cells: vec![CellState::Dead; total_cells],
            width,
            height,
        }
    }
    
    /// Tworzy nową planszę z wymiarami z konfiguracji
    pub fn new_from_config() -> Self {
        let config = crate::config::get_config();
        Self::new(config.initial_board_size, config.initial_board_size)
    }

    /// Zwraca szerokość planszy
    pub fn width(&self) -> usize {
        self.width
    }

    /// Zwraca wysokość planszy
    pub fn height(&self) -> usize {
        self.height
    }

    /// Zwraca całkowitą liczbę komórek na planszy
    pub fn total_cells(&self) -> usize {
        self.width * self.height
    }

    /// Mapuje współrzędne 2D (x, y) na indeks 1D w tablicy
    fn coords_to_index(&self, x: usize, y: usize) -> Option<usize> {
        // Sprawdzamy czy współrzędne mieszczą się w granicach planszy
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    /// Mapuje indeks 1D na współrzędne 2D (x, y)
    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    /// Pobiera stan komórki na podanych współrzędnych
    pub fn get_cell(&self, x: usize, y: usize) -> Option<CellState> {
        self.coords_to_index(x, y)
            .map(|index| self.cells[index])
    }

    /// Ustawia stan komórki na podanych współrzędnych
    pub fn set_cell(&mut self, x: usize, y: usize, state: CellState) -> bool {
        if let Some(index) = self.coords_to_index(x, y) {
            self.cells[index] = state;
            true
        } else {
            false
        }
    }

    /// Przełącza stan komórki na podanych współrzędnych
    /// Martwa komórka staje się żywa, żywa staje się martwa
    pub fn toggle_cell(&mut self, x: usize, y: usize) -> bool {
        if let Some(current_state) = self.get_cell(x, y) {
            let new_state = match current_state {
                CellState::Dead => CellState::Alive,
                CellState::Alive => CellState::Dead,
            };
            self.set_cell(x, y, new_state)
        } else {
            false
        }
    }

    /// Czyści całą planszę (ustawia wszystkie komórki jako martwe)
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = CellState::Dead;
        }
    }

    /// Sprawdza czy współrzędne mieszczą się w granicach planszy
    pub fn is_valid_coords(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Zwraca iterator po wszystkich komórkach planszy
    /// Iterator zwraca tuple (x, y, state) dla każdej komórki
    pub fn iter_cells(&self) -> impl Iterator<Item = (usize, usize, CellState)> + '_ {
        self.cells.iter().enumerate().map(move |(index, &state)| {
            let (x, y) = self.index_to_coords(index);
            (x, y, state)
        })
    }

    /// Zwraca iterator po żywych komórkach planszy
    /// Iterator zwraca tuple (x, y) dla każdej żywej komórki
    pub fn iter_alive_cells(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.iter_cells()
            .filter(|(_, _, state)| *state == CellState::Alive)
            .map(|(x, y, _)| (x, y))
    }

    /// Zlicza liczbę żywych komórek na planszy
    pub fn count_alive_cells(&self) -> usize {
        self.cells.iter()
            .filter(|&&state| state == CellState::Alive)
            .count()
    }
}
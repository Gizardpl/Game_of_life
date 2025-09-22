mod config;
mod logic;
mod ui;

use config::init_config;

fn main() {
    // Inicjalizujemy konfigurację gry
    init_config();
    
    println!("Conway's Game of Life - Initialized with config");
}

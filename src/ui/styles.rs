/// Moduł stylów dla interfejsu użytkownika
/// 
/// Zawiera definicje kolorów, rozmiarów, marginesów i innych elementów stylistycznych
/// używanych w całej aplikacji.

use egui::{Color32, CornerRadius, Stroke, Vec2, Margin, FontId, FontFamily};

/// Paleta kolorów aplikacji
pub struct ColorPalette {
    // Kolory główne
    pub primary: Color32,
    pub secondary: Color32,
    pub accent: Color32,
    
    // Kolory tła
    pub background_dark: Color32,
    pub background_medium: Color32,
    pub background_light: Color32,
    
    // Kolory tekstu
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,
    pub text_disabled: Color32,  // Nowy kolor dla wyłączonych elementów
    
    // Kolory statusu
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub info: Color32,
    
    // Kolory przycisków
    pub button_start: Color32,
    pub button_stop: Color32,
    pub button_reset: Color32,
    pub button_step: Color32,
    
    // Kolory preview
    pub preview_birth: Color32,
    pub preview_death: Color32,
    
    // Kolory dla efektów
    pub glass_effect: Color32,    // Efekt szkła
    pub border_subtle: Color32,   // Subtelne bordery
    pub hover_overlay: Color32,   // Overlay przy hover
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            // Kolory główne - nowoczesna paleta z gradientami
            primary: Color32::from_rgb(99, 102, 241),       // Indygo
            secondary: Color32::from_rgb(107, 114, 128),     // Szary
            accent: Color32::from_rgb(245, 101, 101),        // Koralowy
            
            // Kolory tła - głębokie ciemne z subtelnymi odcieniami
            background_dark: Color32::from_rgb(17, 24, 39),   // Bardzo ciemny niebieski
            background_medium: Color32::from_rgba_unmultiplied(31, 41, 55, 240), // Półprzezroczysty
            background_light: Color32::from_rgba_unmultiplied(55, 65, 81, 200),  // Jeszcze bardziej przezroczysty
            
            // Kolory tekstu - wysokie kontrasty
            text_primary: Color32::from_rgb(249, 250, 251),
            text_secondary: Color32::from_rgb(209, 213, 219),
            text_muted: Color32::from_rgb(156, 163, 175),
            
            // Kolory statusu - żywe ale eleganckie
            success: Color32::from_rgb(34, 197, 94),        // Żywy zielony
            warning: Color32::from_rgb(251, 191, 36),       // Żółty
            error: Color32::from_rgb(239, 68, 68),          // Czerwony
            info: Color32::from_rgb(59, 130, 246),          // Niebieski
            
            // Kolory przycisków - z gradientowymi efektami
            button_start: Color32::from_rgb(34, 197, 94),   // Zielony
            button_stop: Color32::from_rgb(239, 68, 68),    // Czerwony
            button_reset: Color32::from_rgb(59, 130, 246),  // Niebieski
            button_step: Color32::from_rgb(156, 163, 175),  // Szary
            
            // Kolory preview - z lepszą przezroczystością
            preview_birth: Color32::from_rgba_unmultiplied(34, 197, 94, 160),   // Zielony z przezroczystością
            preview_death: Color32::from_rgba_unmultiplied(239, 68, 68, 160),   // Czerwony z przezroczystością
            
            // Nowe kolory
            text_disabled: Color32::from_rgb(75, 85, 99),    // Szary dla wyłączonych elementów
            
            // Kolory dla efektów
            glass_effect: Color32::from_rgba_unmultiplied(255, 255, 255, 10), // Subtelny efekt szkła
            border_subtle: Color32::from_rgba_unmultiplied(75, 85, 99, 100),  // Subtelne bordery
            hover_overlay: Color32::from_rgba_unmultiplied(99, 102, 241, 20), // Overlay przy hover
        }
    }
}

/// Rozmiary i wymiary elementów UI
pub struct Dimensions {
    // Rozmiary przycisków
    pub button_height: f32,
    pub button_width_small: f32,
    pub button_width_medium: f32,
    pub button_width_large: f32,
    
    // Rozmiary sliderów
    pub slider_height: f32,
    pub slider_width: f32,
    
    // Marginesy i padding
    pub margin_small: f32,
    pub margin_medium: f32,
    pub margin_large: f32,
    
    // Separatory
    pub separator_spacing: f32,
    
    // Rozmiary czcionek
    pub font_size_small: f32,
    pub font_size_medium: f32,
    pub font_size_large: f32,
    pub font_size_heading: f32,
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            // Rozmiary przycisków - większe dla lepszej użyteczności
            button_height: 36.0,
            button_width_small: 70.0,
            button_width_medium: 110.0,
            button_width_large: 160.0,
            
            // Rozmiary sliderów - znacznie wydłużone
            slider_height: 28.0,
            slider_width: 220.0,  // Wydłużony slider
            
            // Marginesy i padding - większe dla lepszego wyglądu
            margin_small: 6.0,
            margin_medium: 12.0,
            margin_large: 20.0,
            
            // Separatory
            separator_spacing: 16.0,
            
            // Rozmiary czcionek
            font_size_small: 12.0,
            font_size_medium: 14.0,
            font_size_large: 16.0,
            font_size_heading: 20.0,
        }
    }
}

/// Style dla różnych elementów UI
pub struct UIStyles {
    pub colors: ColorPalette,
    pub dimensions: Dimensions,
}

impl Default for UIStyles {
    fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
            dimensions: Dimensions::default(),
        }
    }
}

impl UIStyles {
    /// Tworzy nowe style UI
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Zwraca styl dla grupy (sekcji) - nowoczesny z efektem szkła
    pub fn group_style(&self) -> egui::Frame {
        egui::Frame::group(&egui::Style::default())
            .fill(self.colors.background_medium)
            .stroke(Stroke::new(1.0, self.colors.border_subtle))
            .corner_radius(CornerRadius::same(12))
            .inner_margin(Margin::same(self.dimensions.margin_medium as i8))
            .outer_margin(Margin::same(self.dimensions.margin_small as i8))
            .shadow(egui::Shadow {
                offset: [0, 4],
                blur: 12,
                spread: 0,
                color: Color32::from_rgba_unmultiplied(0, 0, 0, 40),
            })
    }
    
    /// Zwraca styl dla zagnieżdżonej grupy (podsekcji) - bez borderu
    pub fn nested_group_style(&self) -> egui::Frame {
        egui::Frame::group(&egui::Style::default())
            .fill(self.colors.background_light)
            .stroke(Stroke::NONE)  // Usunięty border
            .corner_radius(CornerRadius::same(8))
            .inner_margin(Margin::same(self.dimensions.margin_medium as i8))
            .outer_margin(Margin {
                left: self.dimensions.margin_small as i8,
                right: self.dimensions.margin_small as i8,
                top: self.dimensions.margin_small as i8,
                bottom: self.dimensions.margin_small as i8,
            })
    }
    
    /// Zwraca rozmiar przycisku na podstawie typu
    pub fn button_size(&self, button_type: ButtonType) -> Vec2 {
        match button_type {
            ButtonType::Small => Vec2::new(self.dimensions.button_width_small, self.dimensions.button_height),
            ButtonType::Medium => Vec2::new(self.dimensions.button_width_medium, self.dimensions.button_height),
            ButtonType::Large => Vec2::new(self.dimensions.button_width_large, self.dimensions.button_height),
        }
    }
    
    /// Zwraca rozmiar slidera
    pub fn slider_size(&self) -> Vec2 {
        Vec2::new(self.dimensions.slider_width, self.dimensions.slider_height)
    }
    
    /// Zwraca FontId dla danego typu tekstu
    pub fn font_id(&self, text_type: TextType) -> FontId {
        let size = match text_type {
            TextType::Small => self.dimensions.font_size_small,
            TextType::Medium => self.dimensions.font_size_medium,
            TextType::Large => self.dimensions.font_size_large,
            TextType::Heading => self.dimensions.font_size_heading,
        };
        FontId::new(size, FontFamily::Proportional)
    }
    
    /// Zwraca spacing dla separatorów
    pub fn separator_spacing(&self) -> f32 {
        self.dimensions.separator_spacing
    }
}

/// Typy przycisków
#[derive(Debug, Clone, Copy)]
pub enum ButtonType {
    Small,
    Medium,
    Large,
}

/// Typy tekstu
#[derive(Debug, Clone, Copy)]
pub enum TextType {
    Small,
    Medium,
    Large,
    Heading,
}

/// Funkcje pomocnicze do tworzenia stylizowanych elementów UI
pub mod helpers {
    use super::*;
    use egui::{Button, RichText, Slider};
    
    /// Tworzy stylizowany przycisk
    pub fn styled_button<'a>(text: &'a str, color: Color32, styles: &UIStyles, button_type: ButtonType) -> Button<'a> {
        Button::new(
            RichText::new(text)
                .color(color)
                .font(styles.font_id(TextType::Medium))
                .strong()
        ).min_size(styles.button_size(button_type))
    }
    
    /// Tworzy stylizowany slider
    pub fn styled_slider<'a>(value: &'a mut f32, range: std::ops::RangeInclusive<f32>, text: &str, _styles: &UIStyles) -> Slider<'a> {
        Slider::new(value, range)
            .text(text)
            .min_decimals(1)
            .max_decimals(1)
    }
    
    /// Tworzy stylizowany nagłówek sekcji
    pub fn section_header(text: &str, styles: &UIStyles) -> RichText {
        RichText::new(text)
            .font(styles.font_id(TextType::Large))
            .color(styles.colors.text_primary)
            .strong()
    }
    
    /// Tworzy stylizowany nagłówek podsekcji
    pub fn subsection_header(text: &str, styles: &UIStyles) -> RichText {
        RichText::new(text)
            .font(styles.font_id(TextType::Medium))
            .color(styles.colors.text_secondary)
            .strong()
    }
    
    /// Tworzy stylizowany tekst etykiety
    pub fn label_text(text: &str, styles: &UIStyles) -> RichText {
        RichText::new(text)
            .font(styles.font_id(TextType::Medium))
            .color(styles.colors.text_secondary)
    }
    
    /// Tworzy stylizowany tekst wartości (monospace)
    pub fn value_text(text: &str, styles: &UIStyles) -> RichText {
        RichText::new(text)
            .font(FontId::new(styles.dimensions.font_size_medium, FontFamily::Monospace))
            .color(styles.colors.text_primary)
    }
    
    /// Tworzy wyłączony (szary) tekst
    pub fn disabled_text(text: &str, styles: &UIStyles) -> RichText {
        RichText::new(text)
            .font(styles.font_id(TextType::Medium))
            .color(styles.colors.text_disabled)
    }
    
    /// Tworzy przycisk strzałki dla sliderów
    pub fn arrow_button<'a>(text: &'a str, enabled: bool, styles: &UIStyles) -> Button<'a> {
        let color = if enabled { styles.colors.text_primary } else { styles.colors.text_disabled };
        Button::new(
            RichText::new(text)
                .color(color)
                .font(styles.font_id(TextType::Medium))
                .strong()
        )
        .min_size(Vec2::new(32.0, styles.dimensions.button_height))
    }
    
    /// Tworzy stylizowany slider z większą szerokością
    pub fn wide_slider<'a>(value: &'a mut f32, range: std::ops::RangeInclusive<f32>, text: &str, _styles: &UIStyles) -> Slider<'a> {
        Slider::new(value, range)
            .text(text)
            .min_decimals(1)
            .max_decimals(1)
    }
    
    /// Tworzy stylizowany checkbox
    pub fn styled_checkbox(ui: &mut egui::Ui, checked: &mut bool, text: &str, styles: &UIStyles) -> egui::Response {
        ui.checkbox(checked, RichText::new(text)
            .font(styles.font_id(TextType::Medium))
            .color(styles.colors.text_secondary))
    }
}

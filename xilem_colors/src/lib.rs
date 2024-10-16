pub(crate) mod apca;
pub(crate) mod scales;
pub mod tokens;
pub mod utils;

use scales::Scales;
use tokens::{ThemeColor, ColorTokens, Token};
use utils::THEMES;
//use interpoli::

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    // pub bg: TokenColor,
    // pub hov_bg: TokenColor,
    pub border_width: f64,
    pub gradient: bool,
    pub color_on_select: bool,
    pub border: Token,
    pub hov_border: Token,
    pub bg_grad: [Token; 2],
    pub hov_bg_grad: [Token; 2],
    pub active_bg_grad: [Token; 2],
}

impl std::default::Default for Style {
    fn default() -> Self {
        Self { 
            // bg: TokenColor::AppBackground, 
            // hov_bg: TokenColor::SubtleBackground,
            border_width: 1.,
            gradient: true, 
            color_on_select: false,
            border: Token::SubtleBordersAndSeparators, 
            hov_border: Token::HoveredUiElementBorder, 
            bg_grad: [Token::AppBackground, Token::SubtleBackground], 
            hov_bg_grad: [Token::SubtleBackground, Token::UiElementBackground],
            active_bg_grad: [Token::AppBackground, Token::SolidBackgrounds],}
    }
}

#[derive(Debug, Default, Clone)]
pub struct Colorix {
    pub tokens: ColorTokens,
    pub(crate) theme: [ThemeColor; 12],
    theme_index: usize,
    pub(crate) scales: Scales,
    pub dark_mode: bool,
}

impl Colorix {
    pub fn init() -> Self {
        let theme = THEMES[6];
        let mut colorix = Self {
            dark_mode: true,
            theme,
            ..Default::default()
        };

        colorix.get_theme_index();
        colorix.update_colors();
        colorix
    }

    // fn set_theme(&mut self, theme: [ColorPreset; 12]) {
    //     self.theme = theme
    // }
    pub fn pick_theme(&mut self, i: usize) {
        self.theme = THEMES[i];
        self.update_colors();
    }

    pub fn invert_mode(&mut self) {
        self.dark_mode = !self.dark_mode;
        self.update_colors();
    }

    fn get_theme_index(&mut self) {
        if let Some(i) = THEMES.iter().position(|t| t == &self.theme) {
            self.theme_index = i;
        };
    }

    fn process_theme(&mut self) {
        let mut processed: Vec<usize> = vec![];
        for (i, v) in self.theme.iter().enumerate() {
            if !processed.contains(&i) {
                self.scales.process_color(*v);
                self.tokens.update_schema(i, self.scales.scale[i]);
                if i < self.theme.len() {
                    for (j, w) in self.theme[i + 1..].iter().enumerate() {
                        if w == v {
                            self.tokens
                                .update_schema(j + i + 1, self.scales.scale[j + i + 1]);
                            processed.push(j + i + 1);
                        }
                    }
                }
            }
        }
        self.tokens.color_on_accent()
    }

    fn _update_color(&mut self, i: usize) {
        self.scales.process_color(self.theme[i]);
        self.tokens.update_schema(i, self.scales.scale[i]);
        //self.tokens.set_text_color();
    }

    fn update_colors(&mut self) {
        self.scales.dark_mode = self.dark_mode;
        self.process_theme();
        //self.tokens.color_on_accent();
    }
}
pub(crate) mod apca;
pub(crate) mod scales;
pub mod tokens;
pub mod utils;

use scales::Scales;
use tokens::{ColorPreset, ColorTokens};
use utils::THEMES;


#[derive(Debug, Default, Clone)]
pub struct Colorix {
    pub tokens: ColorTokens,
    pub(crate) theme: [ColorPreset; 12],
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
    }

    fn _update_color(&mut self, i: usize) {
        self.scales.process_color(self.theme[i]);
        self.tokens.update_schema(i, self.scales.scale[i]);
        //self.tokens.set_text_color();
    }

    fn update_colors(&mut self) {
        self.scales.dark_mode = self.dark_mode;
        self.process_theme();
        self.tokens.set_text_color();
    }


}

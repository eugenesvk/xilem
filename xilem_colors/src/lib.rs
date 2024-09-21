

pub(crate) mod apca;
pub(crate) mod scales;
pub mod tokens;
/// Some predefined themes
pub mod utils;

use scales::Scales;
use tokens::{ColorPreset, ColorTokens};
use utils::{DROPDOWN_TEXT, LABELS, THEMES, THEME_NAMES};

#[derive(Debug, Default, Clone)]
pub struct Colorix {
    pub tokens: ColorTokens,
    //pub tokens2: ColorTokens,
    pub(crate) theme: [ColorPreset; 12],
    theme_index: usize,
    pub(crate) scales: Scales,
}

impl Colorix {
    pub fn init(theme: [ColorPreset; 12]) -> Self {
        let mut colorix = Self {
            theme,
            ..Default::default()
        };

        colorix.get_theme_index();
        colorix.update_colors();
        colorix
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

    // pub fn process_2nd_theme(&mut self, theme: &[ColorPreset; 12]) {
    //     let mut processed: Vec<usize> = vec![];
    //     for (i, v) in theme.iter().enumerate() {
    //         if !processed.contains(&i) {
    //             self.scales.process_color(v);
    //             self.tokens2.update_scheme(i, self.scales.scale[i]);
    //             if i < self.theme.len() {
    //                 for (j, w) in self.theme[i + 1..].iter().enumerate() {
    //                     if w == v {
    //                         self.tokens2.update_scheme(j + i + 1, self.scales.scale[j + i + 1]);
    //                         processed.push(j + i + 1)
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     self.tokens.text_color();
    // }

    fn update_color(&mut self, i: usize) {
        self.scales.process_color(self.theme[i]);
        self.tokens.update_schema(i, self.scales.scale[i]);
        //self.tokens.set_text_color();
    }

    fn update_colors(&mut self) {
        self.process_theme();
        //self.tokens.set_text_color();
    }


}

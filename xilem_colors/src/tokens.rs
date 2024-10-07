use crate::apca::estimate_lc;
use palette::{LinSrgb, Okhsl, Srgb, FromColor, IntoColor};
use peniko::Color;


#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenColor {
    #[default]
    AppBackground,
    SubtleBackground,
    UiElementBackground,
    HoveredUiElementBackground,
    ActiveUiElementBackground,
    SubtleBordersAndSeparators,
    UiElementBorderAndFocusRings,
    HoveredUiElementBorder,
    SolidBackgrounds,
    HoveredSolidBackgrounds,
    LowContrastText,
    HighContrastText,
    AccentText,
    Transparent,
    Custom(Color),
}

/// The functional UI elements mapped to a scale
#[derive(Default, Debug, Clone, Copy)]
pub struct ColorTokens {
    pub app_background: Color,
    pub subtle_background: Color,
    pub ui_element_background: Color,
    pub hovered_ui_element_background: Color,
    pub active_ui_element_background: Color,
    pub subtle_borders_and_separators: Color,
    pub ui_element_border_and_focus_rings: Color,
    pub hovered_ui_element_border: Color,
    pub solid_backgrounds: Color,
    pub hovered_solid_backgrounds: Color,
    pub low_contrast_text: Color,
    pub high_contrast_text: Color,
    pub(crate) inverse_color: bool,
    pub(crate) text_color: Color,
}

impl ColorTokens {
    pub(crate) fn set_text_color(&mut self) {
        let white = Srgb::<u8>::from_components((255, 255, 255));
        let bg = self.solid_backgrounds;
        let bg_cl = Srgb::<u8>::from_components((bg.r, bg.g, bg.b));
  
        let lc = estimate_lc(white, bg_cl);
        if lc > -46. {
            self.inverse_color = true;
            let s = self.solid_backgrounds;
            let inverse = LinSrgb::from_components((s.r, s.g, s.b)).into_format();
            let mut okhsl = Okhsl::from_color(inverse);

            okhsl.lightness = 0.01;
            okhsl.saturation = 0.7;
            let (r, g, b) = Srgb::from_linear(okhsl.into_color()).into();
            self.text_color = Color::rgb8(r, g, b);
        } else {
            self.text_color = Color::WHITE;
        }
    }

    /// the color for the text on accented color (solid backgrounds)
    #[must_use]
    pub const fn text_color(&self) -> Color {
        self.text_color
    }

    /// notifies when lc > -46.
    #[must_use]
    pub const fn inverse_color(&self) -> bool {
        self.inverse_color
    }

    pub(crate) fn update_schema(&mut self, i: usize, fill: Color) {
        match i {
            0 => self.app_background = fill,
            1 => self.subtle_background = fill,
            2 => self.ui_element_background = fill,
            3 => self.hovered_ui_element_background = fill,
            4 => self.active_ui_element_background = fill,
            5 => self.subtle_borders_and_separators = fill,
            6 => self.ui_element_border_and_focus_rings = fill,
            7 => self.hovered_ui_element_border = fill,
            8 => self.solid_backgrounds = fill,
            9 => self.hovered_solid_backgrounds = fill,
            10 => self.low_contrast_text = fill,
            11 => self.high_contrast_text = fill,
            _ => {}
        }
    }

    pub fn set_token(&self, token: TokenColor) -> Color {
        match token {
            TokenColor::AppBackground => self.app_background,
            TokenColor::SubtleBackground => self.subtle_background,
            TokenColor::UiElementBackground => self.ui_element_background,
            TokenColor::HoveredUiElementBackground => self.hovered_ui_element_background,
            TokenColor::ActiveUiElementBackground => self.active_ui_element_background,
            TokenColor::SubtleBordersAndSeparators => self.subtle_borders_and_separators,
            TokenColor::UiElementBorderAndFocusRings => self.ui_element_border_and_focus_rings,
            TokenColor::HoveredUiElementBorder => self.hovered_ui_element_border,
            TokenColor::SolidBackgrounds => self.solid_backgrounds,
            TokenColor::HoveredSolidBackgrounds => self.hovered_solid_backgrounds,
            TokenColor::LowContrastText => self.low_contrast_text,
            TokenColor::HighContrastText => self.high_contrast_text,
            TokenColor::AccentText => self.text_color(),
            TokenColor::Custom(color) => color,
            TokenColor::Transparent => Color::TRANSPARENT,         
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorPreset {
    #[default]
    Gray,
    EguiBlue,
    Tomato,
    Red,
    Ruby,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    Indigo,
    Blue,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Brown,
    Bronze,
    Gold,
    Orange,
    Custom([u8; 3]),
}

impl ColorPreset {
    pub(crate) fn get_srgb(self) -> LinSrgb<f32> {
        match self {
            Self::Gray => Srgb::new(117, 117, 117).into_linear(),
            Self::EguiBlue => Srgb::new(0, 109, 143).into_linear(),
            Self::Tomato => Srgb::new(229, 77, 46).into_linear(),
            Self::Red => Srgb::new(229, 72, 77).into_linear(),
            Self::Ruby => Srgb::new(229, 70, 102).into_linear(),
            Self::Crimson => Srgb::new(233, 61, 130).into_linear(),
            Self::Pink => Srgb::new(214, 64, 159).into_linear(),
            Self::Plum => Srgb::new(171, 74, 186).into_linear(),
            Self::Purple => Srgb::new(142, 78, 198).into_linear(),
            Self::Violet => Srgb::new(110, 86, 207).into_linear(),
            Self::Iris => Srgb::new(91, 91, 214).into_linear(),
            Self::Indigo => Srgb::new(62, 99, 214).into_linear(),
            Self::Blue => Srgb::new(0, 144, 255).into_linear(),
            Self::Cyan => Srgb::new(0, 162, 199).into_linear(),
            Self::Teal => Srgb::new(18, 165, 148).into_linear(),
            Self::Jade => Srgb::new(41, 163, 131).into_linear(),
            Self::Green => Srgb::new(48, 164, 108).into_linear(),
            Self::Grass => Srgb::new(70, 167, 88).into_linear(),
            Self::Brown => Srgb::new(173, 127, 88).into_linear(),
            Self::Bronze => Srgb::new(161, 128, 114).into_linear(),
            Self::Gold => Srgb::new(151, 131, 101).into_linear(),
            Self::Orange => Srgb::new(247, 107, 21).into_linear(),
            Self::Custom(v) => Srgb::new(v[0], v[1], v[2]).into_linear(),
        }
    }
}

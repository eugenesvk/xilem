use crate::tokens::ColorPreset;

pub const EGUI_THEME: [ColorPreset; 12] = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::EguiBlue,
    ColorPreset::EguiBlue,
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub const GRAYS: [ColorPreset; 12] = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub const INDIGO_JADE: [ColorPreset; 12] = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Indigo,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Jade,
    ColorPreset::Jade,
    ColorPreset::Jade,
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub const GRASS_BRONZE: [ColorPreset; 12] = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Grass,
    ColorPreset::Bronze,
    ColorPreset::Bronze,
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Green,
    ColorPreset::Bronze,
    ColorPreset::Bronze,
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub const WARM: [ColorPreset; 12] = [
    ColorPreset::Gray,
    ColorPreset::Gray,
    ColorPreset::Orange,
    ColorPreset::Gold,
    ColorPreset::Gold,
    ColorPreset::Gold,
    ColorPreset::Red,
    ColorPreset::Red,
    ColorPreset::Gold,
    ColorPreset::Gold,
    ColorPreset::Gray,
    ColorPreset::Teal,
];

pub const COOL: [ColorPreset; 12] = [
    ColorPreset::Gray,
    ColorPreset::Indigo,
    ColorPreset::Indigo,
    ColorPreset::Cyan,
    ColorPreset::Indigo,
    ColorPreset::Gray,
    ColorPreset::Cyan,
    ColorPreset::Indigo,
    ColorPreset::Blue,
    ColorPreset::Indigo,
    ColorPreset::Orange,
    ColorPreset::Gray,
];

pub const SEVENTIES: [ColorPreset; 12] = [
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Pink,
    ColorPreset::Pink,
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Custom([254, 180, 0]),
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Custom([95, 78, 163]),
    ColorPreset::Custom([254, 180, 0]),
    ColorPreset::Custom([254, 180, 0]),
    ColorPreset::Gray,
    ColorPreset::Gray,
];

pub(crate) const THEMES: [[ColorPreset; 12]; 6] =
    [EGUI_THEME, INDIGO_JADE, GRASS_BRONZE, WARM, COOL, SEVENTIES];

pub(crate) const DROPDOWN_TEXT: [&str; 23] = [
    "Gray", "EguiBlue", "Tomato", "Red", "Ruby", "Crimson", "Pink", "Plum", "Purple", "Violet",
    "Iris", "Indigo", "Blue", "Cyan", "Teal", "Jade", "Green", "Grass", "Brown", "Bronze", "Gold",
    "Orange", "Custom",
];

pub(crate) const LABELS: [&str; 12] = [
    "app background",
    "subtle background",
    "ui element background",
    "hovered ui element background",
    "active ui element background",
    "subtle borders and separators",
    "ui element border and focus rings",
    "hovered ui element border",
    "solid backgrounds",
    "hovered solid backgrounds",
    "low contrast text",
    "high contrast text",
];

pub(crate) const THEME_NAMES: [&str; 6] = [
    "Egui",
    "Indigo/jade",
    "Grass/bronze",
    "Warm",
    "Cool",
    "Seventies",
];

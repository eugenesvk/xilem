use crate::tokens::ThemeColor;

pub const EGUI_THEME: [ThemeColor; 12] = [
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::EguiBlue,
    ThemeColor::EguiBlue,
    ThemeColor::Gray,
    ThemeColor::Gray,
];

pub const OFFICE_GRAY: [ThemeColor; 12] = [
    ThemeColor::Custom([140, 149, 138]),
    ThemeColor::Custom([140, 149, 138]),
    ThemeColor::Custom([140, 149, 138]),
    ThemeColor::Custom([122, 166, 168]),
    ThemeColor::Gray,
    ThemeColor::Custom([122, 166, 168]),
    ThemeColor::Custom([122, 166, 168]),
    ThemeColor::Custom([122, 166, 168]),
    ThemeColor::Custom([59, 71, 97]),
    ThemeColor::Custom([59, 71, 97]),
    ThemeColor::Custom([185, 178, 168]),
    ThemeColor::Custom([185, 178, 168]),
];

pub const INDIGO_JADE: [ThemeColor; 12] = [
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Indigo,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Jade,
    ThemeColor::Jade,
    ThemeColor::Jade,
    ThemeColor::Gray,
    ThemeColor::Gray,
];

pub const GRASS_BRONZE: [ThemeColor; 12] = [
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Grass,
    ThemeColor::Bronze,
    ThemeColor::Bronze,
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Green,
    ThemeColor::Bronze,
    ThemeColor::Bronze,
    ThemeColor::Gray,
    ThemeColor::Gray,
];

pub const WARM: [ThemeColor; 12] = [
    ThemeColor::Gray,
    ThemeColor::Gray,
    ThemeColor::Orange,
    ThemeColor::Gold,
    ThemeColor::Gold,
    ThemeColor::Gold,
    ThemeColor::Red,
    ThemeColor::Red,
    ThemeColor::Gold,
    ThemeColor::Gold,
    ThemeColor::Gray,
    ThemeColor::Teal,
];

pub const COOL: [ThemeColor; 12] = [
    ThemeColor::Gray,
    ThemeColor::Indigo,
    ThemeColor::Indigo,
    ThemeColor::Iris,
    ThemeColor::Indigo,
    ThemeColor::Gray,
    ThemeColor::Iris,
    ThemeColor::Indigo,
    ThemeColor::Blue,
    ThemeColor::Indigo,
    ThemeColor::Orange,
    ThemeColor::Gray,
];

pub const SEVENTIES: [ThemeColor; 12] = [
    ThemeColor::Custom([95, 78, 163]),
    ThemeColor::Pink,
    ThemeColor::Pink,
    ThemeColor::Custom([95, 78, 163]),
    ThemeColor::Custom([95, 78, 163]),
    ThemeColor::Custom([254, 180, 0]),
    ThemeColor::Custom([95, 78, 163]),
    ThemeColor::Custom([95, 78, 163]),
    ThemeColor::Custom([254, 180, 0]),
    ThemeColor::Custom([254, 180, 0]),
    ThemeColor::Gray,
    ThemeColor::Gray,
];

pub(crate) const THEMES: [[ThemeColor; 12]; 7] =
    [EGUI_THEME, INDIGO_JADE, GRASS_BRONZE, WARM, COOL, SEVENTIES, OFFICE_GRAY];

// pub(crate) const DROPDOWN_TEXT: [&str; 23] = [
//     "Gray", "EguiBlue", "Tomato", "Red", "Ruby", "Crimson", "Pink", "Plum", "Purple", "Violet",
//     "Iris", "Indigo", "Blue", "Cyan", "Teal", "Jade", "Green", "Grass", "Brown", "Bronze", "Gold",
//     "Orange", "Custom",
// ];

// pub(crate) const LABELS: [&str; 12] = [
//     "app background",
//     "subtle background",
//     "ui element background",
//     "hovered ui element background",
//     "active ui element background",
//     "subtle borders and separators",
//     "ui element border and focus rings",
//     "hovered ui element border",
//     "solid backgrounds",
//     "hovered solid backgrounds",
//     "low contrast text",
//     "high contrast text",
// ];

// pub(crate) const THEME_NAMES: [&str; 6] = [
//     "Egui",
//     "Indigo/jade",
//     "Grass/bronze",
//     "Warm",
//     "Cool",
//     "Seventies",
// ];

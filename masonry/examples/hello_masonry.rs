// Copyright 2019 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! This is a very small example of how to setup a masonry application.
//! It does the almost bare minimum while still being useful.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use masonry::app_driver::{AppDriver, DriverCtx};
use masonry::dpi::LogicalSize;
use masonry::widget::{Button, Flex, Label, DarkLightSwitch, RootWidget, SizedBox};
use masonry::{Action, Color, WidgetId};
use winit::window::Window;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

struct Driver {
    dark_mode: bool,
}

impl AppDriver for Driver {
    fn on_action(&mut self, _ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed(_) => {
                println!("Hello");
            }
            Action::ModeSwitched(_button, dark_mode) => {
                self.dark_mode = !dark_mode;
                println!("Dark mode is always{}", dark_mode)
            }
            action => {
                eprintln!("Unexpected action {action:?}");
            }
        }
    }
}

pub fn main() {
    let label = Label::new("Hello").with_text_size(32.0).set_token(xilem_colors::tokens::Token::Custom(Color::BLUE_VIOLET));
    let button = Button::new("Say hello");
    let switch = DarkLightSwitch::new();

    // Arrange the two widgets vertically, with some padding
    let main_widget = Flex::column()
    .with_child(label)
    .with_spacer(VERTICAL_WIDGET_SPACING)
    .with_child(button)
    .with_child(switch)
    .with_spacer(100.);

    let main_main_widget = SizedBox::new(main_widget); 

    let window_size = LogicalSize::new(400.0, 400.0);
    let window_attributes = Window::default_attributes()
        .with_title("Hello World!")
        .with_resizable(true)
        .with_max_inner_size(window_size);

    let driver = Driver {dark_mode: true};

    masonry::event_loop_runner::run(
        masonry::event_loop_runner::EventLoop::with_user_event(),
        window_attributes,
        RootWidget::new(main_main_widget),
        driver,
    )
    .unwrap();
}

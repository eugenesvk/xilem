// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

//! This is a very small example to demonstrate tab focus.
//! It will probably be removed in the future.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use masonry::app::{AppDriver, DriverCtx};
use masonry::core::{Action, WidgetId};
use masonry::dpi::LogicalSize;
use masonry::widgets::{Flex, RootWidget, Textbox};
use winit::window::Window;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

struct Driver;

impl AppDriver for Driver {
    fn on_action(&mut self, _ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, _action: Action) {}
}

fn main() {
    let main_widget = Flex::column()
        .gap(0.0)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Textbox::new(""))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(Textbox::new(""));

    let window_size = LogicalSize::new(400.0, 400.0);
    let window_attributes = Window::default_attributes()
        .with_title("Two textboxes")
        .with_resizable(true)
        .with_min_inner_size(window_size);

    masonry::app::run(
        masonry::app::EventLoop::with_user_event(),
        window_attributes,
        RootWidget::new(main_widget),
        Driver,
    )
    .unwrap();
}

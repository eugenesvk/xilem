use std::borrow::BorrowMut;

use accesskit::{DefaultActionVerb, NodeBuilder, Role, Toggled};
use smallvec::{smallvec, SmallVec};
use tracing::{trace, trace_span, Span};
use vello::kurbo::{Insets, Size};
use vello::Scene;
use xilem_colors::tokens::TokenColor;

use crate::action::Action;
use crate::widget::WidgetMut;

use crate::{
    theme, AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, PointerButton, PointerEvent, RegisterCtx, StatusChange, TextEvent, Widget, WidgetId, WidgetPod, ArcStr
};

use super::{Button, Label};

/// A button that switches between Dark and Light mode.
pub struct LightDarkSwitch {
    button: WidgetPod<Button>,
    dark_mode: bool,
}

impl LightDarkSwitch {
    /// Create a new `Switch`.
    pub fn new() -> LightDarkSwitch {
        LightDarkSwitch {
            button: WidgetPod::new(Button::from_label(Label::new("Switch to LIGHT mode").with_skip_pointer(true)).selectable()),
            dark_mode: true
        }
    }
}

// --- MARK: WIDGETMUT ---
impl WidgetMut<'_, LightDarkSwitch> {
    pub fn switch_mode(&mut self, dark_mode: bool) {
        self.widget.dark_mode = dark_mode;
        if self.widget.dark_mode {
            self.set_text("Switch to LIGHT mode".into());
        }
        else {
            self.set_text("Switch to DARK mode".into());
        }
        self.ctx.invert_mode();
        self.ctx.request_paint();
        self.ctx.request_accessibility_update();
    }

    pub fn set_text(&mut self, new_text: ArcStr){
        let mut label = self.ctx.get_mut(&mut self.widget.button);
        label.set_text(new_text);
    }
}

// --- MARK: IMPL WIDGET ---
impl Widget for LightDarkSwitch {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(_, _) => {
                if !ctx.is_disabled() {
                    //ctx.capture_pointer();
                    ctx.request_paint();
                    trace!("LightDarkSwitch {:?} pressed", ctx.widget_id());
                }
            }
            PointerEvent::PointerUp(_, _) => {
                if ctx.hovered() && !ctx.is_disabled() {
                    self.dark_mode = !self.dark_mode;
                    dbg!(self.dark_mode);
                    let dark = self.dark_mode;
                    ctx.mutate_later(&mut self.button, move|mut button| {
                        if dark {
                            button.selected();
                            button.set_text("Switch to LIGHT mode");
                        }
                        else {
                            button.unselected();
                            button.set_text("Switch to DARK mode");
                        }
                    });
                    ctx.invert_mode();
                    ctx.request_paint();
                    ctx.submit_action(Action::CheckboxChecked(self.dark_mode));
                    ctx.request_accessibility_update();
                    trace!("LightDarkSwitch {:?} released", ctx.widget_id());
                }
            }
            _ => (),
        }
        if let Some(selection) = self.button.take_inner() {
            dbg!(selection.selected);
        }
    }

    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {}

    fn on_access_event(&mut self, ctx: &mut EventCtx, event: &AccessEvent) {
        if event.target == ctx.widget_id() {
            match event.action {
                accesskit::Action::Default => {
                    ctx.submit_action(Action::ButtonPressed(PointerButton::Primary));
                    ctx.request_paint();
                }
                _ => {}
            }
        }
    }

    fn on_status_change(&mut self, ctx: &mut LifeCycleCtx, _event: &StatusChange) {
        ctx.request_paint();
    }

    fn register_children(&mut self, ctx: &mut RegisterCtx) {
        ctx.register_child(&mut self.button);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {

        let x_padding = theme::WIDGET_CONTROL_COMPONENT_PADDING;
        let check_size = theme::BASIC_WIDGET_HEIGHT;
        let size = ctx.run_layout(&mut self.button, bc);
        ctx.place_child(&mut self.button, (check_size + x_padding, 0.0).into());
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        
        // if self.dark_mode {
        //     ctx.mutate(&mut self.button, |mut button| {
        //         button.set_text("Switch to LIGHT mode");
        //     });
        // }
        // else {ctx.mutate(&mut self.button, |mut button| {
        //     button.set_text("Switch to DARK mode");
        // });}
    }

    fn accessibility_role(&self) -> Role {
        Role::CheckBox
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx, node: &mut NodeBuilder) {
        // IMPORTANT: We don't want to merge this code in practice, because
        // the child label already has a 'name' property.
        // This is more of a proof of concept of `get_raw_ref()`.
        if false {
            let button = ctx.get_raw_ref(&self.button);
            let name = button.widget().short_type_name();
            node.set_name(name);
        }
        if self.dark_mode {
            node.set_toggled(Toggled::True);
            node.set_default_action_verb(DefaultActionVerb::Uncheck);
        } else {
            node.set_toggled(Toggled::False);
            node.set_default_action_verb(DefaultActionVerb::Check);
        }
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.button.id()]
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("LightDarkSwitch")
    }

    fn get_debug_text(&self) -> Option<String> {
        if self.dark_mode {
            Some("[X]".to_string())
        } else {
            Some("[ ]".to_string())
        }
    }
}
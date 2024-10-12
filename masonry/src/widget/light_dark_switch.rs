

use accesskit::{DefaultActionVerb, NodeBuilder, Role, Toggled};
use smallvec::{smallvec, SmallVec};
use tracing::{trace, trace_span, Span};
use vello::kurbo::Size;
use vello::Scene;
use xilem_colors::ColorStyle;
use crate::action::Action;
use crate::widget::WidgetMut;
use rand::{thread_rng, Rng};

use crate::{
    theme, AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, PointerButton, PointerEvent, RegisterCtx, StatusChange, TextEvent, Widget, WidgetId, WidgetPod, ArcStr
};

use super::{Button, Label};

/// A button that switches between Dark and Light mode.
pub struct LightDarkSwitch {
    button: WidgetPod<Button>,
    dark_mode: bool,
    style: ColorStyle,
}

impl LightDarkSwitch {
    /// Create a new `Switch`.
    pub fn new() -> LightDarkSwitch {
        LightDarkSwitch {
            button: WidgetPod::new(Button::from_label(Label::new("Switch to LIGHT mode").with_skip_pointer(true))),
            dark_mode: true,
            style: ColorStyle::default(),
        }
    }
    pub fn set_style(&mut self, new_style: ColorStyle) {
        self.style = new_style
    }
}

// --- MARK: WIDGETMUT ---
impl WidgetMut<'_, LightDarkSwitch> {
    pub fn switch_mode(&mut self, dark_mode: bool) {
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0..7);
        self.widget.dark_mode = dark_mode;
        if self.widget.dark_mode {
            self.set_text("Switch to LIGHT mode".into());
        }
        else {
            self.set_text("Switch to DARK mode".into());
        };
        self.ctx.switch_theme(n);
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
                ctx.capture_pointer();
                ctx.release_pointer();
            }
            PointerEvent::PointerUp(_, _) => {
                dbg!(ctx.widget_id());
                if ctx.hovered() && !ctx.is_disabled() {
                    ctx.submit_action(Action::CheckboxChecked(self.dark_mode));
                    //ctx.submit_action(Action::ModeSwitched(PointerButton::None, self.dark_mode));
                    ctx.request_accessibility_update();
                    trace!("light_dark_switch {:?} released", ctx.widget_id());
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
                    ctx.submit_action(Action::ModeSwitched(PointerButton::None, self.dark_mode));
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

    fn paint(&mut self, _ctx: &mut PaintCtx, _scene: &mut Scene) {
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
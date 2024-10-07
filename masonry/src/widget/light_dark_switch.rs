use accesskit::{DefaultActionVerb, NodeBuilder, Role, Toggled};
use smallvec::{smallvec, SmallVec};
use tracing::{trace, trace_span, Span};
use vello::kurbo::Size;
use vello::Scene;

use crate::action::Action;
use crate::widget::WidgetMut;

use crate::{
    theme, AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, PointerButton, PointerEvent, RegisterCtx, StatusChange, TextEvent, Widget, WidgetId, WidgetPod, ArcStr
};

use super::Label;

/// A button that switches between Dark and Light mode.
pub struct LightDarkSwitch {
    label: WidgetPod<Label>,
    dark_mode: bool,
}

impl LightDarkSwitch {
    /// Create a new `Switch`.
    pub fn new() -> LightDarkSwitch {
        LightDarkSwitch {
            label: WidgetPod::new(Label::new("Switch to DARK mode").with_skip_pointer(true)),
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
        /// Set the text.
    ///
    /// We enforce this to be an `ArcStr` to make the allocation explicit.
    pub fn set_text(&mut self, new_text: ArcStr) {
        self.label_mut().set_text(new_text);
    }
    pub fn label_mut(&mut self) -> WidgetMut<'_, Label> {
        self.ctx.get_mut(&mut self.widget.label)
    }

}

// --- MARK: IMPL WIDGET ---
impl Widget for LightDarkSwitch {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(_, _) => {
                if !ctx.is_disabled() {
                    ctx.capture_pointer();
                    //ctx.request_paint();
                    trace!("Checkbox {:?} pressed", ctx.widget_id());
                }
            }
            PointerEvent::PointerUp(_, _) => {
                if ctx.has_pointer_capture() && ctx.hovered() && !ctx.is_disabled() {
                    self.dark_mode = !self.dark_mode;
                    //self.checked = !self.checked;
                    ctx.submit_action(Action::CheckboxChecked(self.dark_mode));
                    ctx.request_accessibility_update();
                    trace!("Checkbox {:?} released", ctx.widget_id());
                }
                ctx.request_paint();
            }
            _ => (),
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
        ctx.register_child(&mut self.label);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        let x_padding = theme::WIDGET_CONTROL_COMPONENT_PADDING;
        let check_size = theme::BASIC_WIDGET_HEIGHT;

        let label_size = ctx.run_layout(&mut self.label, bc);
        ctx.place_child(&mut self.label, (check_size + x_padding, 0.0).into());

        let desired_size = Size::new(
            check_size + x_padding + label_size.width,
            check_size.max(label_size.height),
        );
        let our_size = bc.constrain(desired_size);
        let baseline =
            ctx.child_baseline_offset(&self.label) + (our_size.height - label_size.height);
        ctx.set_baseline_offset(baseline);
        trace!("Computed layout: size={}, baseline={}", our_size, baseline);
        our_size
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _scene: &mut Scene) {

        // if self.dark_mode {
        //     ctx.mutate(&mut self.button, move |mut button| {
        //         button.set_text("Switch to LIGHT MODE");
        //     });
        // }
        // else {
        //     ctx.mutate(&mut self.button, move |mut button| {
        //         button.set_text("Switch to DARK MODE");
        //     });
        // }
    }

    fn accessibility_role(&self) -> Role {
        Role::CheckBox
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx, node: &mut NodeBuilder) {
        // IMPORTANT: We don't want to merge this code in practice, because
        // the child label already has a 'name' property.
        // This is more of a proof of concept of `get_raw_ref()`.
        if false {
            let button = ctx.get_raw_ref(&self.label);
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
        smallvec![self.label.id()]
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("Checkbox")
    }

    fn get_debug_text(&self) -> Option<String> {
        if self.dark_mode {
            Some("[X]".to_string())
        } else {
            Some("[ ]".to_string())
        }
    }
}
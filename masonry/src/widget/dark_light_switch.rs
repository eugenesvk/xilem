

use accesskit::{DefaultActionVerb, NodeBuilder, Role, Toggled};
use smallvec::{smallvec, SmallVec};
use tracing::{trace, trace_span, Span};
use vello::kurbo::{Insets, Size};
use vello::Scene;
use xilem_colors::tokens::TokenColor;
use xilem_colors::ColorStyle;
use crate::action::Action;
use crate::widget::WidgetMut;
use rand::{thread_rng, Rng};

use crate::{
    theme, AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, PointerButton, PointerEvent, RegisterCtx, StatusChange, TextEvent, Widget, WidgetId, WidgetPod, ArcStr
};
use crate::paint_scene_helpers::{fill_lin_gradient, stroke, UnitPoint};

use super::Label;

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

/// A button that switches between Dark and Light mode.
pub struct DarkLightSwitch {
    label: WidgetPod<Label>,
    dark_mode: bool,
    style: ColorStyle,
    selected: bool,
    has_color_on_select: bool,
}

impl DarkLightSwitch {
    /// Create a new `Switch`.
    pub fn new() -> DarkLightSwitch {
        DarkLightSwitch {
            label: WidgetPod::new(Label::new("Switch to LIGHT mode").with_skip_pointer(true)),
            dark_mode: true,
            style: ColorStyle::default(),
            selected: false,
            has_color_on_select: false,
        }
    }
    pub fn set_style(mut self, new_style: ColorStyle) -> DarkLightSwitch{
        self.style = new_style.clone();
        self
    }
}

// --- MARK: WIDGETMUT ---
impl WidgetMut<'_, DarkLightSwitch> {
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
        let mut label = self.ctx.get_mut(&mut self.widget.label);
        label.set_text(new_text);
    }

    pub fn mutate_style(&mut self, new_style: ColorStyle) {}
}

// --- MARK: IMPL WIDGET ---
impl Widget for DarkLightSwitch {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(_, _) => {
                if !ctx.is_disabled() {
                    ctx.capture_pointer();
                    ctx.request_paint();
                    trace!("Button {:?} pressed", ctx.widget_id());
                }
            }
            PointerEvent::PointerUp(_, _) => {
                if ctx.hovered() && !ctx.is_disabled() {
                    //ctx.submit_action(Action::CheckboxChecked(self.dark_mode));
                    ctx.submit_action(Action::ModeSwitched(PointerButton::None, self.dark_mode));
                    ctx.request_accessibility_update();
                    trace!("light_dark_switch {:?} released", ctx.widget_id());
                }
            }
            _ => (),
        }
    }

    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {}

    fn on_access_event(&mut self, ctx: &mut EventCtx, event: &AccessEvent) {
        if event.target == ctx.widget_id() {
            match event.action {
                accesskit::Action::Default => {
                    //ctx.submit_action(Action::CheckboxChecked(self.dark_mode));
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
        ctx.register_child(&mut self.label);
        
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {

        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();

        let label_size = ctx.run_layout(&mut self.label, &label_bc);

        let baseline = ctx.child_baseline_offset(&self.label);
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = theme::BORDERED_WIDGET_HEIGHT;

        let button_size = bc.constrain(Size::new(
            label_size.width + padding.width,
            (label_size.height + padding.height).max(min_height),
        ));

        let label_offset = (button_size.to_vec2() - label_size.to_vec2()) / 2.0;
        ctx.place_child(&mut self.label, label_offset.to_point());

        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        let tokens = ctx.get_colortokens();
        let is_active = ctx.has_pointer_capture();
        let hovered = ctx.hovered();
        let size = ctx.size();
        let (border_color, stroke_width) = if hovered && !ctx.is_disabled() {
            //(tokens.hovered_ui_element_border, 3.)
            (tokens.set_color(self.style.hov_border), 3.)
        } else {
            (tokens.subtle_borders_and_separators, 1.)
        };
        let token = if is_active {
            TokenColor::AccentText
        }
        else if hovered {
            TokenColor::HighContrastText
        }
        else {
            TokenColor::LowContrastText
        };
        ctx.mutate(&mut self.label, move |mut label| {
            label.set_token(Some(token));
            label.ctx.request_paint();
        });

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(theme::BUTTON_BORDER_RADIUS);

        let grad = if self.selected && self.has_color_on_select && hovered {
            [TokenColor::HoveredSolidBackgrounds; 2]
        }
        else if self.selected && self. has_color_on_select {
            [TokenColor::SolidBackgrounds; 2]
        }
        else if is_active {
            self.style.active_bg_grad
        } else if hovered {
                self.style.hov_bg_grad
        } else {
            self.style.bg_grad
        };

        stroke(scene, &rounded_rect, border_color, stroke_width);
        fill_lin_gradient(
            scene,
            &rounded_rect,
            [tokens.set_color(grad[0]), tokens.set_color(grad[1])],
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
        );
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
        trace_span!("DarkLightSwitch")
    }

    fn get_debug_text(&self) -> Option<String> {
        if self.dark_mode {
            Some("[X]".to_string())
        } else {
            Some("[ ]".to_string())
        }
    }
}
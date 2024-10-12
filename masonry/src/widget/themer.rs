// Copyright 2018 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! A button widget.

use accesskit::{DefaultActionVerb, NodeBuilder, Role};
use smallvec::{smallvec, SmallVec};
use tracing::{trace, trace_span, Span};
use vello::Scene;

use crate::action::Action;
use crate::widget::WidgetMut;

use crate::{
    AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx,
    LifeCycleCtx, PaintCtx, PointerEvent, Size, StatusChange, TextEvent, Widget, WidgetId,
};

pub struct Themer {}

// --- MARK: BUILDERS ---
impl Themer {
    pub fn new() -> Themer {
        Self {}
    }
}

// --- MARK: WIDGETMUT ---
impl WidgetMut<'_, Themer> {
    /// Set the theme_index.
    pub fn set_theme(&mut self, num: usize) {
        self.ctx.switch_theme(num);
    }

}

// --- MARK: IMPL WIDGET ---
impl Widget for Themer {
    fn on_pointer_event(&mut self, ctx: &mut EventCtx, event: &PointerEvent) {
        match event {
            PointerEvent::PointerDown(_, _) => {
                if !ctx.is_disabled() {
                    ctx.capture_pointer();
                    trace!("Button {:?} pressed", ctx.widget_id());
                }
            }
            PointerEvent::PointerUp(button, _) => {
                if ctx.has_pointer_capture() && ctx.hovered() && !ctx.is_disabled() {
                    ctx.submit_action(Action::ButtonPressed(*button));
                    trace!("Button {:?} released", ctx.widget_id());
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
                    //ctx.submit_action(Action::ButtonPressed(PointerButton::Primary));
                    ctx.request_paint();
                }
                _ => {}
            }
        }
    }

    fn on_status_change(&mut self, ctx: &mut LifeCycleCtx, _event: &StatusChange) {
        ctx.request_paint();
    }

    fn register_children(&mut self, _ctx: &mut crate::RegisterCtx) {
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints) -> Size {
        Size::new(0., 0.)
    }

    fn paint(&mut self, _ctx: &mut PaintCtx, _scene: &mut Scene) {
    }

    fn accessibility_role(&self) -> Role {
        Role::Button
    }

    fn accessibility(&mut self, _ctx: &mut AccessCtx, node: &mut NodeBuilder) {
        // IMPORTANT: We don't want to merge this code in practice, because
        // the child label already has a 'name' property.
        // This is more of a proof of concept of `get_raw_ref()`.
        node.set_default_action_verb(DefaultActionVerb::Click);
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![]
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("Button")
    }
}

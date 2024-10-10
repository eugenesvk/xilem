// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use masonry::{widget, PointerButton};
use xilem_core::{Mut, ViewMarker};

use crate::{MessageResult, Pod, View, ViewCtx, ViewId};

// pub fn light_dark_switch<F, State, Action>(
//     callback: impl Fn(&mut State, bool) -> Action + Send + Sync + 'static,
// ) -> LightDarkSwitch<F>
// {
//     LightDarkSwitch {
//         dark_mode: true,
//         callback: F,
//     }
// }

pub fn light_dark_switch<F, State, Action>(
    dark_mode: bool,
    callback: F,
) -> LightDarkSwitch<F>
where
    F: Fn(&mut State, bool) -> Action + Send + 'static,
{
    LightDarkSwitch {
        callback,
        dark_mode,
    }
}

pub struct LightDarkSwitch<F> {
    dark_mode: bool,
    callback: F,
}

// impl<F, State, Action> View<State, Action, ViewCtx> for Checkbox<F>
// where
//     F: Fn(&mut State, bool) -> Action + Send + Sync + 'static,
// {
//     type Element = Pod<widget::Checkbox>;
//     type ViewState = ();

//     fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
//         ctx.with_leaf_action_widget(|ctx| {
//             ctx.new_pod(masonry::widget::Checkbox::new(
//                 self.checked,
//                 self.label.clone(),
//             ))
//         })
//     }

impl<F> ViewMarker for LightDarkSwitch<F> {}
impl<F, State, Action> View<State, Action, ViewCtx> for LightDarkSwitch<F>
where
    F: Fn(&mut State, bool) -> Action + Send + Sync + 'static,
{
    type Element = Pod<widget::LightDarkSwitch>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx) -> (Self::Element, Self::ViewState) {
        ctx.with_leaf_action_widget(|ctx| {
            ctx.new_pod(masonry::widget::LightDarkSwitch::new(
            ))
        })
    }

    fn rebuild<'el>(
        &self,
        prev: &Self,
        _: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: Mut<'el, Self::Element>,
    ) -> Mut<'el, Self::Element> {
        if prev.dark_mode != self.dark_mode {
            element.switch_mode(self.dark_mode);
            ctx.mark_changed();
        }

        element
    }

    fn teardown(
        &self,
        _: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: Mut<'_, Self::Element>,
    ) {
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        _: &mut Self::ViewState,
        id_path: &[ViewId],
        _message: xilem_core::DynMessage,
        _app_state: &mut State,
    ) -> MessageResult<Action> {
        debug_assert!(
            id_path.is_empty(),
            "id path should be empty in Button::message"
        );
        // match message.downcast::<masonry::Action>() {
        //     Ok(action) => {
        //         if let masonry::Action::ButtonPressed(_button) = *action {
        //             MessageResult::Action((self.callback)(app_state))
        //         } else {
        //             tracing::error!("Wrong action type in Button::message: {action:?}");
        //             MessageResult::Stale(action)
        //         }
        //     }
        //     Err(message) => {
        //         tracing::error!("Wrong message type in Button::message: {message:?}");
        //         MessageResult::Stale(message)
        //     }
        // }
        MessageResult::Nop
    }
}
